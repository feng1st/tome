# Proposal: player-attacks | 变更提案：player-attacks

## Why

The fourth entry of the minimal combat loop (OPEN_ISSUES entries 1-6):
bumping into a monster means attacking it, and the player can kill a
rat. Every ingredient is already landed — the monster combat profile,
the player's combat bonuses, and the damage channel with death
handling. What does not exist is the trigger: nothing turns a click
on a monster into an attack. The click-to-move model also owes its
answer to what monsters mean for movement: today the player walks
straight through them.

最小战斗闭环（OPEN_ISSUES 条目 1-6）的第四条：撞向怪物即攻击，玩家
能打死老鼠。素材已全部落地——怪物战斗档案、玩家战斗加成、伤害通
道与死亡处理。缺的是扳机：没有任何东西把“点击怪物”变成一次攻
击。点击移动模型也欠一个回答——怪物对移动意味着什么：今天主角
径直穿怪而过。

## What Changes

- Standing orders replace the precomputed path (**BREAKING** for the
  movement model): the `Path` component deletes. The command executor
  only dispatches — a click on a monster-occupied cell issues an
  attack order, a click on a walkable cell issues a move order, a new
  click replaces the standing order, and an unwalkable target is
  ignored as today. Planning no longer consumes a frozen queue; each
  due turn plans one step by pathing against the current world.

  站立指令取代预计算路径（对移动模型为 **BREAKING**）：`Path` 组件
  删除。命令执行器只做分派——点到怪物格发出攻击指令，点到可走格
  发出移动指令，新点击替换旧指令，不可走目标照旧忽略。策划不再消
  费冻结的队列；每个到期回合对当前世界重新寻路、计划一步。

- Attack order (pursuit): while adjacent, the strike plans, spends
  the turn, and clears the order — one strike per order, repeated
  strikes come from repeated clicks (the reference behavior). While
  not adjacent, plan one step toward the target's current cell;
  monster cells are obstacles with the target's own cell exempt. An
  unreachable target clears the order.

  攻击指令（追击）：相邻时出手、消耗回合、清除指令——一道指令只
  出手一次，连打靠连点（参照实现的行为）。不相邻时朝目标当前格寻
  路走一步；怪物格是障碍，目标格本身豁免。目标不可达时清除指令。

- Move order: each due turn plans one step toward the target with
  monster cells as obstacles (the target itself exempt — walking up
  beside a monster standing on the target clears the order). Arrival
  or unreachable terrain clears the order. With no standing order the
  turn is not spent and the world freezes awaiting input, as today.

  移动指令：每个到期回合朝目标寻路走一步，怪物格为障碍（目标格本
  身豁免——走到站在目标格上的怪物旁边即清除指令）。到格或地形
  不可达时清除指令。没有站立指令时回合不消耗、世界冻结等待输入，
  与今天相同。

- Attack resolution in the combat domain: an `Attack { target }`
  action planned from the attack order and executed in `PlayerAct`.
  The hit skeleton follows the tome2 reference — the percentile's
  certain-hit/certain-miss band, a non-positive chance never hitting,
  and the power roll against three quarters of the target's armor.
  Damage is the landed unarmed formula (1 + damage bonus, floored at
  zero), emitted through the existing `Damage` channel. Attack and
  movement share the same base action duration.

  combat 域的攻击解析：由攻击指令策划出 `Attack { target }` 行动，
  在 `PlayerAct` 执行。命中骨架照搬 tome2 参照——percentile 的必
  中/必失带、非正 chance 必失、威力骰对四分之三护甲。伤害用已落
  地的空手公式（1 + 伤害加成，下端夹零），经既有 `Damage` 通道发
  出。出手与移动同基准行动时长。

- Damage interrupts: applying damage clears the target's standing
  orders — the reference's "being hit interrupts" world rule, landed
  at the damage channel's application point so the monster-side attacks
  of entry 5 inherit it with no extra work.

  受击打断：伤害落地即清除目标的站立指令——参照实现“被打中即打
  断”的世界规则，落在伤害通道的施加点，条目 5 的怪物攻击到时零
  成本继承。

- Monster occupancy coherence: wandering monsters pick only cells
  free of living creatures — no stacking, no stepping onto a living
  player's cell, which is also the precondition entry 5's adjacency
  check needs.

  怪物占位自洽：随机移动只选无活物的格子——不叠格、不走上存活主
  角所在格，这也是条目 5 相邻判定需要的前置。

- Out of this entry (see OPEN_ISSUES): monsters fight back (entry 5);
  every presentation concern (entry 6); weapon dice, criticals and
  multiple blows (entries 8, 9); the reference's new-enemy-sighting
  interruption (no field-of-view system exists) and its
  lastAction/resume convenience.

  不入本条（见 OPEN_ISSUES）：怪物还手（条目 5）；一切表现（条目
  6）；武器骰、暴击与多击（条目 8、9）；参照实现的新敌入视野打
  断（视野系统不存在）与其 lastAction/resume 便利机制。

