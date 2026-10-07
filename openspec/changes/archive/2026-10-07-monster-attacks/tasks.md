# Tasks: monster-attacks | 任务：monster-attacks

## 1. Blow and Armor Components (combat domain) | 打击与护甲组件（combat 域）

- [x] 1.1 Create `src/core/combat/types/blow.rs`: the `BlowDamage`
  value type (`Fixed(i32)` for flat damage / `Roll(Dice)` for dice)
  with a `roll` helper delegating to the dice domain's roll, and the
  `Blow { chance: i32, damage: BlowDamage }` value type. Doc comments
  state the pairing rule: chance is the attacker's own, armor the
  target's, and a pairing is never stored. verify: unit tests for
  both damage forms' rolls pass with `cargo test`.

  新建 `src/core/combat/types/blow.rs`：`BlowDamage` 值类型
  （`Fixed(i32)` 定值 / `Roll(Dice)` 掷骰），带委托骰子域 roll 的
  `roll` 辅助；`Blow { chance: i32, damage: BlowDamage }` 值类型。
  doc comment 写明配对规则：chance 归攻击方、护甲归目标、配对从不
  落成数据。验证：定值与掷骰两种形式的 roll 单测 `cargo test` 通过。

- [x] 1.2 Create `src/core/combat/components/blows.rs`
  (`Blows(Vec<Blow>)`, the blows an attacker strikes with — an
  absent or empty list means the creature cannot strike) and
  `src/core/combat/components/armor_class.rs` (`ArmorClass(i32)`, the
  armor a strike lands against — absence reads as zero in the hit
  check);
  update the components `mod.rs`. verify: `cargo check --all-targets`
  passes.

  新建 `src/core/combat/components/blows.rs`（`Blows(Vec<Blow>)`，
  攻击方的打击——缺席或空表即该生物不能出手）与
  `src/core/combat/components/armor_class.rs`（`ArmorClass(i32)`，
  出手落点的护甲——缺席在命中判定里按零读）；更新组件 `mod.rs`。验
  证：`cargo check --all-targets` 通过。

## 2. Player-Side Production | 玩家侧生产

- [x] 2.1 Add `derive_strike` (combat domain, Derive phase, chained
  after `derive_combat_bonuses`): on `Changed<CombatBonuses>` write
  the entity's `Blows` — exactly one blow, chance the attack chance,
  damage `Fixed(unarmed damage)` — and its `ArmorClass` from
  `armor_class(...)`. The formulas stay in `combat/utils/`; only the
  call site moves here, and `armor_class`'s dead-code allowance
  deletes. verify: a unit test derives the blow and armor class from
  the bonuses, matching the `attack_chance` / `unarmed_damage` /
  `armor_class` formulas; the formulas' literal values are pinned by
  the utils' own tests. `cargo test` passes.

  新增 `derive_strike`（combat 域，Derive 阶段，链在
  `derive_combat_bonuses` 之后）：`Changed<CombatBonuses>` 触发时写
  实体的 `Blows`——恰好一击，命中品质为攻击值，伤害
  `Fixed(徒手伤害)`——与由 `armor_class(...)` 得出的
  `ArmorClass`。公式原地留在 `combat/utils/`，只有调用点移到此处；
  `armor_class` 的 dead_code 豁免删除。验证：单测按加成派生一击与
  护甲，与 `attack_chance`/`unarmed_damage`/`armor_class` 公式一
  致；公式的具体字面值由 utils 自身单测钉死。`cargo test` 通过。

- [x] 2.2 Pin the derive chain (design D6): a test mutates a `Stats`
  value after birth and asserts the derived `Blows`/`ArmorClass`
  follow on the next frame — every stage writes unconditionally, no
  skip-if-equal. verify: the test passes with `cargo test`.

  钉死派生链（design D6）：测试在出生后修改一项 `Stats`，断言下一
  帧 `Blows`/`ArmorClass` 跟随——每一阶段无条件写入，不做"相同
  即跳过"。验证：该测试 `cargo test` 通过。

## 3. Uniform Strike Resolution | 统一出手解析

