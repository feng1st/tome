# Tasks: damage-and-death | 任务：damage-and-death

## 1. Health Domain: The Channel and the Marker | health 域：通道与标记

- [x] 1.1 Create `src/core/health/messages/damage.rs` (and
  `messages/mod.rs`): the `Damage { target: Entity, amount: i32 }`
  message — the only entry through which creatures are hurt; the doc
  comment states the `amount >= 0` contract and the destructive-drain
  consumption. verify: `cargo check --all-targets` passes.

  新建 `src/core/health/messages/damage.rs`（及 `messages/mod.rs`）：
  `Damage { target: Entity, amount: i32 }` 消息——伤害生物的唯一入
  口；doc comment 写明 `amount >= 0` 契约与破坏式排干的消费方式。
  验证：`cargo check --all-targets` 通过。

- [x] 1.2 Create `src/core/health/components/dead.rs`: the `Dead`
  marker component for the slain driver. verify:
  `cargo check --all-targets` passes.

  新建 `src/core/health/components/dead.rs`：死去驾驶者的 `Dead`
  标记组件。验证：`cargo check --all-targets` 通过。

## 2. Health Domain: The Settle System | health 域：settle 系统

- [x] 2.1 Create `src/core/health/systems/settle_damage.rs` (and
  `systems/mod.rs`): drain `Messages<Damage>` destructively
  (`ResMut<Messages<Damage>>` + `drain()`, with the comment explaining
  why two instances never double-apply), subtract each amount without
  clamping at zero (`debug_assert!(amount >= 0)`), then sweep every
  creature for `current < 0` — monsters despawn, the driver gains
  `Dead`; entities that are neither stay untouched. verify:
  `cargo test` passes the world tests — reduction (12→7), unclamped
  (3→-2), the ceiling invariant holds, a creature at exactly zero is
  alive (monster stays, player unmarked), a monster at -1 is despawned,
  the player at -1 carries `Dead`.

  新建 `src/core/health/systems/settle_damage.rs`（及
  `systems/mod.rs`）：破坏式排干 `Messages<Damage>`
  （`ResMut<Messages<Damage>>` + `drain()`，注释说明两个实例为何不
  会重复施加）、每笔数额不夹零下端地扣减（`debug_assert!(amount
  >= 0)`），然后对一切生物扫 `current < 0`——怪物 despawn、驾驶
  者获得 `Dead`；既非怪物亦非驾驶者的实体不处理。验证：
  `cargo test` 通过世界测试——扣减（12→7）、不夹零（3→-2）、上
  限不变量保持、恰好为零者存活（怪物存留、玩家无标记）、-1 怪物
  被 despawn、-1 玩家携带 `Dead`。

- [x] 2.2 Remove the `#[allow(dead_code)]` on `HitPoints::current` in
  `src/core/health/components/hit_points.rs` (it gains its first
  readers; the ceiling keeps a consumer-named allowance until a
  display or restorative channel reads it). verify:
  `cargo clippy --all-targets` reports zero warnings.

  移除 `src/core/health/components/hit_points.rs` 中
  `HitPoints::current` 上的 `#[allow(dead_code)]`（它迎来第一批读
  者；上限保留注明消费者的豁免，直到生命显示或回血通道读取它）。
  验证：`cargo clippy --all-targets` 零警告。

## 3. Core Phases and Registration | 核心阶段与注册

- [x] 3.1 `src/core/core_phase.rs`: add `PlayerResolve` (after
  `PlayerAct`) and `WorldResolve` (after `WorldAct`), each with a doc
  comment stating what settles there; rewrite the module doc — the
  header's "six system sets" claim is stale (seven are listed today),
  so drop the count and describe the Plan/Act/Resolve symmetry.
  verify: `cargo check --all-targets` passes.

  `src/core/core_phase.rs`：新增 `PlayerResolve`（`PlayerAct` 之后）
  与 `WorldResolve`（`WorldAct` 之后），各自 doc comment 写明在此
  处理之物；重写模块文档——头部“六个系统集”的断言已过时（今天
  已列七个），去掉数量、改写为 Plan/Act/Resolve 对称的描述。验
  证：`cargo check --all-targets` 通过。

- [x] 3.2 `src/core/health/mod.rs`: add the domain `register` —
  register the `Damage` message and `settle_damage` once per resolve
  phase (two instances, one in `PlayerResolve`, one in
  `WorldResolve`); `src/core/mod.rs`: `health::register` joins the
  domain list and the chain gains the two phases in order. verify:
  `cargo test` passes, including a world test wired through the real
  `health::register` that emits `Damage` and observes the outcome
  after one update.

  `src/core/health/mod.rs`：新增域 `register`——注册 `Damage` 消
  息，并把 `settle_damage` 在每个 Resolve 阶段各注册一次（两个实
  例，分属 `PlayerResolve` 与 `WorldResolve`）；
  `src/core/mod.rs`：`health::register` 入列，链上按序接入两个阶
  段。验证：`cargo test` 通过，含一个经真实 `health::register` 接
  线的世界测试——发出 `Damage`，一次 update 后观察到结果。

## 4. Player Domain: The Two Gates | player 域：两处门控

- [x] 4.1 `src/core/player/systems/commands/move_to_cell.rs`: the
  player query gains `Without<Dead>`; commands addressed to the dead
  driver are ignored. verify: `cargo test` passes the new test — a
  dead driver plus a `MoveToCell` command yields no `Path`.

  `src/core/player/systems/commands/move_to_cell.rs`：玩家查询加
  `Without<Dead>`；发给死去驾驶者的命令被无视。验证：`cargo
  test` 通过新测试——死去的驾驶者收到 `MoveToCell` 后不产生
  `Path`。

- [x] 4.2 `src/core/player/systems/plan_move.rs`: the player query
  gains `Without<Dead>`; the dead driver never spends its turn again.
  verify: `cargo test` passes the world-freeze test — a dead driver
  with a queued path never spends its turn, the clock does not advance
  past it, and a monster whose turn lies beyond never comes due.

  `src/core/player/systems/plan_move.rs`：玩家查询加
  `Without<Dead>`；死去的驾驶者不再消耗回合。验证：`cargo test`
  通过世界冻结测试——队列中仍有路径的死去驾驶者不消耗回合，时
  钟不越过该回合推进，回合在更后的怪物永不到期。

## 5. Registry and Quality Gates | 登记表与质量门

- [x] 5.1 Delete OPEN_ISSUES entry 3 and repair the cross-references:
  the ordering-rationale line notes entry 3 landed as
  damage-and-death; entries 4 and 5 rewrite their dependency lines
  (damage channel and death handling now landed). verify:
  `grep -n "条目 3" openspec/OPEN_ISSUES.txt` shows only the
  intentional mentions in the rationale and dependency lines.

  删除 OPEN_ISSUES 条目 3 并修顺交叉引用：顺序理由行注明条目 3
  已以 damage-and-death 落地；条目 4、5 的依赖行改写（伤害通道与
  死亡处理已落地）。验证：`grep -n "条目 3"
  openspec/OPEN_ISSUES.txt` 仅剩顺序理由与依赖行中的有意提及。

- [x] 5.2 Final gates: `cargo +nightly fmt --check`,
  `cargo clippy --all-targets` (zero warnings),
  `cargo check --all-targets`, `cargo test`, and
  `openspec validate damage-and-death` all pass.

  最终质量门：`cargo +nightly fmt --check`、
  `cargo clippy --all-targets`（零警告）、
  `cargo check --all-targets`、`cargo test` 与
  `openspec validate damage-and-death` 全部通过。
