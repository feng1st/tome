# Tasks: player-attacks | 任务：player-attacks

## 1. Order Components and Pathfinding Overlay | 指令组件与寻路覆盖

- [x] 1.1 Create `src/core/player/components/order.rs`: the `Order`
  enum standing order (`Move { target: CellCoord }` /
  `Attack { target: Entity }`); the doc comment states the
  persist-until-cleared contract (arrival / unreachable / strike /
  interruption) and that inserting a variant replaces whatever stood
  before. verify: `cargo check --all-targets` passes.

  新建 `src/core/player/components/order.rs`：`Order` 枚举站立指令
  （`Move { target: CellCoord }` / `Attack { target: Entity }`）；
  doc comment 写明存续至清除的契约（到格 / 不可达 / 出手 / 被打
  断）与“插入变体即替换旧指令”的规则。验证：
  `cargo check --all-targets` 通过。

- [x] 1.2 Create `src/core/combat/components/attack.rs`
  (`Attack { target }`, the transient action mirroring `Move`), with
  the components `mod.rs` updated. verify:
  `cargo check --all-targets` passes.

  新建 `src/core/combat/components/attack.rs`（`Attack { target }`，
  镜像 `Move` 的瞬态行动），并更新组件 `mod.rs`。验证：
  `cargo check --all-targets` 通过。

- [x] 1.3 `src/core/map/utils/pathfinding.rs`: `find_path` gains an
  obstacle overlay (a set of blocked cells consulted beside terrain;
  the destination cell stays reachable). Tests: a route goes around a
  blocked cell; a blocked destination still resolves a route ending
  beside it; an unreachable target yields `None`. verify: the new unit
  tests pass with `cargo test`.

  `src/core/map/utils/pathfinding.rs`：`find_path` 增加障碍覆盖（地
  形之外再查一组障碍格；目的格本身保持可达）。测试：路线绕开障碍
  格；目的格被占时仍给出止于其旁的路线；彻底不可达返回 `None`。
  验证：新单测 `cargo test` 通过。

## 2. Combat Domain: Hit Skeleton and Strike Execution | combat 域：命中骨架与出手执行

- [x] 2.1 `src/core/combat/utils/attack.rs`: add
  `attack_hits(chance, armor_class, rng)` — the reference skeleton as
  a pure function with the random source injected (percentile first,
  the power roll only for a positive chance), carrying
  `#[allow(dead_code)]` until the strike executor lands. Tests pin
  every branch
  boundary with a seeded source: the certain-hit band, the
  certain-miss band, the non-positive chance, and the
  three-quarters-of-armor edge. verify: the new unit tests pass.

  `src/core/combat/utils/attack.rs`：新增
  `attack_hits(chance, armor_class, rng)`——参照骨架落成注入随机源
  的纯函数（先抽 percentile，chance 为正才抽威力骰），在出手执行器
  落地前暂挂 `#[allow(dead_code)]`。测试用固定种子钉住每条分支边界：
  必中带、必失带、非正 chance、四分之三护甲边线。验证：新单测通
  过。

- [x] 2.2 Create `src/core/combat/systems/act_attack.rs`: consumes
  `Added<Attack>`, reads the attacker's `CombatBonuses` and the
  target kind's `armor_class` (fallible `get` on the target), rolls
  `attack_hits`, and on a hit writes
  `Damage { target, amount: unarmed_damage(bonuses) }`; the action
  component is consumed either way. Register it in
  `CorePhase::PlayerAct` from the combat domain. Tests: a seeded hit
  emits exactly one matching `Damage`; a seeded miss emits none; a
  vanished target is skipped. verify: the new tests pass.

  新建 `src/core/combat/systems/act_attack.rs`：消费
  `Added<Attack>`，读攻击方 `CombatBonuses` 与目标种类的
  `armor_class`（对目标用可失败的 `get`），掷 `attack_hits`，命
  中则写 `Damage { target, amount: unarmed_damage(bonuses) }`；无
  论命中与否行动组件都被消费。在 combat 域注册进
  `CorePhase::PlayerAct`。测试：种子命中恰好发出一条对应
  `Damage`；种子未命中不发；目标消失则跳过。验证：新测试通过。