- [x] 3.1 Rewrite `act_attack`: query `(Entity, &Attack,
  Option<&Blows>)` with `Added<Attack>`, read the target's
  `Option<&ArmorClass>` fallibly; per blow, `attack_hits(blow.chance,
  armor_or_zero, rng)` and on a hit write one `Damage` for
  `blow.damage.roll(...)`; consume the `Attack` whether or not blows
  exist. No registry, no `CombatBonuses`, no identity checks.
  Register it in both `PlayerAct` and `WorldAct` (the `act_move`
  template). verify: `cargo check --all-targets` passes and
  `combat/mod.rs` shows both phase registrations.

  重写 `act_attack`：查询 `(Entity, &Attack, Option<&Blows>)` 加
  `Added<Attack>`，目标护甲以 `Option<&ArmorClass>` 容错读取；逐击
  `attack_hits(blow.chance, armor_or_zero, rng)`，命中即为
  `blow.damage.roll(...)` 写一条 `Damage`；无论有无打击都消耗
  `Attack`。不读词表、不读 `CombatBonuses`、不做身份判断。注册进
  `PlayerAct` 与 `WorldAct` 两个阶段（`act_move` 模板）。验证：
  `cargo check --all-targets` 通过，`combat/mod.rs` 可见两处阶段
  注册。

- [x] 3.2 Rewrite the `act_attack` tests around the components:
  seeded pins for a hit writing the blow's rolled damage and a miss
  writing nothing; a multi-blow attacker judges and rolls each blow
  on its own; a target without `ArmorClass` reads the threshold as
  zero; an attacker without `Blows` spends the action with no damage;
  a vanished target is skipped (the existing fallibility comment
  stays). verify: the rewritten tests pass with `cargo test`.

  按组件形状重写 `act_attack` 测试：种子钉死"命中写该击掷出的伤
  害"与"未命中什么都不写"；多击攻击方逐击独立判定与掷骰；无
  `ArmorClass` 的目标阈值按零；无 `Blows` 的攻击方空耗动作无伤
  害；目标消失跳过（既有容错注释保留）。验证：重写后的测试
  `cargo test` 通过。

## 4. Monster-Side Production | 怪物侧生产

- [x] 4.1 `spawn_monsters` attaches the blows and armor:
  `ArmorClass` from the kind's armor class, and `Blows` — one blow
  per kind blow, chance `MONSTER_BLOW_POWER_STANDIN` (60) plus three
  times the kind's level, damage `Roll` of the kind blow's dice. The
  stand-in constant lives in the spawning file with its comment
  mapping to the reference's HURT effect power and the OPEN_ISSUES
  entry 10 recovery; `MonsterKind`'s `level`/`blows` dead-code
  allowances delete. verify: a spawn test asserts the rat carries
  `ArmorClass(7)` and one blow of chance 72 with damage 1d3;
  `cargo test` and `cargo clippy --all-targets` pass.

  `spawn_monsters` 挂载打击与护甲：`ArmorClass` 取种类的护甲值，
  `Blows`——种类每条 blow 对应一击，命中品质为
  `MONSTER_BLOW_POWER_STANDIN`（60）加三倍种类等级，伤害为该
  blow 骰子的 `Roll`。顶替常量住在出生文件，注释写明对参照 HURT
  effect 威力的映射与 OPEN_ISSUES 条目 10 的回收；
  `MonsterKind` 的 `level`/`blows` 摘除 dead_code 豁免。验证：出
  生测试断言老鼠携带 `ArmorClass(7)` 与一击（chance 72、伤害
  1d3）；`cargo test` 与 `cargo clippy --all-targets` 通过。

## 5. Monster Planner | 怪物策划

- [x] 5.1 Rename `plan_wander.rs` to `plan_action.rs` (file, system,
  mod registration, module doc): the system plans the monster's turn,
  wandering as the fallback branch. verify: no `plan_wander` remnant in
  live code (`grep -r plan_wander src/` is empty; the rename narrative
  in openspec is intended) and `cargo check --all-targets` passes.

  `plan_wander.rs` 改名 `plan_action.rs`（文件、系统、mod 注册、模
  块 doc）：系统职责是策划怪物的回合，游荡为回落分支。验证：
  `grep -r plan_wander src/` 无残留（openspec 内为改名叙述），
  `cargo check --all-targets` 通过。

