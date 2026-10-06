# health Specification (delta) | health 规格（增量）

## ADDED Requirements

### Requirement: Damage Channel | 伤害通道

A creature SHALL be hurt only through a damage request naming the
target creature and a non-negative amount; the request SHALL reduce
the target's current hit points by that amount. The reduction SHALL
NOT be clamped at zero: the current value may go negative.

生物受到伤害 SHALL 只经由一条伤害请求，请求指明目标生物与非负数
额；请求 SHALL 从目标的当前生命值中减去该数额。扣减 SHALL NOT
在下端夹零：当前值可为负。

#### Scenario: Damage reduces the current value | 伤害扣减当前值

- **WHEN** a creature at 12 of 19 hit points receives a damage request for 5 | 一个生命值 12/19 的生物收到数额 5 的伤害请求时
- **THEN** its current hit points become 7 | 其当前生命值变为 7

#### Scenario: The reduction is not clamped at zero | 扣减下端不夹零

- **WHEN** a creature at 3 hit points receives a damage request for 5 | 一个生命值 3 的生物收到数额 5 的伤害请求时
- **THEN** its current hit points become -2 | 其当前生命值变为 -2

#### Scenario: The ceiling invariant survives damage | 上限不变量在伤害后保持

- **WHEN** any damage request is applied to any creature | 任意伤害请求施加于任意生物时
- **THEN** the creature's current hit points do not exceed the ceiling | 该生物的当前生命值不超过上限

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

A dead monster SHALL be removed from the world: its entity ceases to
exist, presentation included.

死亡的怪物 SHALL 被移出世界：其实体不复存在，表现随之一并消失。

#### Scenario: A slain monster leaves the world | 被杀的怪物移出世界

- **WHEN** damage leaves a monster at negative hit points | 伤害使一只怪物的生命值降为负时
- **THEN** the monster's entity no longer exists | 该怪物的实体不复存在

#### Scenario: A monster at zero stays | 零血怪物存留

- **WHEN** damage leaves a monster at exactly zero hit points | 伤害使一只怪物的生命值恰好为零时
- **THEN** the monster's entity still exists | 该怪物的实体仍然存在

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
- **THEN** the command is ignored and no path is queued | 该命令被无视，不排入任何路径

#### Scenario: The world freezes on the dead driver | 世界随死去的驾驶者冻结

- **WHEN** the driver is dead | 驾驶者死亡后
- **THEN** the driver's due turn is never spent, the clock does not advance past it, and no monster comes due | 驾驶者到期的回合不再被消耗，时钟不越过该回合推进，任何怪物不再到期
