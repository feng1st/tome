# Proposal: character-attributes

## Why

主角与怪物目前只有身份与速度，没有任何属性数值：没有生命、没有六维、没有攻防，战斗、状态、掉落等一切机制都无依附。词表要对齐 tome2 的种族/职业修正与怪物 kind 束，得先把属性体系立起来；这同时也是 OPEN_ISSUES #2 登记的 race 词表终局形态的裁决时点。

## What Changes

- 新增 `stats` 域（玩家向六维）：`Stats { max, current }` 组件、`Stat` 枚举（Strength/Intelligence/Wisdom/Dexterity/Constitution/Charisma）。
- 出生掷点照搬 tome2：每组六维 `5 + d3 + d4 + d5`，总和须落在 42–57；race/class 的 `stat_modifiers` 按 18 以上分段非线性并入（机制对应 tome2 `adjust_stat`）。
- 新增 `health` 域（全生物共享）：`HitPoints { current, max }`。玩家 `max = hit_die + constitution_hp_bonus(CON) / 2`（`constitution_hp_bonus` 为 38 项查表，机制对应 tome2 `adj_con_mhp`；固定等级 1，出生期一次算好；逐级数组与运行时重算押后）。怪物将来由 `hit_die` 喂 `max`，不经六维。
- race/class 独立成域（`core/race/`、`core/class/`）：各增 `stat_modifiers`、`hit_die`；两张注册表升级为保留条目（`RaceKind`/`ClassKind`）。`core/creature` 解散。
- 身份拆分：race 收缩为"可被扮演的种族"；怪物以 kind（MonsterIndex）为身份，条目去掉 race/class/unique_id，减为 kind + speed。
- unique 整套移除：`UniqueRegistry`、`UniqueIndex`、出生聚合与 creature-identity 的 unique 条款（现在没人用）。
- figure 绑定键加 `monster` 形态，优先级 monsterId > race+class > race；去掉 unique 键。
- 出生流程：race/class 仍是代码常量，新增出生步骤掷点写组件；不做角色创建界面。

## Capabilities

### New Capabilities

- `stats`: 玩家向六维属性的声明、出生掷点与种族/职业修正。
- `health`: 全生物共享的生命值组件；玩家生命值由体质派生。
- `race`: race 词表（可被扮演的种族，携带六维修正与生命骰）。
- `class`: class 词表（职业，携带六维修正与生命骰）。

### Modified Capabilities

- `creature-identity`: 收缩为身份模型与形象绑定——词表注册表与校验拆分为 `race` 与 `class`，绑定键加 `monster` 形态、去 unique 键，优先级 monsterId > race+class > race；unique 整套移除。
- `monster`: 条目去掉 race/class/unique_id，以 kind 承担身份与绑定。

## Impact

- 新增 `core/stats/`、`core/health/`、`core/race/`、`core/class/`；`core/creature` 解散；移植体质生命加成表（38 项，对应 tome2 `adj_con_mhp`）。
- `core/monster`：`MonsterKind` 去 race/class，条目解析、出生、绑定调整。
- `frontend/display/creature`：绑定键与 attach（改由 `Added<MonsterIndex>` 触发）。
- 数据：`races.ron`/`classes.ron` 加字段、`monsters.ron` 减字段、`creature_figures.ron` 换键。
- 沿用 `rand`；文档：OPEN_ISSUES #2 裁决登记。
