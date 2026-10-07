# health delta for combat-feedback | health 增量：combat-feedback

## MODIFIED Requirements

### Requirement: Damage Channel | 伤害通道

A creature SHALL be hurt only through a damage request naming the
target creature, a non-negative amount, and optionally the source
creature; the request SHALL reduce the target's current hit points by
that amount. The reduction SHALL NOT be clamped at zero: the current
value may go negative. The dead are not woundable: a request naming a
creature already carrying the death marker SHALL drop, without a fact
and without change.

Each applied request SHALL emit a damage-applied fact naming the
target, the target's cell, the source's cell when the source still
exists (none otherwise), the amount, and the target's maximum. The
fact is a past-tense broadcast: it MUST NOT alter application, and any
number of readers MAY consume it.

生物受到伤害 SHALL 只经由一条伤害请求，请求指明目标生物、非负数
额，并可指明来源生物；请求 SHALL 从目标的当前生命值中减去该数
额。扣减 SHALL NOT 在下端夹零：当前值可为负。死者不可再受伤害：
指名已携带死亡标记生物的请求 SHALL 落空——无事实、无变化。

每条被施加的请求 SHALL 发出一条伤害落地事实，指明目标、目标所
在格、来源所在格（来源已不在时为空）、数额与目标上限。事实是
过去时广播：MUST NOT 改变施加本身，任意数量的读者 MAY 消费
它。

#### Scenario: Damage reduces the current value | 伤害扣减当前值

- **WHEN** a creature at 12 of 19 hit points receives a damage request for 5 | 一个生命值 12/19 的生物收到数额 5 的伤害请求时
- **THEN** its current hit points become 7 | 其当前生命值变为 7

#### Scenario: The reduction is not clamped at zero | 扣减下端不夹零

- **WHEN** a creature at 3 hit points receives a damage request for 5 | 一个生命值 3 的生物收到数额 5 的伤害请求时
- **THEN** its current hit points become -2 | 其当前生命值变为 -2

#### Scenario: The maximum invariant survives damage | 上限不变量在伤害后保持

- **WHEN** any damage request is applied to any creature | 任意伤害请求施加于任意生物时
- **THEN** the creature's current hit points do not exceed the ceiling | 该生物的当前生命值不超过上限

#### Scenario: An applied request emits its fact | 被施加的请求发出事实

- **WHEN** a damage request for 5 from a source creature is applied to a target at 12 of 19 hit points | 一条数额 5、带来源生物的伤害请求施加于生命值 12/19 的目标时
- **THEN** a damage-applied fact names the target, the target's cell, the source's cell, the amount 5, and the ceiling 19 | 一条伤害落地事实指明目标、目标格、来源格、数额 5 与上限 19

#### Scenario: A vanished source reads as none | 来源消失读作无

- **WHEN** a damage request is applied while its source creature no longer exists | 一条伤害请求在其来源生物已不存在时被施加时
- **THEN** the damage-applied fact's source cell is empty | 伤害落地事实的来源格为空

#### Scenario: A request onto the dead drops | 指名死者的请求落空

- **WHEN** a damage request names a creature already carrying the death marker | 一条伤害请求指名的生物已携带死亡标记时
- **THEN** the request drops — no reduction, no fact | 请求落空——不扣减、不发事实

### Requirement: Monster Death | 怪物死亡

A dead monster SHALL carry the death marker. Its body stays as
inert remains: the death presentation claims it (animation and fade)
and the body SHALL leave the world when the presentation releases it;
with no presentation claiming it, the body SHALL leave on the next
departure sweep. Death itself SHALL strip nothing here — the clock
drops dead turns, the planners skip the marked, and the departure
sweep owns the exit.

死亡的怪物 SHALL 携带死亡标记。其躯体作为惰性遗存留下：死亡呈现
认领它（动画与淡出），呈现释放后躯体 SHALL 离开世界；无人认领
时，躯体 SHALL 在下一次离场清扫中离开。死亡本身在此不剥除任何
东西——时钟抛开死亡回合，策划跳过被标记者，离场清扫拥有离场。

#### Scenario: Death marks the body | 死亡为躯体加标记

- **WHEN** damage leaves a monster at negative hit points | 伤害使一只怪物的生命值降为负时
- **THEN** the monster carries the death marker, and its body still stands in the world | 该怪物携带死亡标记，其躯体仍留在世界中

#### Scenario: A slain monster leaves the world | 被杀的怪物移出世界

- **WHEN** a slain monster's death presentation ends its fade — or none ever claimed it | 被杀怪物的死亡呈现淡出完毕——或从未被认领时
- **THEN** the body leaves the world on the next departure sweep | 躯体在下一次离场清扫中离开世界

#### Scenario: A monster at zero stays | 零血怪物存留

- **WHEN** damage leaves a monster at exactly zero hit points | 伤害使一只怪物的生命值恰好为零时
- **THEN** the monster's entity still exists and carries no death marker | 该怪物的实体仍然存在，且不携带死亡标记
