# Design: monster-attacks | 设计：monster-attacks

## Context

Landed pieces this design composes (see proposal.md — Why for the
motivation): the `Attack { target }` action component and the shared
hit skeleton `attack_hits` (combat domain); the `Damage` channel with
`apply_damage` dual-registered in both Resolve phases (health);
`act_move` single-registered into both `PlayerAct` and `WorldAct`
(movement — the template for a two-sided action); the monster
vocabulary carrying `level`/`blows` under dead-code allowances whose
comments await this change as their consumer; `plan_wander` planning
in `WorldPlan`. Stand-ins follow the house convention: a constant in
its consuming file, a comment mapping it to the reference source, and
a recovery note in OPEN_ISSUES.

本设计组合的既有件（动机见 proposal.md — Why）：`Attack { target }`
动作组件与共享命中骨架 `attack_hits`（combat 域）；`Damage` 通道与
双 Resolve 阶段注册的 `apply_damage`（health）；单系统注册进
`PlayerAct` 与 `WorldAct` 的 `act_move`（movement——双侧行动的模
板）；怪物词表携带 `level`/`blows`，其 dead_code 豁免注释等候本条
变更作为消费者；`plan_wander` 在 `WorldPlan` 策划。顶替值遵循既有
体例：常量住在消费文件、注释记映射、OPEN_ISSUES 记回收。

## Goals / Non-Goals

**Goals:**

- One strike pipeline: the system reads only the blows and
  armor-class components and never asks what an entity is;
  player→monster, monster→player, and monster→monster resolve
  identically.
- The monster's strike closes the minimal loop: adjacency triggers it,
  the turn is spent, and damage flows through the existing channel.
- Every player-monster difference lives at a producer (derive or
  spawn), never at resolution.

**Non-Goals:**

- Awareness, noticing, fleeing, and chase AI (OPEN_ISSUES entry 11) —
  the strike branch fires on adjacency alone, and no monster plans a
  strike at another monster yet (the pipeline is ready, no planner is).
- Effect attachment, armor damage reduction, and per-blow effect
  powers from the effect table (entry 10); the player's multi-blow
  count and weapon dice (entry 9); dodge and the exemption chain
  (entry 8).
- The presentation of strikes (entry 6).

**目标：**

- 单一出手管线：系统只读打击与护甲组件，从不问实体是谁；玩家→
  怪、怪→玩家、怪→怪同款解析。
- 怪物出手合拢最小闭环：相邻即触发、回合照价消耗、伤害走既有通
  道。
- 玩家与怪物的每一处差异都住在生产侧（派生或出生），不住解析侧。

**不做：**

- 感知、发觉、逃跑与追击 AI（条目 11）——攻击分支只看相邻；怪物
  之间也还无人策划互殴（管线就绪、策划不做）。
- effect 附加、护甲折减与威力表的逐击威力（条目 10）；玩家多击数
  与武器骰（条目 9）；闪避与豁免链（条目 8）。
- 出手的呈现（条目 6）。

## Decisions

### D1: A due turn with an adjacent living player plans a strike — deterministic, top branch | 到期回合遇相邻活主角必出手——确定式、最高优先分支

The monster planner checks the eight neighboring cells for the living
player before anything else; on a hit it plans `Attack { target:
player }` and spends the turn, and the wander never runs. This
deliberately deviates from tome2, where the strike is a movement
accident — the movement AI's chosen cell happens to be the player's
(melee2.c:7138-7152), which under RAND_25 makes bites rare. Pixel
Dungeon's shape is the deliberate one (Hunting + `canAttack` =
8-adjacency, deterministic), and our wander already excludes occupied
cells, so the tome2 trigger could never fire here. The reference's
notice delay (a wandering mob spends a turn noticing before it hunts)
belongs to the awareness layer — entry 11.

怪物策划先于一切检查八邻格内的存活主角：命中即策划
`Attack { target: 主角 }` 并消耗回合，游荡不再运行。这有意偏离
tome2——那里出手是移动的事故：移动 AI 选中的格子恰是主角格
（melee2.c:7138-7152），在 RAND_25 下咬伤稀少。Pixel Dungeon 的
形状才是确定式（Hunting + `canAttack` = 8 邻接），且我们的游荡已
把占位格排除，tome2 的触发路径在此永远不会发生。参照的"发觉延
迟"（游荡怪先花一个回合发觉再追击）归感知层——条目 11。

**Alternatives rejected | 否决的备选**: the tome2-faithful
move-pick conversion — conflicts with the landed occupancy blocking,
and the bite rate collapses below any threat.

否决的备选：tome2 原样的"选格变质"——与已落地的占位避让冲突，
且咬伤率塌到没有威胁。