## Capabilities

### New Capabilities

(none | 无)

### Modified Capabilities

- `combat`: adds the player attack requirements — the attack order
  with its pursuit and clear semantics, the hit skeleton, and unarmed
  damage through the damage channel; the attack chance gains the
  weapon-side stand-in it carries until the skill and equipment
  systems land.

  `combat`：新增玩家攻击 requirement——攻击指令及其追击与清除语
  义、命中骨架、经伤害通道的空手伤害；攻击值新增武器侧顶替值，
  随技能与装备系统落地删除。

- `player-movement`: the movement model changes from a precomputed
  path to a standing order with per-turn re-pathing; monster cells
  are routed around.

  `player-movement`：移动模型从预计算路径改为站立指令加每回合寻
  路；怪物格绕行。

- `monster`: monsters block the player — the walk-through clause
  inverts — and wandering avoids occupied cells.

  `monster`：怪物阻挡主角——穿行条款反转——且随机移动避开占位
  格。

- `health`: applying damage clears the target's standing order.

  `health`：伤害落地清除目标的站立指令。

- `world-clock`: records the reference's energy-accumulation model as
  superseded by the next-turn slot (the original code quoted for
  search), and rewords the freeze clause from "queued path" to the
  standing order — no behavior change.

  `world-clock`：把参照的能量累积模型记录为已被回合槽机制取代（附
  原文供检索），并将冻结条款的“排队路径”改写为站立指令——无行
  为变化。

## Impact

- `src/core/movement/`: `components/path.rs` deletes;
  `act_move` drops its path-popping branch.

  `src/core/movement/`：`components/path.rs` 删除；`act_move` 去
  掉弹路径分支。

- `src/core/player/`: the `TargetCell` executor becomes
  dispatch-only (no pathfinding; gains the monster-occupancy lookup);
  the planner rewrites into per-turn planning over the standing
  order, and the order itself (`Order` — the move/attack variant enum)
  lives here beside them.

  `src/core/player/`：`TargetCell` 执行器改为只做分派（不再寻路，
  增加怪物占位查询）；策划重写为对站立指令的每回合策划，指令组件
  （`Order`——移动/攻击变体枚举）也落在这里与它们作伴。

- `src/core/combat/`: gains the `Attack` action component and the
  `act_attack` system; the hit skeleton joins `utils/attack.rs` and its
  three `#[allow(dead_code)]` markers (the melee skill term, the
  attack chance, unarmed damage) drop as attack resolution becomes
  their consumer.

  `src/core/combat/`：新增 `Attack` 行动组件与 `act_attack` 系统；
  命中骨架进入 `utils/attack.rs`，其中三处
  `#[allow(dead_code)]`（命中技能项、攻击机会、空手伤害）随攻击
  解析成为消费者而摘除。

- `src/core/map/utils/pathfinding.rs`: gains an obstacle overlay so
  planning can route around monster cells.

  `src/core/map/utils/pathfinding.rs`：增加障碍覆盖参数，供策划绕
  开怪物格。

- `src/core/health/systems/apply_damage.rs`: applying damage also
  strips the target's standing order.

  `src/core/health/systems/apply_damage.rs`：伤害落地时一并清除目
  标的站立指令组件。

- `src/core/monster/systems/plan_wander.rs`: the walkable check gains
  a creature-free clause; `MonsterKind.armor_class`'s
  `#[allow(dead_code)]` drops (level and blows keep theirs until
  entry 5).

  `src/core/monster/systems/plan_wander.rs`：可走判定增加无生物条
  款；`MonsterKind.armor_class` 的 `#[allow(dead_code)]` 摘除
  （level 与 blows 的保留到条目 5）。

- `src/frontend/display/sprite_animation/systems/sync_animation.rs`:
  the run-animation signal switches from `Path` to the standing
  orders.

  `src/frontend/display/sprite_animation/systems/sync_animation.rs`：
  跑步动画信号从 `Path` 换绑到站立指令。

- `src/frontend/input/`: collapses to mouse translation — the cell
  gesture and resolver layers and the monster paradigm marker are
  deleted, and a press becomes the core command `TargetCell`
  directly.

  `src/frontend/input/`：塌缩为鼠标翻译——cell 手势层、解析器层与
  怪物范式标记删除，按下即产生核心命令 `TargetCell`。

- Specs: delta files for `combat`, `player-movement`, `monster`,
  `health`, and `world-clock`. No data-file or dependency changes.

  spec 侧：`combat`、`player-movement`、`monster`、`health`、
  `world-clock` 五个增量文件。数据文件与依赖不变。
