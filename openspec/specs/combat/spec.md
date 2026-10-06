# combat Specification | combat 规格

## Purpose

Player-side combat numbers: the melee bonuses derived from the six
statistics, and the composite formulas built on them (attack chance,
unarmed damage, armor class). Monster-side combat numbers come from the
vocabulary, not from this capability.

玩家侧战斗数值：由六维派生的近战修正，以及立于其上的合成公式（攻击
值、徒手伤害、护甲）。怪物侧战斗数值出自词表，不经本能力。

## Requirements

### Requirement: Stat Bonus Tables | 属性修正表

The four stat bonus tables (strength-to-hit, strength-to-damage,
dexterity-to-hit, dexterity-to-armor) SHALL each hold 38 entries indexed
by the compressed statistic index, storing the source-table values with
their 128 bias; a reader subtracts the bias to obtain the bonus.

四张属性修正表（力量到命中、力量到伤害、敏捷到命中、敏捷到护甲）
SHALL 各持 38 项，按压缩属性索引查值；表存含 128 零点的源表原值，
读取方减去零点得修正。

#### Scenario: Every bracket yields the reference value | 每个分段给出参考值

- **WHEN** any of the four tables is read at any of the 38 brackets | 在 38 个分段的任一段读取四张表中的任一张
- **THEN** the value equals the reference table's entry at that bracket | 读得值等于参考表对应分段的表项

### Requirement: Combat Bonus Derivation | 战斗修正派生

The three combat bonuses SHALL derive from the statistics' current
values: the hit bonus is the strength-to-hit entry plus the
dexterity-to-hit entry, the damage bonus is the strength-to-damage
entry, and the armor bonus is the dexterity-to-armor entry.

三个战斗修正 SHALL 由六维的当前值派生：命中修正为力量命中项加敏捷
命中项，伤害修正为力量伤害项，护甲修正为敏捷护甲项。

#### Scenario: Bonuses are a fixed function of the current values | 修正为当前值的确定函数

- **WHEN** a playable character's six current values are given | 给定一个可被扮演角色的六维当前值
- **THEN** the three combat bonuses are determined, equal to their respective table entries combined as above | 三个战斗修正是确定的，各自等于上述表项的组合

### Requirement: Combat Bonus Component | 战斗修正组件

A playable character SHALL carry a combat-bonus component holding the
three derived bonuses; the component is recomputed from scratch whenever
the statistics change, birth included.

可被扮演角色 SHALL 携带战斗修正组件，记录三个派生修正；六维一旦变化
（含出生），组件即整体重算。

#### Scenario: Born with the bonuses | 出生携带修正

- **WHEN** a playable character is born | 一个可被扮演角色出生时
- **THEN** the entity carries the combat-bonus component, its values equal to the derivation from the birthed statistics | 实体携带战斗修正组件，其值等于由出生六维派生的结果

#### Scenario: A statistics change triggers recomputation | 六维变化触发重算

- **WHEN** a carried statistic value changes | 已携带的六维值发生变化时
- **THEN** the component is recomputed from the new current values | 组件按新的当前值整体重算

### Requirement: Attack Chance | 攻击值

The attack chance SHALL equal the melee skill term plus three times the
hit bonus. Until the skill system lands, the melee skill term is the
fixed stand-in 5 — the value the reference formula yields for the
level-one warrior's skill bases under its stepwise integer divisions.

攻击值 SHALL 等于命中技能项加三倍命中修正。技能系统落地前，命中技能
项为固定顶替值 5——参考公式代入一级战士的技能基值、按其逐步整数
除法算得的结果。

#### Scenario: The chance is a fixed function of the hit bonus | 攻击值为命中修正的确定函数

- **WHEN** the hit bonus is given | 给定命中修正时
- **THEN** the attack chance equals 5 plus three times the hit bonus | 攻击值等于 5 加三倍命中修正

### Requirement: Unarmed Damage | 徒手伤害

Unarmed damage SHALL be 1 plus the damage bonus, floored at zero.

徒手伤害 SHALL 为 1 加伤害修正，负值截 0。

#### Scenario: A positive bonus adds up | 正修正累加

- **WHEN** the damage bonus is positive | 伤害修正为正时
- **THEN** unarmed damage equals 1 plus the bonus | 徒手伤害等于 1 加修正

#### Scenario: A negative bonus is floored at zero | 负修正截 0

- **WHEN** 1 plus the damage bonus is negative | 1 加伤害修正为负时
- **THEN** unarmed damage is zero | 徒手伤害为 0

### Requirement: Player Armor Class | 玩家护甲

A player's armor class SHALL be the armor bonus; the equipment-granted
base is zero while the equipment system does not exist.

玩家护甲 SHALL 为护甲修正；装备系统不存在期间，装备给予的基数为 0。

#### Scenario: Armor class equals the armor bonus | 护甲等于护甲修正

- **WHEN** a player's armor class is read | 读取玩家护甲时
- **THEN** it equals the armor bonus derived from dexterity | 其值等于由敏捷派生的护甲修正