### D2: One pipeline over the blows and armor-class components; the side difference moves to producers | 打击与护甲组件上的单一管线；两侧差异移到生产侧

The attacker carries `Blows` — a list of `Blow { chance, damage }` —
and the target carries `ArmorClass`. One `act_attack` system,
registered in both `PlayerAct` and `WorldAct` (the `act_move`
template), resolves every `Attack`: per blow, judge the hit against
the target's armor and write one `Damage` request on a hit. The
system reads no registry and no side-specific component. The player's
`Blows`/`ArmorClass` derive from the six statistics in the Derive
phase (the existing formulas — melee skill to-hit, attack quality,
unarmed damage, armor class — stay in `combat/utils/attack.rs` and
`armor_class.rs`; only their call site moves, from the act system to
the derive). The monster's translate from the vocabulary row at
spawn. A pairing is never materialized: chance comes from the
attacker's component, armor from the target's, and the pair exists
only as one `attack_hits` call's arguments.

攻击方携带 `Blows`——`Blow { chance, damage }` 的列表；目标携带
`ArmorClass`。单一 `act_attack` 系统注册进 `PlayerAct` 与
`WorldAct` 两个阶段（`act_move` 模板），解析每一次 `Attack`：逐
击对目标护甲判定命中、命中即写一条 `Damage` 请求。系统不读词表、
不读任何一侧专属组件。玩家的 `Blows`/`ArmorClass` 在 Derive 阶段
由六维派生（既有公式——近战技能命中值、攻击品质、徒手伤害、护甲—
—原地留在 `combat/utils/attack.rs` 与 `armor_class.rs`，只有调用
点从执行系统移到派生）；怪物的在出生时从词表行翻译。配对从不落
成数据：chance 取自攻击方组件、护甲取自目标组件，配对只作为一次
`attack_hits` 调用的参数存在。

**Alternatives rejected | 否决的备选**:

- Two side-specific act systems — the target side would still branch
  the day monster-on-monster arrives; the branch is deferred, not
  removed.
- One system reading `MonsterRegistry`/`CombatBonuses` by kind —
  resolution-time table lookups break the house grain (a creature's
  current values live on the creature: `Speed`, `HitPoints`), and the
  registry never leaves the strike path.
- A monster-specific `CombatProfile` component — superseded: the two
  components already say everything the strike needs, and
  the kind's `level` folds into each blow's chance at spawn. The
  level stays reachable through the `MonsterIndex` handle for its
  future consumers (fear thresholds, experience) without a component.

否决的备选：

- 双侧各建执行系统——怪物互殴到来的那天目标侧仍要分支，分支只
  是被推迟而非消失。
- 单系统按种类读 `MonsterRegistry`/`CombatBonuses`——解析时查表
  违背本库纹理（生物的当前值住在生物身上：`Speed`、
  `HitPoints`），词表也永远退不出出手路径。
- 怪物专属 `CombatProfile` 组件——被取代：两个组件已说尽出手所
  需，词表 level 在出生时折进每击的 chance；未来的消费者
  （恐惧阈值、经验）经 `MonsterIndex` 句柄回词表取 level，无需
  组件。

### D3: A mechanic is an optional component; absence switches it off | 机制即可选组件，缺席即关闭

- `ArmorClass` absent → three quarters of armor reads as zero: armor
  takes no part in the hit check (the certain bands still apply).
- `Blows` absent or empty → the strike loop iterates nothing: the
  creature cannot hit. The reference's `RF1_NEVER_BLOW` kinds express
  as an empty `blows: []` vocabulary entry — no flag.
