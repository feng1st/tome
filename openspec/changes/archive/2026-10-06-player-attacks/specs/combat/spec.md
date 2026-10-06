# combat Specification (delta) | combat 规格（增量）

## ADDED Requirements

### Requirement: Attack Order | 攻击指令

When the player's click targets a cell holding a living monster, the
game SHALL issue the player a standing attack order naming that
monster, replacing any standing order; such a click MUST NOT produce
movement. While the order stands, each of the player's due turns
SHALL plan from it: when the target is adjacent (any of the eight
neighboring cells), the turn SHALL plan a strike at the target, be
spent, and clear the order — one order, one strike; otherwise the
turn SHALL plan one step toward the target's current cell and be
spent; when no route to the target exists, or the target is no longer
in the world, the order SHALL be cleared without the turn being spent.

玩家的点击目标格持有活怪物时，游戏 SHALL 向玩家发出以该怪物为目标
的站立攻击指令，替换任何既有站立指令；此类点击 MUST NOT 产生移
动。指令存续期间，玩家每个到期回合 SHALL 按它策划：目标相邻（八
方向任一邻格）时，本回合 SHALL 策划一次对它的出手、消耗回合并清
除指令——一道指令一次出手；不相邻时，本回合 SHALL 朝目标当前格
策划一步并消耗回合；目标无路可达、或目标已不在世界中时 SHALL
清除指令且不消耗回合。

#### Scenario: An adjacent strike clears the order | 相邻出手即清指令

- **WHEN** the player holds an attack order whose target is adjacent and the player's turn comes due | 玩家持有攻击指令、目标相邻且回合到期时
- **THEN** a strike at the target is planned, the next-turn slot advances by one action's duration, and the order is gone | 策划一次对目标的出手，回合槽推进一个行动时长，指令消失

#### Scenario: Pursuit follows the target's current cell | 追击跟随目标当前格

- **WHEN** the player holds an attack order whose target is not adjacent and the player's turn comes due | 玩家持有攻击指令、目标不相邻且回合到期时
- **THEN** one step is planned along a route computed against the target's current cell, even if the target has moved since the click | 沿对目标当前格即时计算的路线策划一步，即使目标在点击之后移动过

#### Scenario: An unreachable target clears the order | 目标不可达清指令

- **WHEN** no route to the target's cell exists at planning time | 策划时不存在通往目标格的路线时
- **THEN** the order is cleared and the next-turn slot stays put | 指令清除，回合槽保持不动

#### Scenario: A gone target clears the order | 目标消失清指令

- **WHEN** the attack order's target is no longer in the world at planning time | 策划时攻击指令的目标已不在世界中时
- **THEN** the order is cleared and the next-turn slot stays put | 指令清除，回合槽保持不动

### Requirement: Player Hit Determination | 玩家命中判定

A strike SHALL judge the hit with the reference skeleton: draw a
percentile of 0..99 first — below 10, the strike hits if and only if
it is below 5; otherwise a chance at or below zero always misses;
otherwise draw a power roll of 0..chance-1 — the strike hits if and
only if the power roll is at least three quarters of the target's
armor class (integer division). The chance SHALL be the attacker's
attack chance (the melee skill term plus three times the attack
quality — the hit bonus and the weapon-side stand-in summed); the
armor class SHALL be the target kind's vocabulary value. The power
roll MUST NOT be drawn while the chance is non-positive.

出手 SHALL 按参照骨架判定命中：先抽 0..99 的 percentile——小于 10
时当且仅当小于 5 命中；否则 chance 不大于零必失；否则抽
0..chance-1 的威力骰——当且仅当威力骰不小于目标护甲的四分之三
（整数除法）时命中。chance SHALL 取攻击方的攻击值（命中技能项加
三倍攻击品质——命中修正与武器侧顶替值之和）；护甲 SHALL 取目标种
类的词表值。chance 非正时 MUST NOT 抽威力骰。

#### Scenario: The certain-hit band | 必中带