- [x] 2.3 Remove the three `#[allow(dead_code)]` markers in
  `utils/attack.rs` (melee skill term, attack chance, unarmed damage)
  and the one on `MonsterKind.armor_class` — attack resolution is now
  their consumer; `level`/`blows` and the player-side `armor_class`
  util keep theirs until entry 5. verify: `cargo clippy --all-targets`
  reports zero warnings.

  摘除 `utils/attack.rs` 的三处 `#[allow(dead_code)]`（命中技能
  项、攻击值、徒手伤害）与 `MonsterKind.armor_class` 的一处——攻
  击解析已成为它们的消费者；`level`/`blows` 与玩家侧
  `armor_class` 工具的豁免留到条目 5。验证：
  `cargo clippy --all-targets` 零警告。

- [x] 2.4 The attack chance carries the attack-quality stand-in —
  `ATTACK_QUALITY_STANDIN = 1`, the weapon-side to-hit a level-one
  warrior's weapon contributes (the weaponmastery to-hit at level one
  plus the basic weapon's +0), deleted when the skill and equipment
  systems land. Tests: a level-one warrior's chance is 8, above
  three quarters of a rat's armor; the real assembly (its full
  registers, data, and phase chain) slays a rat across a 30-seed
  sweep. verify: the new tests pass.

  攻击值携带攻击品质顶替值——`ATTACK_QUALITY_STANDIN = 1`，一级战
  士所持武器贡献的命中（武器掌握一级的命中项加基础武器的 +0），
  技能与装备系统落地后删除。测试：一级战士攻击值为 8，高过老鼠
  护甲的四分之三；真装配（全量 register、数据与阶段链）在 30 种子
  扫荡中能杀死老鼠。验证：新测试通过。

## 3. Player Domain: The Executor Becomes Dispatch-Only | player 域：执行器改为只做分派

- [x] 3.1 Rewrite
  `src/core/player/systems/commands/target_cell.rs`: no pathfinding
  — a targeted cell holding a living monster inserts
  `Order::Attack { target }`; a walkable targeted cell inserts
  `Order::Move { target }`; an impassable target or the player's own
  cell is ignored — inserting a variant replaces the standing order.
  Tests: a monster target yields the attack variant; a floor target
  replaces a standing attack variant with the move variant; wall and
  self targets change nothing. verify: the rewritten tests pass.

  重写 `src/core/player/systems/commands/target_cell.rs`：不再寻路
  ——目标格持有活怪物则插入 `Order::Attack { target }`；目标格可走
  则插入 `Order::Move { target }`；目标不可走或落在自身格则忽略
  ——插入变体即替换站立指令。测试：以怪物为目标得攻击变体；以地
  板为目标把存续的攻击变体换成移动变体；以墙或自身为目标不改任何
  状态。验证：重写后的测试通过。

- [x] 3.2 Collapse the frontend input path: delete the cell-gesture
  and resolver layers, the monster paradigm marker, and the input
  sub-phases; the mouse system moves up beside `systems/` and writes
  the core command `TargetCell` directly — the input domain shrinks
  to device translation. verify: `cargo check --all-targets` passes
  and no gesture references remain (`grep`).

  前端输入链路塌缩：删除 cell 手势层、解析器层、怪物范式标记与输
  入子阶段；鼠标系统上提到 `systems/` 旁并直接产生核心命令
  `TargetCell`——输入域收缩为设备翻译。验证：
  `cargo check --all-targets` 通过且无手势残留（grep）。

## 4. Player Domain: Order-Driven Planning | player 域：指令驱动的策划

