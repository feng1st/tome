# stats Specification

## Purpose

玩家向六维属性：可被扮演角色的六维（Strength/Intelligence/Wisdom/Dexterity/Constitution/Charisma）以出生掷点得出、种族与职业修正并入。六维以组件承载，是生命等派生数值的来源；怪物不携带六维。

## Requirements

### Requirement: 六维组件

可被扮演角色 SHALL 携带六维组件，记录六维的最大值与当前值；六维以 Strength/Intelligence/Wisdom/Dexterity/Constitution/Charisma 的固定顺序索引。本变更只初始化最大值与当前值；暂时吸取、含修正派生值等留待对应系统立项。

#### Scenario: 组件承载六维

- **WHEN** 读取一个可被扮演角色的六维组件
- **THEN** 可按六个固定维度各自取得最大值与当前值

### Requirement: 出生掷点

六维基准值 SHALL 由出生掷点得出：每个维度为 `5 + d3 + d4 + d5`（范围 8–17）；六维总和须落在 42–57，否则整组重掷。

#### Scenario: 掷点区间与总和

- **WHEN** 进行一次出生掷点
- **THEN** 每个维度的基准值落在 8–17，六维总和落在 42–57

### Requirement: 修正并入

种族与职业的六维修正相加后，SHALL 按分段非线性地并入对应维度的基准值：基准未满 18 时逐点增减，达到 18 之后按分段以更大步长增减（默认出生路径在 18 以上引入随机步长）；负修正先使数值回到 18 再逐点下探，任何数值 MUST NOT 低于 3。并入只朝修正方向移动：正修正的并入结果 MUST NOT 低于基准，负修正的并入结果 MUST NOT 高于基准。并入结果写入六维的最大值与当前值。

#### Scenario: 正修正不低于基准加修正

- **WHEN** 给定基准值与正的种族、职业修正
- **THEN** 并入结果不低于"基准加该修正"——每个并入点至少加一

#### Scenario: 负修正不高于基准且有下限

- **WHEN** 给定基准值与负的种族、职业修正
- **THEN** 并入结果不高于基准，且不低于 3

### Requirement: 玩家出生属性

主角出生时 SHALL 携带六维组件：六维基准由出生掷点得出，再并入其种族与职业的六维修正。

#### Scenario: 主角出生携带六维

- **WHEN** 主角出生
- **THEN** 主角实体携带六维组件，六维值落在掷点与修正可达的区间内