- [x] 5.2 The attack branch precedes the wander (design D1): with the
  living player in one of the eight neighboring cells, a due monster
  inserts `Attack { target: player }`, plans no step, and prices its
  turn as usual; otherwise the wander runs unchanged. The player
  query carries the entity and stays `Without<Dead>`. Tests: adjacent
  living player → `Attack` inserted, no `Move`, slot advances (and a
  seeded "stand" roll still strikes — the branch precedes the 75%
  roll); distant player → the wander runs; dead player → no strike.
  verify: the new tests plus every renamed wander test pass with
  `cargo test`.

  攻击分支先于游荡（design D1）：存活主角在八邻格之一时，到期怪物
  插入 `Attack { target: 主角 }`、不策划步伐、回合照价消耗；否则
  游荡照旧。主角查询携带实体并保持 `Without<Dead>`。测试：相邻活
  主角 → 插入 `Attack`、无 `Move`、回合槽推进（且种子钉死"站住"
  掷点下仍出手——分支先于 75% 掷点）；不相邻 → 游荡运行；主角死
  亡 → 不出手。验证：新测试与全部改名游荡测试 `cargo test` 通
  过。

## 6. Whole-App Regression | 完整应用回归

- [x] 6.1 In `src/core/mod.rs`: `the_real_assembly_bites_back` —
  arrange one rat adjacent to the player, keep the world running by
  re-issuing the player's standing order as bites clear it, and assert
  the player's hit points drop below the ceiling — the drop itself
  runs planning → resolution → channel → application, all real. The
  slay sweep renames to `the_real_assembly_hunts_the_rats`: with the
  rat biting back the bounty is no longer guaranteed, so the sweep
  asserts its spread instead (some hunts slay their rat, some fall to
  the pack). verify: both real-assembly tests pass with `cargo test`.

  在 `src/core/mod.rs` 加 `the_real_assembly_bites_back`：摆一只老
  鼠与主角相邻，靠反复重发站立指令（咬伤会清指令）维持世界运转，
  断言主角生命值跌破上限——血降本身就走完策划 → 解析 → 通道 →
  施加，全真。杀鼠扫荡改名 `the_real_assembly_hunts_the_rats`：老
  鼠还手后战果不再保证，扫荡改为断言分布（有的猎杀成功，有的反被
  咬死）。验证：两个完整应用测试 `cargo test` 通过。

## 7. Registry and Gates | 登记表与门禁

- [x] 7.1 Update `openspec/OPEN_ISSUES.txt`: delete entry 5 (this
  change lands it); the minimal-loop support list gains the monster
  strike; entry 10's recovery note gains the power stand-in's home
  (the spawn translation constant). verify: the file reads coherently
  and `grep -n "条目 5\|^5\." openspec/OPEN_ISSUES.txt` shows the
  entry gone.

  更新 `openspec/OPEN_ISSUES.txt`：删除条目 5（本条落地）；最小闭
  环支撑系统清单补怪物出手；条目 10 的回收注记补威力顶替值的落点
  （出生翻译处常量）。验证：文件通读连贯，`grep -n "条目 5\|^5\."
  openspec/OPEN_ISSUES.txt` 确认条目已删。

- [x] 7.2 Full gates: `cargo +nightly fmt --check`, `cargo clippy
  --all-targets` zero warnings, `cargo check --all-targets`, `cargo
  test` all green, `openspec validate monster-attacks` valid, and the
  language blacklist finds no hit in the touched files. verify: every
  gate passes.

  全门禁：`cargo +nightly fmt --check`、`cargo clippy
  --all-targets` 零警告、`cargo check --all-targets`、`cargo
  test` 全绿、`openspec validate monster-attacks` 通过、语言黑名
  单在触及文件零命中。验证：各门全过。