- [x] 4.1 Rewrite `src/core/player/systems/plan_action.rs`: on a due
  turn, plan from the standing order — attack order: an adjacent
  target (eight directions) inserts `Attack { target }`, spends the
  turn, and clears the order; a distant target paths to its current
  cell (overlay: monster cells blocked, the target's cell exempt) and
  inserts `Move` for the first step, spending the turn; no route or a
  gone target clears the order without spending. Move order: at the
  target clears the order without spending; otherwise path to the
  target (overlay: monster cells blocked, the target exempt) and
  insert `Move` for the first step, spending the turn; no route clears
  the order without spending. No order: nothing planned, nothing
  spent. Tests cover: strike-clears-order, pursuit follows a moved
  target, routing around a third monster mid-pursuit, arrival clears,
  unreachable clears, gone-target clears, and the dead driver still
  freezes the world. verify: the rewritten tests pass.

  重写 `src/core/player/systems/plan_action.rs`：回合到期即按站立指令
  策划——攻击指令：目标相邻（八方向）则插入 `Attack { target }`、
  消耗回合、清除指令；目标不相邻则朝其当前格寻路（覆盖：怪物格为
  障碍，目标格豁免），为首步插入 `Move` 并消耗回合；无路或目标消
  失则清指令不消耗。移动指令：已到目标格则清指令不消耗；否则朝目
  标格寻路（覆盖：怪物格为障碍，目标格豁免），为首步插入 `Move`
  并消耗回合；无路则清指令不消耗。无指令：不策划、不消耗。测试覆
  盖：出手清指令、追击跟随已移动的目标、追击途中绕开第三只怪物、
  到格清除、不可达清除、目标消失清除、死去的驾驶者依旧冻结世界。
  验证：重写后的测试通过。

## 5. Movement Domain: `Path` Deletion | movement 域：`Path` 删除

- [x] 5.1 `act_move` drops its path-popping branch (the `Move` action
  stands alone); delete `src/core/movement/components/path.rs`;
  `sync_animation`'s run signal switches from `path.is_some()` to
  "a standing order present"; the world-clock cadence test
  rewires its `Path` bundle with an `Order`. verify:
  `cargo test` passes and `cargo clippy --all-targets` reports zero
  warnings.

  `act_move` 去掉弹路径分支（`Move` 行动独立成立）；删除
  `src/core/movement/components/path.rs`；`sync_animation` 的跑步信
  号从 `path.is_some()` 换为“站立指令在场”；世界时钟的节奏
  测试把 `Path` 布景换为 `Order` 重接。验证：`cargo test` 通
  过且 `cargo clippy --all-targets` 零警告。

## 6. Health Domain: Damage Interrupts | health 域：伤害打断

- [x] 6.1 `apply_damage`: applying damage also removes the target's
  `Order`. Tests: a damaged driver loses its order; an undamaged one
  keeps it; clearing on a monster (which holds none) is a no-op.
  verify: the new tests pass.

  `apply_damage`：施加伤害时一并移除目标的 `Order`。测试：受伤驾
  驶者失去指令；未受伤者保留；对不持指令的怪物执行清除是空操作。
  验证：新测试通过。

## 7. Monster Domain: Wandering Avoids Occupied Cells | monster 域：随机移动避开占位格

- [x] 7.1 `plan_wander`'s walkable check gains the creature-free
  clause (a cell holding the living player or another monster fails
  the pick). Tests: an occupied neighbor counts as blocked; a monster
  ringed by creatures stands; the living player's cell is never
  stepped onto. verify: the new tests pass.

  `plan_wander` 的可走判定增加无生物条款（持有存活主角或其他怪物的格子
  子让该次选取失败）。测试：被占邻格视同不可走；被生物围住的怪物
  原地不动；怪物永不踏上存活主角所在格。验证：新测试通过。

## 8. End-to-End and Registry | 端到端与登记

- [x] 8.1 A phase-topology world test with the real registers: the
  player clicks a distant rat, pursues turn by turn, strikes when
  adjacent, and the rat leaves the world — with a seeded source. Also
  assert a click on a floor cell beyond a monster routes around it
  and never starts a fight. verify: the test passes.

  用真实 register 接一个阶段拓扑世界测试：玩家点击远处老鼠→逐回
  合追击→相邻出手→老鼠离开世界（固定种子）。另断言：点击怪物后
  方的地板格会绕行且永不卷入战斗。验证：测试通过。

- [x] 8.2 Edit `openspec/specs/monster/spec.md` directly: the Purpose
  line "do not block movement" becomes the blocking wording (delta
  files never carry Purpose). verify: `openspec validate
  player-attacks` still passes.

  直改 `openspec/specs/monster/spec.md`：Purpose 中“不阻碍移动”改
  为阻挡措辞（增量文件不承载 Purpose）。验证：
  `openspec validate player-attacks` 仍通过。

- [x] 8.3 Delete OPEN_ISSUES entry 4 and repair the references
  (entries 5, 6 and the ordering line). verify: the registry reads
  coherently with entries 5 and 6 as the remaining loop items.

  删除 OPEN_ISSUES 条目 4 并修顺引用（条目 5、6 与顺序理由行）。
  验证：登记表读来通顺，闭环剩余条目为 5、6。

- [x] 8.4 Full gates: `cargo +nightly fmt --check`, `cargo clippy
  --all-targets` (zero warnings), `cargo check --all-targets`,
  `cargo test`, `openspec validate player-attacks`. verify: all
  green.

  全门：`cargo +nightly fmt --check`、`cargo clippy --all-targets`
  （零警告）、`cargo check --all-targets`、`cargo test`、
  `openspec validate player-attacks`。验证：全绿。
