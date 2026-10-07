# combat delta for monster-attacks | combat 增量：monster-attacks

## ADDED Requirements

### Requirement: Strike Resolution | 出手解析

Every planned strike SHALL resolve through one shared pipeline: for
each blow the attacker carries, judge the hit (see Hit Determination)
against the target; on a hit issue exactly one damage request for
that blow's damage — a fixed amount, or one roll of its dice; on a
miss issue nothing. The strike action
SHALL be consumed either way. Strikes planned by the player resolve
in the player's acting phase; strikes planned by a monster resolve in
the world's acting phase. Resolution MUST NOT read any vocabulary and
MUST NOT branch on what the attacker or the target is — every number
comes from the blows and armor-class components the creatures carry.

每一次策划出的出手 SHALL 经同一条共享管线解析：对攻击方携带的每
一击，判定其命中（见"命中判定"）；命中则恰好发出一条伤害请求，
数值为该击的伤害——定值，或其骰子的一次掷出；未命中则什么都不
发出。
出手动作 SHALL 照例消耗。玩家策划的出手在玩家执行阶段解析；怪物
策划的出手在世界执行阶段解析。解析 MUST NOT 读任何词表，也
MUST NOT 按攻击方或目标是谁而分支——一切数值取自生物携带的打
击与护甲组件。

#### Scenario: Every blow judges and rolls on its own | 每击独立判定与掷骰

- **WHEN** a strike resolves for an attacker carrying several blows | 攻击方携带多击的出手解析时
- **THEN** each blow judges the hit on its own chance and issues its own damage request on a hit | 每击按自身命中品质独立判定，命中各发各的伤害请求

#### Scenario: A target without armor takes no threshold | 无护甲目标无阈值

- **WHEN** a strike's target carries no armor-class component | 出手的目标不携带护甲组件时
- **THEN** three quarters of armor reads as zero, and a blow with a positive chance always lands past the certain bands | 护甲的四分之三按零读，命中品质为正的一击过带必中

#### Scenario: An attacker without blows strikes nothing | 无打击者出手为空

- **WHEN** a strike resolves for an attacker carrying no blows | 攻击方不携带打击的出手解析时
- **THEN** no damage request issues, and the strike action is spent as usual | 不产生任何伤害请求，出手动作照常消耗

## RENAMED Requirements

- FROM: `### Requirement: Player Hit Determination | 玩家命中判定`
  TO: `### Requirement: Hit Determination | 命中判定`

## MODIFIED Requirements

### Requirement: Hit Determination | 命中判定

A strike SHALL judge each blow's hit with the reference skeleton:
draw a percentile of 0..99 first — below 10, the blow hits if and
only if it is below 5; otherwise a chance at or below zero always
misses; otherwise draw a power roll of 0..chance-1 — the blow hits if
and only if the power roll is at least three quarters of the target's
armor class (integer division). The chance SHALL be the blow's own,
as the attacker carries it; the armor class SHALL be the target's
armor-class component, reading as zero for a target that carries
none. The power roll MUST NOT be drawn while the chance is
non-positive.

出手 SHALL 按参照骨架逐击判定命中：先抽 0..99 的 percentile——
小于 10 时当且仅当小于 5 命中；否则 chance 不大于零必失；否则
抽 0..chance-1 的威力骰——当且仅当威力骰不小于目标护甲的四分
之三（整数除法）时命中。chance SHALL 取攻击方携带的该击自身之
值；护甲 SHALL 取目标的护甲组件，目标不携带时按零读。chance 非
正时 MUST NOT 抽威力骰。

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

The player's blows SHALL derive exactly one blow — the unarmed
strike: its chance is the attack chance, its damage fixed at the
unarmed damage (one plus the damage bonus, floored at zero), and the
derivation SHALL renew the blow whenever the combat bonuses renew.
The strike SHALL spend the same base action duration as one step.

玩家的打击列表 SHALL 派生恰好一击——徒手一击：命中品质即攻击
值，伤害固定为徒手伤害（一加伤害加成，下端夹零）；战斗加成更新
时 SHALL 同步重派生该击。出手 SHALL 与移动一步同基准行动时长。

#### Scenario: A hit hurts through the channel | 命中经通道致伤

- **WHEN** a strike hits a monster | 出手命中怪物时
- **THEN** the monster's current hit points drop by the unarmed damage amount, and death handling follows as the health capability defines | 怪物当前生命值按徒手伤害数值下降，死亡处理按 health 能力的规定跟进

#### Scenario: A miss issues nothing | 未命中无效果

- **WHEN** a strike misses | 出手未命中时
- **THEN** no damage request exists and the target's hit points are unchanged | 不产生任何伤害请求，目标生命值不变

### Requirement: Player Armor Class | 玩家护甲

A player's armor class SHALL be the armor bonus; the
equipment-granted base is zero while the equipment system does not
exist. The armor class SHALL be carried as a component on the
creature, derived anew whenever the combat bonuses renew.

玩家护甲 SHALL 为护甲修正；装备系统不存在期间，装备给予的基数为
0。护甲 SHALL 以组件形式携带在生物身上，随战斗加成更新重派生。

#### Scenario: Armor class equals the armor bonus | 护甲等于护甲修正

- **WHEN** a player's armor class is read | 读取玩家护甲时
- **THEN** it equals the armor bonus derived from dexterity | 其值等于由敏捷派生的护甲修正
