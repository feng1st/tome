# stats Specification | stats 规格

## Purpose

Player-facing six statistics: a playable character's
Strength/Intelligence/Wisdom/Dexterity/Constitution/Charisma come from
the birth roll, merged with race and class modifiers. Carried as a
component, the six are the source of derived values such as hit
points; monsters carry no statistics.

玩家向六维属性：可被扮演角色的六维（Strength/Intelligence/Wisdom/
Dexterity/Constitution/Charisma）以出生掷点得出、种族与职业修正并入。
六维以组件承载，是生命等派生数值的来源；怪物不携带六维。

## Requirements

### Requirement: The Six-Statistic Component | 六维组件

A playable character SHALL carry a six-statistic component recording
each statistic's ceiling and current value, indexed in the fixed order
Strength/Intelligence/Wisdom/Dexterity/Constitution/Charisma. The
component carries plain values only; temporary drains and
modifier-derived values belong to their own systems.

可被扮演角色 SHALL 携带六维组件，记录六维的最大值与当前值；六维以
Strength/Intelligence/Wisdom/Dexterity/Constitution/Charisma 的固定顺序
索引。组件只承载数值本身；暂时吸取、含修正派生值等机制归各自系统。

#### Scenario: The component carries the six | 组件承载六维

- **WHEN** a playable character's six-statistic component is read | 读取一个可被扮演角色的六维组件时
- **THEN** each of the six fixed dimensions yields a ceiling and a current value | 可按六个固定维度各自取得最大值与当前值

### Requirement: Birth Rolls | 出生掷点

The six base values SHALL come from the birth roll: each dimension is
`5 + d3 + d4 + d5` (8–17); the set is kept only while the six total is
strictly greater than 42 and strictly less than 57, otherwise the
whole set is rerolled.

六维基准值 SHALL 由出生掷点得出：每个维度为 `5 + d3 + d4 + d5`（范围
8–17）；六维总和须严格大于 42 且严格小于 57，否则整组重掷。

#### Scenario: Roll range and total | 掷点区间与总和

- **WHEN** a birth roll is made | 进行一次出生掷点时
- **THEN** every base value lands in 8–17 and the total lands strictly between 42 and 57, both ends excluded | 每个维度的基准值落在 8–17，六维总和严格落在 42 与 57 之间（不含两端）

### Requirement: Modifier Merging | 修正并入

The race and class modifiers are summed, then SHALL merge into the
base value of the matching dimension in stepped, nonlinear fashion:
below 18 every modifier point moves the value one for one; from 18 up
the merge moves in larger steps by bracket (random draws on the
default birth path). A negative modifier's points first drain the
value toward 18 — faster than one per point above it — then drain one
per point below it; no value MUST drop below 3. The merge moves only
along the modifier's direction: a positive modifier's result MUST NOT
be below the base, a negative one's result MUST NOT be above it. The
merged result is written into both the ceiling and the current value.

种族与职业的六维修正相加后，SHALL 按分段非线性地并入对应维度的基准
值：基准未满 18 时每点修正逐点增减；达到 18 之后按分段以更大步长增减
（默认出生路径在 18 以上引入随机步长）。负修正的每一点先把数值拉回
18——在 18 以上快于逐点——到 18 以下再逐点下探；任何数值 MUST NOT
低于 3。并入只朝修正方向移动：正修正的并入结果 MUST NOT 低于基准，负
修正的并入结果 MUST NOT 高于基准。并入结果写入六维的最大值与当前值。

#### Scenario: A positive modifier never lands below base plus modifier | 正修正不低于基准加修正

- **WHEN** a base value and positive race and class modifiers are given | 给定基准值与正的种族、职业修正时
- **THEN** the merged result is no less than base plus the modifier — every merged point adds at least one | 并入结果不低于"基准加该修正"——每个并入点至少加一

#### Scenario: A negative modifier never exceeds the base and has a floor | 负修正不高于基准且有下限

- **WHEN** a base value and negative race and class modifiers are given | 给定基准值与负的种族、职业修正时
- **THEN** the merged result is no higher than the base and no lower than 3 | 并入结果不高于基准，且不低于 3

### Requirement: Player Birth Statistics | 玩家出生属性

At birth the player SHALL carry the six-statistic component: the bases
come from the birth roll, then merge with the player's race and class
modifiers.

主角出生时 SHALL 携带六维组件：六维基准由出生掷点得出，再并入其种族
与职业的六维修正。

#### Scenario: The player is born with the six | 主角出生携带六维

- **WHEN** the player is born | 主角出生时
- **THEN** the player entity carries the six-statistic component, every value within the range the roll and the modifiers can reach | 主角实体携带六维组件，六维值落在掷点与修正可达的区间内
