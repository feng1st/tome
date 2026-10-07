# combat delta for combat-feedback | combat 增量：combat-feedback

## Terminology | 术语

The attack pipeline's words, kept apart where everyday language blurs
them (a *hit* is a judgement; *damage* is an amount; *chance* is a
quality number, never a probability in this pipeline):

攻击管线的用词表，把日常语言里容易混的地方钉死（*hit* 是判定，
*damage* 是数额，*chance* 是品质值、在本管线里从来不是概率）：

| Term 术语 | Description | 释义 |
|---|---|---|
| attack 攻击 | one consumed attack action; it comprises the attacker's blows | 一次被消耗的攻击行动，由攻击方携带的若干打击组成 |
| blow 打击 | one swing unit the attacker carries: its own chance and damage | 攻击方携带的一个挥击单元：自带命中率与伤害 |
| chance 命中率 | a blow's own to-hit quality number, judged against the target's armor class; the player's composite of it is the attack chance | 打击自带的命中数值，对着目标护甲等级判定；玩家侧对它的合成值即攻击命中率（attack chance） |
| hit 命中 | whether a blow lands — a judgement, never an amount | 一击是否落中——判定，不是数值 |
| heavy hit 重击 | a landed blow heavy enough to matter beyond the wound itself (as the camera shake); heavy is judged by damage, a distinct word from hit | 落中且重到伤口之外还有动静的一击（如震屏）；重按伤害论，与 hit 是两个词 |
| damage 攻击伤害 | the amount a landed blow subtracts — a fixed value or one roll of its dice | 落中一击所扣减的数额——定值，或其骰子的一次掷出 |
| damage amount 攻击伤害值 | one blow's damage value; shortens to `amount` where the context is clear | 单击的伤害数值；上下文明确时可简称 amount |
| blow damage 打击伤害 | the damage a blow carries — a fixed value or one roll of its dice, whether declared in the vocabulary or derived | 打击携带的伤害——定值或其骰子的一次掷出，词表声明或派生皆可 |
| armor class 护甲等级 | the number a blow's chance is judged against | 打击命中率判定所对的数值 |
| stat bonus 属性修正 | a table-looked modifier from one statistic's current value | 按某一项属性的当前值查表得出的修正 |
| hit bonus 命中修正 | the stat bonus feeding the attack chance: strength-to-hit plus dexterity-to-hit | 进攻击命中率的属性修正：力量命中项加敏捷命中项 |
| damage bonus 伤害修正 | the stat bonus feeding unarmed damage: strength-to-damage | 进徒手伤害的属性修正：力量伤害项 |
| armor bonus 护甲修正 | the stat bonus feeding the armor class: dexterity-to-armor | 进护甲等级的属性修正：敏捷护甲项 |

## MODIFIED Requirements

### Requirement: Attack Resolution | 攻击解析

Every planned attack SHALL resolve through one shared pipeline: for
each blow the attacker carries, judge the hit (see Hit Determination)
against the target; on a hit issue exactly one damage request for
that blow's damage — a fixed amount, or one roll of its dice; on a
miss issue nothing. The attack action
SHALL be consumed either way. A target that left the world or already
carries the death marker SHALL be skipped — the action is spent and
nothing issues. Attacks planned by the player resolve
in the player's acting phase; attacks planned by a monster resolve in
the world's acting phase. Resolution MUST NOT read any vocabulary and
MUST NOT branch on what the attacker or the target is — every number
comes from the blows and armor-class components the creatures carry.

Whether hit or miss, the resolution SHALL also emit an attack-resolved
fact naming the attacker, the attacker's cell, the target's cell, and
the landed blows' damage amounts in blow order (`damage_amounts`; an
empty list when every blow missed). The fact is a past-tense
broadcast: it MUST NOT alter resolution, and any number of readers MAY
consume it.

每一次策划出的攻击 SHALL 经同一条共享管线解析：对攻击方携带的每
一击，判定其命中（见"命中判定"）；命中则恰好发出一条伤害请求，
数值为该击的伤害——定值，或其骰子的一次掷出；未命中则什么都不
发出。
攻击动作 SHALL 照例消耗。已离开世界或已携带死亡标记的目标 SHALL
被跳过——动作照耗、什么都不发出。玩家策划的攻击在玩家执行阶段
解析；怪物策划的攻击在世界执行阶段解析。解析 MUST NOT 读任何词
表，也 MUST NOT 按攻击方或目标是谁而分支——一切数值取自生物携
带的打击与护甲等级组件。

无论命中与否，解析 SHALL 同时发出一条攻击结果事实，指明攻击
方、攻击方所在格、目标格与各命中击的伤害数值（按击序，
`damage_amounts`；全部未命中时为空表）。事实是过去时广播：MUST
NOT 改变解析本身，任意数量的读者 MAY 消费它。

#### Scenario: Every blow judges and rolls on its own | 每击独立判定与掷骰

- **WHEN** an attack resolves for an attacker carrying several blows | 攻击方携带多击的攻击解析时
- **THEN** each blow judges the hit on its own chance and issues its own damage request on a hit | 每击按自身命中率独立判定，命中各发各的伤害请求

#### Scenario: A target without armor takes no threshold | 无护甲等级目标无阈值

- **WHEN** an attack's target carries no armor-class component | 攻击的目标不携带护甲等级组件时
- **THEN** three quarters of armor reads as zero, and a blow with a positive chance always lands past the certain bands | 护甲等级的四分之三按零读，命中率为正的一击过带必中

#### Scenario: An attacker without blows attacks nothing | 无打击者攻击为空

- **WHEN** an attack resolves for an attacker carrying no blows | 攻击方不携带打击的攻击解析时
- **THEN** no damage request issues, and the attack action is spent as usual | 不产生任何伤害请求，攻击动作照常消耗

#### Scenario: An attack emits its fact | 攻击发出其结果事实

- **WHEN** an attack resolves with one landed blow for 4 and one missed blow | 一次攻击解析为一击命中伤害 4、一击未中时
- **THEN** the attack-resolved fact names the attacker, both cells, and `damage_amounts` of 4 | 攻击结果事实指明攻击方、双方格坐标与 `damage_amounts` 数值 4

#### Scenario: A dead target is skipped | 死亡目标被跳过

- **WHEN** an attack resolves against a target carrying the death marker | 攻击解析的对象携带死亡标记时
- **THEN** no damage request and no attack-resolved fact issue, and the attack action is spent | 不产生伤害请求与攻击结果事实，攻击动作照常消耗

#### Scenario: A full miss emits an empty fact | 全失攻击发出空事实

- **WHEN** an attack resolves with every blow missed | 一次攻击的所有击全部未中时
- **THEN** the attack-resolved fact carries an empty `damage_amounts`, and the attack action is still consumed | 攻击结果事实携带空 `damage_amounts`，攻击动作照常消耗
