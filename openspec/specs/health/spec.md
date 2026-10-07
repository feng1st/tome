# health Specification | health 规格

## Purpose

Hit points: the life component shared by every woundable creature
(current value and maximum). A player's maximum derives from
constitution and the hit die at birth; a monster's maximum is rolled
from its vocabulary hit dice at spawn, independent of the six
statistics.

生命值：一切可受伤生物共享的生命组件（当前值与上限）。玩家生命值上限
由体质与生命骰在出生期派生；怪物生命值上限由其词表生命骰在出生期掷
出，不经六维。

## Requirements

### Requirement: Hit-Point Component | 生命值组件

A woundable creature SHALL carry a hit-point component recording the
current value and the maximum; the current value MUST NOT exceed the
maximum. The component is common to players and monsters.

可受伤生物 SHALL 携带生命值组件，记录当前值与上限；当前值 MUST NOT 超
过上限。该组件对玩家与怪物通用。

#### Scenario: The component carries current and maximum | 组件承载当前值与上限

- **WHEN** a woundable creature's hit-point component is read | 读取一个可受伤生物的生命值组件时
- **THEN** the current value and the maximum are available, and the current value does not exceed the maximum | 可取得当前值与上限，且当前值不超过上限

### Requirement: Player Hit-Point Derivation | 玩家生命值派生

A player's hit-point maximum SHALL derive at birth from their
constitution and their race's and class's hit dice: at level 1 the
maximum equals the hit die plus half the constitution bonus (integer
division); the hit die is the race's and class's shares summed, and
the constitution bonus is looked up in the 38-bracket bonus table by
the constitution's current value. The derivation runs once at birth
and the current value starts equal to the maximum; per-level hit
tables and recomputation on constitution change are deferred.

玩家的生命值上限 SHALL 在出生期由其体质与种族、职业生命骰派生：等级 1
时上限等于生命骰加上体质修正的一半（整数除法）；生命骰为种族与职业生
命骰之和，体质修正按体质的当前值查 38 项分段修正表得出。出生期一次算
好，当前值初始等于上限；逐级生命表与体质变动时的重算延后。

#### Scenario: The maximum is a fixed function of constitution and hit die | 上限为体质与生命骰的确定函数

- **WHEN** the player's constitution current value and the race and class hit dice are given | 给定玩家的体质当前值与种族、职业生命骰时
- **THEN** the hit-point maximum is determined, equal to the hit die plus half the constitution bonus (integer division) | 生命值上限是确定的，且等于生命骰加上体质修正的一半（整数除法）

#### Scenario: The current value starts at the maximum | 当前值初始等于上限

- **WHEN** the player is born | 主角出生时
- **THEN** the hit-point current value equals the maximum | 生命值当前值等于上限

### Requirement: Player Birth Hit Points | 玩家出生生命

At birth the player SHALL carry the hit-point component, its maximum
derived as in "Player Hit-Point Derivation".

主角出生时 SHALL 携带生命值组件，其上限按"玩家生命值派生"得出。

#### Scenario: The player is born with hit points | 主角出生携带生命值

- **WHEN** the player is born | 主角出生时
- **THEN** the player entity carries the hit-point component, its maximum the value derived from constitution and hit die | 主角实体携带生命值组件，上限为体质与生命骰派生值

### Requirement: Monster Hit-Point Derivation | 怪物生命值派生

