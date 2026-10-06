# Proposal: combat-bonuses | 变更提案：combat-bonuses

## Why

The second entry of the minimal combat loop (OPEN_ISSUES entries 1-6).
The monster-side combat profile is in place (hit points, armor class,
level, damage dice), while the player side has no combat numbers at
all: the hit, damage, and armor derivations do not exist, leaving
entries 4 (player attacks) and 5 (monster attacks) with nothing to draw
from. This entry supplies the player-side combat derivations in one
pass — the four stat bonus tables and the three composite formulas are
the shared material of the two attack entries that follow.

最小战斗闭环（OPEN_ISSUES 条目 1-6）的第二条。怪物侧战斗档案已就位
（生命、护甲、等级、伤害骰），玩家侧没有任何战斗数值：命中、伤害、
护甲的派生通道不存在，条目 4（玩家攻击怪物）与条目 5（怪物攻击主角）
届时无数值可取。本条把玩家战斗数值的派生工具一次备齐——四张属性
修正表与三个合成公式是后续两条攻击条目的公共取材。

## What Changes

- New `combat` domain: derivations from the six statistics to combat
  numbers.

  新增 combat 域：六维到战斗数值的派生。

  - Four 38-entry stat bonus tables, value-by-value aligned with their
    reference counterparts: strength-to-hit, strength-to-damage,
    dexterity-to-hit, dexterity-to-armor. Tables are stored in
    source-table form (128 bias included) and read through the
    compressed statistic index, with readers subtracting the bias. The
    existing constitution table is brought to the same form (values
    unchanged).

    四张 38 项属性修正表，逐值对齐参考实现的对应表：力量到命中、
    力量到伤害、敏捷到命中、敏捷到护甲；表按源表原值形态存储
    （含 128 零点），读取处减 128，按压缩属性索引查值。既有体质
    生命表一并改为同形态（数值不变）。

  - Three derived bonuses: hit = strength hit + dexterity hit; damage =
    strength damage; armor = dexterity armor. Table reads take the
    current statistic values.

    三个派生值：to_h = 力量命中修正 + 敏捷命中修正；to_d = 力量伤害
    修正；to_a = 敏捷护甲修正。查表取六维的当前值。

  - Attack chance: chance = melee skill term + hit × 3. The melee skill
    formula lands in this entry (stepwise integer divisions), fed by
    two stand-in factors (the level-one warrior's bases: melee style 1,
    combat 2 — the skill term works out to 5). The skill system does
    not exist; design records the derivation chain and the endgame
    source mapping, to be recovered under OPEN_ISSUES entry 8 by
    swapping in real skill reads while the formula stays untouched.

    攻击值：chance = 命中技能项 + to_h × 3。命中技能公式本条落地
    （逐步整数除法），其两个技能因子为顶替常量（一级战士基值：近战
    风格 1、战斗 2，算得命中技能项 5）；技能系统不存在，design 记录
    推导链与终局来源的映射（OPEN_ISSUES 条目 8 回收，届时换真实技能
    读取、公式不动）。

  - Unarmed damage: 1 + damage bonus, negative results floored at zero.
    Weapon dice and criticals depend on the equipment system and stay
    out of this entry.

    徒手伤害：1 + to_d，负值截 0。武器骰与暴击依赖装备系统，不入
    本条。

  - Player armor class: ac + armor bonus; the equipment system does not
    exist, so ac is always 0.

    玩家护甲：ac + to_a；装备系统不存在，ac 恒 0。

- The compressed statistic index (one bracket per point from 3 to 18,
  one bracket per ten points of 18/x, 38 brackets ending at 18/220) is
  held publicly by the stats domain; the health domain's hit-point
  derivation and the combat domain's bonus tables read through the same
  scale.

  压缩属性索引（3–18 逐点、18/x 每十点一段、18/220 起共 38 段）由
  stats 域公开持有，health 域的生命派生与 combat 域的修正表经同一
  刻度取索引。

- Zero observable gameplay change in this entry: no actions or systems
  are wired; the numbers serve entries 4 and 5, and verification lives
  entirely in unit tests.

  本条零游戏行为变化：不接任何行动与系统，数值仅供条目 4、5 消费，
  验证全在单测层。

- Out of this entry (supporting systems absent; see OPEN_ISSUES
  entries 7-10): true skill values, weapon to-hit/to-damage and weapon
  dice, the five critical tiers, multiple blows, the stance and
  movement posture tables.

  不入本条（支撑系统不存在，见 OPEN_ISSUES 条目 7-10）：技能真值、
  武器 to_h/to_d 与武器骰、暴击五档、多击、站姿与移动姿态表。

## Capabilities

### New Capabilities

- `combat`: player-side combat-number derivation — stat bonus table
  reads, the composition of hit/damage/armor bonuses, attack chance,
  unarmed damage, and player armor class.

  `combat`：玩家战斗数值派生——属性修正表的查值、to_h/to_d/to_a
  的合成、攻击值、徒手伤害、玩家护甲。

### Modified Capabilities

- `stats`: adds the "Compressed Statistic Index" requirement — the
  38-bracket compressed scale becomes a public behavior of the stats
  domain, shared by every derivation channel.

  `stats`：新增"压缩属性索引"requirement——38 段压缩刻度成为
  stats 域的公开行为，供各派生通道共用。

## Impact

- New domain `src/core/combat/`: constants (the four bonus tables),
  components (the combat-bonus component), systems (the derive system
  reacting to Stats changes), utils (table-read and composition pure
  functions, serving the derive system and the future attack entries).

  新域 `src/core/combat/`：constants（四张修正表）、components
  （战斗修正组件）、systems（响应 Stats 变化的派生系统）、utils
  （查表与合成纯函数，供派生系统与后续攻击条目调用）。

- `src/core/stats/utils/` exposes the compressed statistic index;
  `src/core/health/utils/hp.rs` reads the index through the stats
  domain, behavior unchanged.

  `src/core/stats/utils/` 公开压缩属性索引；
  `src/core/health/utils/hp.rs` 经 stats 域取索引，行为不变。

- Specs: new `specs/combat/spec.md`; `specs/stats/spec.md` gains one
  requirement.

  spec 侧：新增 `specs/combat/spec.md`；`specs/stats/spec.md` 增一条
  requirement。

- Design: records the stand-in factors of the melee skill term with
  their endgame source mapping.

  design 侧：登记命中技能顶替因子与其终局来源的映射。

- No data-file, frontend, assembly, or dependency changes.

  数据文件、前端、装配、依赖均不变。