- Per-blow `chance` storage is the shape entry 10 needs (the effect
  table's power is per blow); the list shape is what entry 9 needs
  (the player's blow count grows the list). Neither entry changes the
  component shapes.

- `ArmorClass` 缺席 → 护甲的四分之三按零读：护甲不参与命中判定
  （必中/必失带照常）。
- `Blows` 缺席或为空 → 出手循环空转：该生物打不中。参照的
  `RF1_NEVER_BLOW` 种类表达为词表 `blows: []`——不用旗标。
- 逐击存 `chance` 正是条目 10 需要的形状（威力表的威力逐击）；
  列表形状正是条目 9 需要的（玩家多击数把列表变长）。两个条目都
  不改组件形状。

### D4: The monster power stand-in is 60 | 怪物威力顶替值取 60

`60 + 3 × level` per blow, mapping to the reference's effect power
table entry `RBE_HURT = 60` (melee1.c:1416-1565) — the plain-damage
family, which is what an effect-less blow behaves as. The rat's
literal effect power (POISON = 5, from its `BITE:POISON:1d3` r_info
row) is reference-only: nothing here attaches poison, so the blow is
honestly HURT-shaped. The constant lives at the spawn translation
with the mapping comment; entry 10 deletes it when blows gain effect
columns. The choice is balance-neutral at birth: a level-one warrior's
armor class is 0-1, so the threshold is zero and any positive chance
hits past the bands — 60 and 5 both land about 95% of strikes.

每击 `60 + 3 × level`，映射参照威力表的 `RBE_HURT = 60`
（melee1.c:1416-1565）——纯伤害家族，无效果附加的一击行为上正
是它。老鼠字面的威力（POISON = 5，出自其 `BITE:POISON:1d3` 词表
行）仅作参考：此处不附加中毒，这一击诚实地说就是 HURT 形状。常
量住在出生翻译处并注释映射；条目 10 在 blows 长出 effect 列时删
除。该选择在出生局面对平衡中性：一级战士护甲 0-1，阈值按零计，
任意正 chance 过带即中——60 与 5 都约 95% 命中。

### D5: plan_wander renames to plan_action | plan_wander 改名 plan_action

The system's job is now "plan the monster's turn": strike branch
first, wander as the fallback. The name follows the job (and mirrors
the player side's `plan_action`); the wander spec requirement keeps
its name — it specifies the wandering behavior, which is unchanged in
kind.

系统的职责已是"策划怪物的回合"：攻击分支在先、游荡回落。名字跟
随职责（并与玩家侧 `plan_action` 对仗）；游荡的 spec 条款名不动
——它规定的是游荡行为本身，行为的种类没有变。

### D6: The derive chain must write every pass | 派生链每趟必须真实写入

The player's blows and armor class derive in two stages: `Changed<Stats>`
rewrites `CombatBonuses`, and `Changed<CombatBonuses>` rewrites
`Blows`/`ArmorClass`. The chain breaks silently if a stage skips
"unchanged" writes (`Changed` never fires downstream), so every stage
writes unconditionally; a test edits a stat and asserts the blow
values follow. Alternative considered: one pass computing everything
from `Stats` — rejected, because the bonuses are a spec'd layer of
their own (combat-bonuses) and two cheap passes beat one duplicated
one.

玩家的打击与护甲分两阶段派生：`Changed<Stats>` 重写
`CombatBonuses`，`Changed<CombatBonuses>` 重写
`Blows`/`ArmorClass`。若某一阶段跳过"值没变"的写入（`Changed`
不再向下游触发），链会无声断裂，故每一阶段无条件写入；配一个修
改属性、断言打击数值跟随的测试。考虑过备选：一趟从 `Stats` 直出
全部——否决，因为加成自身是一条已立 spec 的层（combat-
bonuses），两趟廉价派生好过一趟重复派生。

## Risks / Trade-offs

- Lethality: at the stand-in values the rat lands about 95% of bites
  for 1d3 each and kills an idle level-one warrior in roughly ten rat
  turns, while the player kills the rat in two to four player turns —
  one on one, the player finishes first. The rat is no faster than
  the player (both speed 110), and it does not pursue before entry
  11: stepping out of adjacency ends the threat, and only the turns
  spent adjacent are dangerous. Accepted as the minimal loop's
  tension. → The stand-in is one constant; real balance arrives with
  entries 8/9/10.

  杀伤：顶替值下老鼠约 95% 命中、每口 1d3，主角站着不动约十个鼠
  回合致死；主角反手二至四回合杀鼠——一对一，主角先打死对方。
  老鼠与主角同速（110），条目 11 之前也没有追击：脱离相邻格威胁
  即止，危险只在贴身的那些回合。作为最小闭环的张力接受。→ 顶替
  值是一个常量；真正的平衡随条目 8/9/10 到来。

- A bite clears the player's standing order (the landed interruption
  rule applies to every creature, the player included) — being bitten
  stops the player mid-move, Pixel Dungeon's `interrupt()` in kind.
  Accepted; no new mechanism.

  咬伤会清掉主角的站立指令（已落地的打断条款对任何生物生效，主
  角不例外）——被咬即停步，与 Pixel Dungeon 的 `interrupt()`
  同款。接受；不加新机制。

- An empty-blows monster would spend its turn striking nothing; the
  reference's NEVER_BLOW never attacks at all. No such kind exists
  today. → Noted here; if one arrives, the planner learns to skip
  creatures without blows.

  空 blows 的怪物会空耗回合出手空转；参照的 NEVER_BLOW 压根不出
  手。今天没有这样的种类。→ 记在此；若有，策划学会跳过无打击
  者。

## Migration Plan

No persisted state exists; nothing to migrate.

无存档状态，无迁移。
