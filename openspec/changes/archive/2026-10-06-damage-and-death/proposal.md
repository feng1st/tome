# Proposal: damage-and-death | 变更提案：damage-and-death

## Why

The third entry of the minimal combat loop (OPEN_ISSUES entries 1-6).
Every woundable creature already carries hit points, yet nothing can
spend them: no damage entry exists, no zero-crossing handling exists,
and a dead monster or player would walk on forever. Entries 4 (player
attacks) and 5 (monster attacks) each need to hurt a creature; this
entry lands the single channel they will both plug into — damage
application and death handling, independently testable before any
producer exists.

最小战斗闭环（OPEN_ISSUES 条目 1-6）的第三条。一切可受伤生物都已携
带生命值，但没有任何途径消耗它：伤害入口不存在，归零处理不存在，
怪物或玩家死后仍会照常行动。条目 4（玩家攻击）与条目 5（怪物攻击）
都需要伤害一个生物；本条落地二者共同接入的唯一通道——伤害施加与
死亡处理，在没有任何生产者时即可独立测试。

## What Changes

- A damage channel in the health domain: a `Damage { target, amount }`
  message (the only entry through which creatures are hurt) and a
  settle system that drains the queue and subtracts `amount` from the
  target's current hit points. The subtraction is not clamped at zero —
  negative values are legal, copied from the reference implementation;
  the message contract is `amount >= 0`, so the current value only ever
  decreases and the ceiling invariant is preserved.

  health 域的伤害通道：`Damage { target, amount }` 消息（伤害生物的
  唯一入口）与 settle 系统——排干消息队列，从目标当前生命值中减去
  amount。减法的下端不夹零——负值合法，照搬参考实现；消息契约为
  amount >= 0，当前值只减不增，上限不变量自然保持。

- Death handling at strictly negative hit points, one sweep after the
  subtraction: a creature at exactly zero is alive. A monster below
  zero is despawned (its entity leaves the world, figure included); the
  player below zero gains a `Dead` marker component.

  严格负值的死亡处理，紧随扣减的一次扫尾：恰好为零的生物仍然活
  着。生命值小于零的怪物被 despawn（实体移出世界，形象随实体一
  并移除）；生命值小于零的玩家获得 `Dead` 标记组件。

- Two new core phases so ordering stays in the phase enums:
  `PlayerResolve` after `PlayerAct` and `WorldResolve` after
  `WorldAct`; the settle system registers once per resolve phase.
  Damage a player deals in `PlayerAct` is fully processed before
  `WorldPlan`, so a slain monster never plans or acts again; damage
  the world deals in `WorldAct` is fully processed before the next
  frame, so a slain player never plans another step.

  核心阶段新增两个，排序留在阶段枚举里：`PlayerAct` 之后的
  `PlayerResolve` 与 `WorldAct` 之后的 `WorldResolve`；settle 系
  统在每个 Resolve 阶段各注册一次。玩家在 `PlayerAct` 造成的伤
  害在 `WorldPlan` 之前处理完毕，被杀的怪物不再策划或行动；世
  界在 `WorldAct` 造成的伤害在下一帧之前处理完毕，被杀的玩家
  不再策划任何一步。

- The dead driver stops: command execution (`MoveToCell`) and the
  driver's step planning ignore the player carrying `Dead`. With the
  driver's due turn never spent again, the clock pins at that turn and
  the world freezes — no gate in monster planning is needed.

  死去的驾驶者停住：命令执行（`MoveToCell`）与驾驶者的步进策划无
  视携带 `Dead` 的玩家。驾驶者到期的回合不再被消耗，时钟钉在该回
  合上，世界随之冻结——怪物策划无需任何死亡门控。

- Zero damage producers in this entry: no attack actions are wired;
  entries 4 and 5 plug in later. Verification lives in unit and world
  tests that emit `Damage` directly.

  本条零伤害生产者：不接任何攻击行动，条目 4、5 后续接入。验证全
  在单测与世界测试层，由测试直接发出 `Damage`。

- Out of this entry (supporting systems absent; see OPEN_ISSUES
  entries 6, 11-13): the death presentation (tombstone and any state
  switch, entry 6), experience, drops and corpses, monster fear rolls.

  不入本条（支撑系统不存在，见 OPEN_ISSUES 条目 6、11-13）：死亡
  表现（墓碑与可能的状态切换，条目 6）、经验、掉落与尸体、怪物
  恐惧掷点。

## Capabilities

### New Capabilities

(none | 无)

### Modified Capabilities

- `health`: adds the damage channel and death handling requirements —
  spending hit points through the `Damage` message, the strictly
  negative death threshold, monster removal, and the player's `Dead`
  marker with the driver's stop.

  `health`：新增伤害通道与死亡处理 requirement——经 `Damage` 消息
  消耗生命值、严格负值的死亡界线、怪物移除、玩家的 `Dead` 标记与
  驾驶者停住。

## Impact

- `src/core/health/` grows from a component-only domain into a
  registered one: `messages/damage.rs` (the channel message),
  `systems/settle_damage.rs` (drain, subtract, sweep deaths),
  `components/dead.rs` (the player marker), and the domain `register`.

  `src/core/health/` 从纯组件域升级为有注册的域：
  `messages/damage.rs`（通道消息）、`systems/settle_damage.rs`
  （排干、扣减、扫尾死亡）、`components/dead.rs`（玩家标记）与域
  `register`。

- `src/core/core_phase.rs`: two new phases (`PlayerResolve`,
  `WorldResolve`) and a module-doc rewrite — the header still says
  "six system sets" while listing seven; the count claim is dropped
  for good. `src/core/mod.rs`: the chain gains the two phases and
  `health::register` joins the domain list.

  `src/core/core_phase.rs`：新增两个阶段（`PlayerResolve`、
  `WorldResolve`）并重写模块文档——头部仍写“六个系统集”而实际
  已列七个，数量断言一并去除。`src/core/mod.rs`：链上接两个阶段，
  `health::register` 入列。

- `src/core/player/`: `MoveToCell` execution and step planning gain a
  `Without<Dead>` filter each.

  `src/core/player/`：`MoveToCell` 执行与步进策划各加一个
  `Without<Dead>` 过滤。

- The `#[allow(dead_code)]` on the `HitPoints` current value is
  removed — it gains its first readers; the ceiling keeps its
  allowance until a display or restorative channel consumes it.

  `HitPoints` 当前值字段上的 `#[allow(dead_code)]` 移除——它迎来
  第一批读者；上限字段的豁免保留，直到生命显示或回血通道消费它。

- Specs: `specs/health/spec.md` gains the damage channel and death
  requirements.

  spec 侧：`specs/health/spec.md` 增伤害通道与死亡 requirement。

- No data-file, frontend, assembly, or dependency changes. The
  `AppState` tree is untouched — the death presentation decides later
  whether a mode switch is warranted.

  数据文件、前端、装配、依赖均不变；`AppState` 树不动——是否升
  格为模式切换由死亡表现届时定夺。