- **WHEN** a strike draws a percentile below 5 | 出手抽到小于 5 的 percentile 时
- **THEN** it hits, regardless of the chance and the target's armor class | 无论 chance 与目标护甲如何都命中

#### Scenario: The certain-miss band | 必失带

- **WHEN** a strike draws a percentile of 5 to 9 | 出手抽到 5 至 9 的 percentile 时
- **THEN** it misses, regardless of the chance and the target's armor class | 无论 chance 与目标护甲如何都判失

#### Scenario: A non-positive chance never hits | 非正 chance 必失

- **WHEN** the percentile is 10 or above and the chance is zero or negative | percentile 不小于 10 且 chance 为零或负时
- **THEN** the strike misses | 出手判失

#### Scenario: Power against three quarters of armor | 威力对四分之三护甲

- **WHEN** the percentile is 10 or above and the chance is positive | percentile 不小于 10 且 chance 为正时
- **THEN** the strike hits if and only if the power roll is at least three quarters of the target's armor class | 当且仅当威力骰不小于目标护甲的四分之三时命中

### Requirement: Unarmed Strike Damage | 徒手出手伤害

A hitting strike SHALL issue a damage request against the target for
the unarmed damage (one plus the attacker's damage bonus, floored at
zero); a missing strike SHALL issue nothing. The strike SHALL spend
the same base action duration as one step.

命中的出手 SHALL 对目标发出伤害请求，数值为徒手伤害（一加攻击方
伤害加成，下端夹零）；未命中 SHALL 什么都不发出。出手 SHALL 与移
动一步同基准行动时长。

#### Scenario: A hit hurts through the channel | 命中经通道致伤

- **WHEN** a strike hits a monster | 出手命中怪物时
- **THEN** the monster's current hit points drop by the unarmed damage amount, and death handling follows as the health capability defines | 怪物当前生命值按徒手伤害数值下降，死亡处理按 health 能力的规定跟进

#### Scenario: A miss issues nothing | 未命中无效果

- **WHEN** a strike misses | 出手未命中时
- **THEN** no damage request exists and the target's hit points are unchanged | 不产生任何伤害请求，目标生命值不变

## MODIFIED Requirements

### Requirement: Attack Chance | 攻击值

The attack chance SHALL equal the melee skill term plus three times
the attack quality — the hit bonus plus the weapon-side stand-in.
Until the skill system lands, the melee skill term is the fixed
stand-in 5 — the value the reference formula yields for the level-one
warrior's skill bases under its stepwise integer divisions. Until the
skill and equipment systems land, the attack-quality stand-in is the
fixed 1 — the to-hit a level-one warrior's weapon contributes in the
reference: the weaponmastery skill's to-hit (the skill level at level
one) plus the basic weapon's own to-hit. Both stand-ins delete when
their systems land.

攻击值 SHALL 等于命中技能项加三倍攻击品质——命中修正加武器侧顶替
值。技能系统落地前，命中技能项为固定顶替值 5——参考公式代入一级
战士的技能基值、按其逐步整数除法算得的结果；技能与装备系统落地
前，攻击品质顶替值为固定值 1——参考实现中一级战士所持武器贡献的
命中：武器掌握技能的命中项（一级时为技能等级）加基础武器自身的命
中。两个顶替值随各自系统落地删除。

#### Scenario: The chance is a fixed function of the hit bonus | 攻击值为命中修正的确定函数

- **WHEN** the hit bonus is given | 给定命中修正时
- **THEN** the attack chance equals 8 plus three times the hit bonus | 攻击值等于 8 加三倍命中修正

#### Scenario: A level-one warrior clears a rat's armor | 一级战士压过老鼠护甲

- **WHEN** a level-one warrior with zero hit bonus strikes a giant white rat (armor class 7) | 命中修正为零的一级战士对巨白鼠（护甲 7）出手时
- **THEN** the chance is 8, above three quarters of the rat's armor (5) — strikes outside the certain bands can land | 攻击值为 8，高过老鼠护甲的四分之三（5）——必中带之外也能命中
