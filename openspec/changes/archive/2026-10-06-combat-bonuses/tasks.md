# Tasks: combat-bonuses | 任务：combat-bonuses

## 1. Stats Domain: Publish the Index and the Bias | stats 域：压缩索引与表零点公开

- [x] 1.1 Create `src/core/stats/utils/stat_bonus_index.rs`: publish
  `stat_bonus_index` (the 38-bracket compressed scale, moved in from
  `health/utils/hp.rs`). verify: `cargo test` passes the bracket
  landmark tests — at or below 3 lands on the first bracket; 4–18 one
  bracket per point (18 on the sixteenth); one bracket per ten points
  of x from 18/10 (28) on; 18/220 (238) and beyond on the
  thirty-eighth.

  新建 `src/core/stats/utils/stat_bonus_index.rs`：公开
  `stat_bonus_index`（38 段压缩刻度，实现自 `health/utils/hp.rs` 迁入）。
  验证：`cargo test` 通过分段地标单测——不大于 3 落第一段；4–18 逐点
  一段（18 落第十六段）；18/10（28）起每十点 x 一段；18/220（238）及
  以上落第三十八段。

- [x] 1.2 Create `src/core/stats/constants/stat_bonus_table.rs`:
  publish the stat-bonus-table bias constant (128). verify:
  `cargo check --all-targets` passes.

  新建 `src/core/stats/constants/stat_bonus_table.rs`：公开属性
  加成表零点常量（128）。验证：`cargo check --all-targets` 通过。

## 2. Health Domain: Align the Constitution Table to Source Form | health 域：体质表对齐原值形态

- [x] 2.1 `CONSTITUTION_HP_BONUS` stores the source-table values with
  the bias included (values unchanged, 128 added to each entry);
  `constitution_hp_bonus` evaluates through the stats domain's index
  and bias; the private index in hp.rs is deleted. verify: `cargo test`
  keeps the existing health tests green, untouched (behavior
  unchanged).

  `CONSTITUTION_HP_BONUS` 改存含零点的源表原值（数值不变，每项
  加 128）；`constitution_hp_bonus` 经 stats 域的索引与零点求值；hp.rs
  的私有索引删除。验证：`cargo test` 既有 health 测试一行不改全绿
  （行为不变）。

## 3. Combat Domain: Tables, Derivation, Component, Composites | combat 域：表、派生、组件、合成公式

- [x] 3.1 Create `src/core/combat/constants/stat_bonus_tables.rs`: the
  four 38-entry tables in source form (strength hit, strength damage,
  dexterity hit, dexterity armor), value-by-value aligned with the four
  reference anchors (tome2 src/tables.c:472, :518, :564, :610). verify: `cargo test`
  passes the value-by-value tests (all 38 entries asserted per table).

  新建 `src/core/combat/constants/stat_bonus_tables.rs`：四张
  38 项原值表（力量命中、力量伤害、敏捷命中、敏捷护甲），逐值对齐
  四个参考锚点（tome2 src/tables.c:472、:518、:564、:610）。验证：`cargo test` 逐值测试通过
  （每表 38 项逐项断言）。

- [x] 3.2 Create `src/core/combat/utils/combat_bonus.rs`: the three
  table-read derivations (reading the current statistic values, bias
  subtracted) — hit = strength hit + dexterity hit, damage = strength
  damage, armor = dexterity armor. verify: `cargo test` passes
  hand-computed cases, including negative bonuses from low statistics.

  新建 `src/core/combat/utils/combat_bonus.rs`：三个查表派生
  （取六维当前值、减零点）——命中 = 力量命中 + 敏捷命中，伤害 = 力量
  伤害，护甲 = 敏捷护甲。验证：`cargo test` 手算用例通过，含负修正
  用例（低属性值得负修正）。

- [x] 3.3 Create `src/core/combat/components/combat_bonuses.rs`:
  the `CombatBonuses { hit, damage, armor }` component. verify:
  `cargo check --all-targets` passes.

  新建 `src/core/combat/components/combat_bonuses.rs`：
  `CombatBonuses { hit, damage, armor }` 组件。验证：
  `cargo check --all-targets` 通过。

- [x] 3.4 Create `src/core/combat/systems/derive_combat_bonuses.rs`,
  register it in the combat domain, and mount it in the core root
  module: react to `Changed<Stats>` with wholesale recomputation,
  inserting or overwriting the component. verify: `cargo test` passes
  the world test — after spawn_player, running the system yields the
  component with hand-computed values; after a current statistic value
  changes and the system reruns, the component is overwritten with the
  new values.

  新建 `src/core/combat/systems/derive_combat_bonuses.rs` 并在
  combat 域 register、core 根模块挂载：响应 `Changed<Stats>` 整体重算
  并插入/覆盖组件。验证：`cargo test` 世界测试通过——spawn_player
  后跑系统得组件、三值与手算一致；改动 Stats 当前值后再跑，组件被新值
  覆盖。

- [x] 3.5 Create `src/core/combat/utils/attack.rs`: the melee skill
  formula `50×((7×style + 3×combat) / 10) / 10` (both divisions in
  source order), the stand-in factor constants (melee style 1, combat
  2; their doc comment spells out the factor semantics, the served
  formula, the recovery path, and the stand-in boundary, naming no
  reference project), attack chance (skill term + hit ×
  3), and unarmed damage
  (max(0, 1 + damage)); no production consumers in this entry —
  transition with `#[allow(dead_code)]` and a comment naming the
  consumers. verify: `cargo test` passes — factors (1, 2) yield 5
  stepwise (not the folded /100 figure 6); the chance composition;
  unarmed damage floored at zero.

  新建 `src/core/combat/utils/attack.rs`：命中技能公式
  `50×((7×style + 3×combat) / 10) / 10`（两步整除按源码次序）、顶替
  因子常量（近战风格 1、战斗 2；doc comment 写明因子语义、服务公式、
  回收去向与顶替边界，不提参考工程）、攻击值（命中技能项 +
  命中 × 3）、徒手伤害（max(0, 1 + 伤害)）；本轮无生产消费方，以
  `#[allow(dead_code)]` 过渡并注释注明消费方。验证：`cargo test`
  通过——因子 (1, 2) 经逐步整除得 5（非 /100 之 6）；攻击值合成；
  徒手负修正截 0。

- [x] 3.6 Create `src/core/combat/utils/armor_class.rs`: player armor
  class = armor bonus (equipment base 0); same `#[allow(dead_code)]`
  transition. verify: `cargo test` passes.

  新建 `src/core/combat/utils/armor_class.rs`：玩家护甲 = 护甲
  修正（装备基数 0）；同款 `#[allow(dead_code)]` 过渡。验证：
  `cargo test` 通过。

## 4. Wrap-up | 收尾

- [x] 4.1 Quality gate: `cargo +nightly fmt --check`,
  `cargo clippy --all-targets`, `cargo check --all-targets`, and
  `cargo test` all green with zero warnings.

  质量门：`cargo +nightly fmt --check`、
  `cargo clippy --all-targets`、`cargo check --all-targets`、
  `cargo test` 全绿零警告。

- [x] 4.2 Remove entry 2 from OPEN_ISSUES.txt (entry 8's recovery
  registry stays). verify: entry 2 no longer exists in OPEN_ISSUES.txt,
  and the numbering and cross-references stay consistent.

  删除 OPEN_ISSUES.txt 条目 2（条目 8 的回收登记保留）。
  验证：OPEN_ISSUES.txt 中条目 2 不复存在，条目编号与交叉引用保持
  自洽。