A monster's hit-point maximum SHALL be rolled at spawn from its
vocabulary hit dice: the maximum is the sum of N draws of 1..=M (N and
M the entry's dice count and face count), independent of the six
statistics; the current value starts equal to the maximum.

怪物的生命值上限 SHALL 在出生期由其词表生命骰掷出：上限等于 N 个
1..=M 之和（N、M 为该条目生命骰的骰数与面数），不经六维；当前值初始
等于上限。

#### Scenario: The maximum is one roll of the hit dice | 上限为生命骰的一次掷出

- **WHEN** a vocabulary entry declares hit dice NdM and the monster spawns | 词表条目声明生命骰 NdM 且对应怪物出生时
- **THEN** its hit-point maximum lands between N and N×M inclusive, independent of the six statistics | 其生命值上限落在 N 到 N×M 之间（含两端），且与六维无关

#### Scenario: The current value starts at the maximum | 当前值初始等于上限

- **WHEN** a monster spawns | 怪物出生时
- **THEN** the hit-point current value equals the maximum | 生命值当前值等于上限

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

### Requirement: Death Threshold | 死亡界线

A creature SHALL be dead exactly when its current hit points are
strictly negative; a creature at exactly zero SHALL be alive. Death
handling SHALL run once the killing damage is applied, before the
side opposite the killer plans its next action.

生物 SHALL 当且仅当当前生命值为严格负值时死亡；恰好为零的生物
SHALL 为存活。死亡处理 SHALL 在致死伤害施加后、杀伤方对侧的下
一次策划前完成。

#### Scenario: Zero is alive | 零为存活

- **WHEN** damage leaves a creature at exactly zero hit points | 伤害使一个生物的生命值恰好为零时
- **THEN** the creature is not dead and no death handling runs | 该生物未死，不发生任何死亡处理

#### Scenario: Below zero is dead | 小于零为死亡

- **WHEN** damage leaves a creature at negative one hit points | 伤害使一个生物的生命值降为 -1 时
- **THEN** the creature is dead and its death handling runs before the opposite side plans again | 该生物死亡，其死亡处理在对侧再次策划前完成

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

### Requirement: Player Death | 玩家死亡

A dead player SHALL NOT be removed; it SHALL carry a death marker
instead. Commands addressed to the dead driver SHALL be ignored, and
the dead driver SHALL plan no further action; with the driver's due
turn never spent again, the clock SHALL hold at that turn and no
creature SHALL come due.

死亡的玩家 SHALL NOT 被移除，而 SHALL 携带死亡标记。发给死去驾
驶者的命令 SHALL 被无视；死去的驾驶者 SHALL 不再策划任何行动；
驾驶者到期的回合不再被消耗，时钟 SHALL 停在该回合，任何生物不
再到期。

#### Scenario: Death marks the player instead of removing it | 死亡为玩家加标记而非移除

- **WHEN** damage leaves the player at negative hit points | 伤害使玩家的生命值降为负时
- **THEN** the player entity still exists and carries the death marker | 玩家实体仍然存在，并携带死亡标记

#### Scenario: A player at zero is unmarked | 零血玩家无标记

- **WHEN** damage leaves the player at exactly zero hit points | 伤害使玩家的生命值恰好为零时
- **THEN** the player carries no death marker | 玩家不携带死亡标记

#### Scenario: Commands to the dead driver are ignored | 发给死去驾驶者的命令被无视

- **WHEN** a movement command arrives after the driver's death | 驾驶者死亡后有移动命令到达时
- **THEN** the command is ignored and no standing order forms | 该命令被无视，不形成任何站立指令

#### Scenario: The world freezes on the dead driver | 世界随死去的驾驶者冻结

- **WHEN** the driver is dead | 驾驶者死亡后
- **THEN** the driver's due turn is never spent, the clock does not advance past it, and no monster comes due | 驾驶者到期的回合不再被消耗，时钟不越过该回合推进，任何怪物不再到期

### Requirement: Damage Interrupts Standing Orders | 伤害打断站立指令

Applying damage to a creature SHALL clear the target's standing
order — whether it is a move order or an attack order; a creature that
takes no damage SHALL keep its order. The rule lives at the damage channel's
application point, so no damage source can bypass it.

对生物施加伤害 SHALL 清除该目标的站立指令——无论它是移动指令还
是攻击指令；未受伤害的生物 SHALL 保留其指令。该规则落在伤害通道的施加点，任
何伤害来源都无法绕过。

#### Scenario: A hit clears the target's orders | 命中清除目标指令

- **WHEN** damage is applied to a creature holding a standing order | 对持有站立指令的生物施加伤害时
- **THEN** the target's standing order is gone after the application | 施加之后目标的站立指令消失

#### Scenario: No damage, no interruption | 无伤害无打断

- **WHEN** an attack misses and no damage is applied | 攻击未命中、没有伤害被施加时
- **THEN** the target's standing order is unchanged | 目标的站立指令不变
