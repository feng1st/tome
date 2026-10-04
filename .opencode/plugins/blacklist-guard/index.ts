// Blacklist guard: two jobs.
// 1. Every model request gets the current language blacklist injected into
//    its system prompt (~/.agents/language-blacklist.txt, gitignore-style:
//    bare word = block, !word = allow, # = comment; a trailing （…）or (…)
//    note on an entry line is stripped).
// 2. A write/edit whose *replacement* content hits a block word not covered
//    by an allow word is rejected before the tool runs. An edit's source
//    (oldString) may contain blacklist words — only the replacement must
//    not. Fail-open by design: a guard error never blocks work; only a
//    real hit throws.
//
// Exported as a plain { id, setup } object: the packaged server cannot
// resolve the bare "@opencode/plugin" specifier from a project-local
// plugin, and the V2 runtime only reads `id` and `setup`.

import { readFileSync, writeFileSync } from "node:fs"
import { homedir } from "node:os"
import { join } from "node:path"

const LIST_PATH = join(homedir(), ".agents", "language-blacklist.txt")
// Tools whose input carries file content. bash/grep are deliberately not
// guarded: blacklist self-checks need to pass the banned words as arguments.
const WRITE_TOOLS = new Set(["write", "edit", "multiedit", "patch", "apply_patch"])

const REPLACEMENT_KEYS = ["content", "newString", "new_string", "newText", "replacement", "text"]
// Fields that carry the replaced-away source: never scanned.
const SOURCE_KEYS = ["oldString", "old_string", "oldText", "search", "find", "source", "from"]

class BlacklistHit extends Error {}

function isExempt(path: string, content: string): boolean {
  const normalized = path.replaceAll("\\", "/")
  return (
    // The list itself is exempt in both directions: the guard only ever
    // intercepts file-writing tools, so reads are untouched, and writes to
    // the list must pass so it can be maintained. The config that
    // references it, frozen history, and text about the list are exempt
    // for the same reason.
    normalized.endsWith(".agents/language-blacklist.txt") ||
    normalized.endsWith("openspec/config.yaml") ||
    normalized.includes("openspec/changes/archive/") ||
    content.includes("语言黑名单")
  )
}

function loadRules(file: string): { block: string[]; allow: string[] } {
  const text = readFileSync(file, "utf8")
  const block: string[] = []
  const allow: string[] = []
  const firstWord = (line: string) => line.match(/^([^\s（(]+)/)?.[1]
  for (const raw of text.split("\n")) {
    const line = raw.trim()
    if (!line || line.startsWith("#")) continue
    if (line.startsWith("!")) {
      const word = firstWord(line.slice(1).trim())
      if (word) allow.push(word)
    } else {
      const word = firstWord(line)
      if (word) block.push(word)
    }
  }
  return { block, allow }
}

// Whether the [start, end) occurrence is fully covered by an allow phrase.
function covered(content: string, allow: string[], start: number, end: number): boolean {
  for (const phrase of allow) {
    let at = content.indexOf(phrase)
    while (at !== -1) {
      if (at <= start && end <= at + phrase.length) return true
      at = content.indexOf(phrase, at + 1)
    }
  }
  return false
}

function findHit(content: string, rules: { block: string[]; allow: string[] }): string | null {
  for (const word of rules.block) {
    let at = content.indexOf(word)
    while (at !== -1) {
      if (!covered(content, rules.allow, at, at + word.length)) return word
      at = content.indexOf(word, at + 1)
    }
  }
  return null
}

function firstString(input: Record<string, unknown>, keys: string[]): string | undefined {
  for (const key of keys) {
    const value = input[key]
    if (typeof value === "string") return value
  }
  return undefined
}

// Strings under unknown keys, excluding known source fields.
function fallbackText(input: Record<string, unknown>): string {
  const parts: string[] = []
  const walk = (value: unknown, key?: string) => {
    if (typeof value === "string") {
      if (key && SOURCE_KEYS.includes(key)) return
      parts.push(value)
    } else if (Array.isArray(value)) {
      for (const item of value) walk(item)
    } else if (value && typeof value === "object") {
      for (const [childKey, child] of Object.entries(value)) walk(child, childKey)
    }
  }
  walk(input)
  return parts.join("\n")
}

// The text a tool would actually write: an edit contributes its replacement
// only; an edit's source is allowed to contain blacklist words.
function writtenText(tool: string, input: Record<string, unknown>): string {
  if (tool === "write") {
    return firstString(input, REPLACEMENT_KEYS) ?? fallbackText(input)
  }
  if (tool === "edit") {
    return firstString(input, REPLACEMENT_KEYS) ?? fallbackText(input)
  }
  if (tool === "multiedit") {
    const edits = input.edits
    if (Array.isArray(edits)) {
      const texts = edits
        .map((item) => firstString((item ?? {}) as Record<string, unknown>, REPLACEMENT_KEYS))
        .filter((text): text is string => typeof text === "string")
      if (texts.length > 0) return texts.join("\n")
    }
  }
  return fallbackText(input)
}

const plugin = {
  id: "tome.blacklist-guard",
  async setup(ctx: any) {
    let contextLogged = false
    await ctx.session.hook("context", (event: any) => {
      const stamp = (note: string) => {
        try {
          writeFileSync(
            join(homedir(), ".agents", ".last-injection"),
            `${new Date().toISOString()} ${note}`,
          )
        } catch {
          // Diagnostics only.
        }
      }
      try {
        stamp("enter")
        const list = readFileSync(LIST_PATH, "utf8").trim()
        if (!list) {
          stamp("empty list")
          return
        }
        event.system.push({
          type: "text",
          text:
            "【语言黑名单（每次请求自动注入）】可见回复、写入的文件与注释均不得包含下列词；" +
            "放行规则见 ~/.agents/language-blacklist.txt。\n\n" +
            list,
        })
        stamp("injected")
        if (!contextLogged) {
          contextLogged = true
          console.log("[blacklist-guard] context injection active")
        }
      } catch (error) {
        stamp(`error: ${String(error)}`)
        console.error("[blacklist-guard]", error)
      }
    })
    await ctx.tool.hook("execute.before", (event: any) => {
      if (!WRITE_TOOLS.has(event.tool)) return
      try {
        const input = event.input as Record<string, unknown>
        const path =
          typeof input.filePath === "string"
            ? input.filePath
            : typeof input.path === "string"
              ? input.path
              : ""
        const content = writtenText(event.tool, input)
        if (isExempt(path, content)) return
        const hit = findHit(content, loadRules(LIST_PATH))
        if (hit) {
          throw new BlacklistHit(
            `黑名单拦截：「${hit}」不得出现在将写入 ${path} 的内容里。完整词表见 ~/.agents/language-blacklist.txt，读取后改写重试。`,
          )
        }
      } catch (error) {
        if (error instanceof BlacklistHit) throw error
        console.error("[blacklist-guard]", error)
      }
    })
    console.log("[blacklist-guard] loaded")
  },
}

export default plugin
