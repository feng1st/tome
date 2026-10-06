历史归档，不符合先英文后中文规范，请勿参考

# monster Specification Delta

## REMOVED Requirements

### Requirement: 怪物词表注册表

**Reason**: 怪物条目减为 monster + speed，去掉 race/class/unique_id；原 race/class 校验场景随之移除。

**Migration**: 由本 delta 新增的同名 requirement 承接。

## ADDED Requirements

### Requirement: 怪物词表

怪物的种类 SHALL 由 core 的数据文件定义：词表文件以词条列表声明全部怪物 id，每个条目 SHALL 声明该怪物的 speed（速率表索引原值）。kind 即怪物的身份——不声明 race、class 或 unique id。加载时按条目出现顺序为每个 id 分配运行时句柄。句柄只是进程内一次加载中的临时标识，MUST NOT 假定其跨加载稳定；存档等跨加载场景 MUST 经"句柄 ↔ id"映射翻译。代码 MUST NOT 包含怪物枚举，也不以句柄或 id 判断怪物身份。新增怪物 MUST 只需在词表文件、形象绑定文件与形象表中各新增一个条目，不改动任何代码。

#### Scenario: 按名换取句柄

- **WHEN** 词表文件声明怪物 id 词条列表并加载
- **THEN** 每个 id 可换取一个句柄；同一 id 在同一次加载内换取的句柄相等，不同 id 的句柄不同

#### Scenario: 新增怪物不改代码

- **WHEN** 在词表文件、形象绑定文件与形象表中为一个新怪物各新增一个条目，并在地图文件声明其出生
- **THEN** 不改动任何代码即可加载该怪物并在游戏中呈现

#### Scenario: speed 缺失或越界拒绝启动

- **WHEN** 怪物条目缺失 speed 字段或 speed 超出速率表范围
- **THEN** 启动失败；缺失时错误信息指明出错文件与缺失字段，越界时进一步指明该怪物 id

## MODIFIED Requirements

### Requirement: 怪物出生

进入游戏状态时 SHALL 按当前地图的出生表为每个出生条目生成一个怪物实体：实体携带怪物句柄、出生格坐标、由条目 speed 解析的速度组件与回合槽组件；不携带 race、class 或 unique 句柄。呈现数据由前端按出生格坐标在出生反应中挂载。出生条目的怪物 id 未在词表声明时 SHALL 导致启动失败，错误信息指明该 id。

#### Scenario: 按地图声明出生

- **WHEN** 当前地图声明了若干怪物出生条目并进入游戏状态
- **THEN** 每个出生条目生成一个位于对应格子、携带怪物句柄、速度组件与回合槽的实体

#### Scenario: 未知怪物拒绝启动

- **WHEN** 当前地图的出生条目引用词表之外的怪物 id
- **THEN** 启动失败，错误信息指明该怪物 id

### Requirement: 怪物呈现与占位

怪物 SHALL 按其 kind 键绑定的形象呈现并播放 idle 动画。多只怪物同帧出生时，它们的 idle 播放起始帧 SHALL 由各自出生格坐标派生，而非全零同步。怪物 MUST NOT 阻碍移动：寻路与目标格判定只读地形，主角可移动到怪物所在格。

#### Scenario: 出生即呈现并播放 idle

- **WHEN** 怪物实体进入世界
- **THEN** 该实体呈现其 kind 键绑定形象的 idle 动画并持续循环播放

#### Scenario: 群体起始帧错开

- **WHEN** 两只出生格坐标之和不同的怪物同帧出生
- **THEN** 它们的 idle 播放起始帧不同

#### Scenario: 主角穿行怪物所在格

- **WHEN** 主角以怪物所在格为目标移动
- **THEN** 寻路与移动不受怪物影响，主角到达并停留于该格

### Requirement: 巨白鼠数据

怪物词表 SHALL 声明条目 `( monster: "giant_white_rat", speed: 110 )`；其形象绑定与形象数据由 creature-identity 与 figure 能力承接。测试房间 SHALL 声明两只巨白鼠出生，格子分别为 (28,10) 与 (24,13)。

#### Scenario: 测试房间呈现两只老鼠

- **WHEN** 启动游戏进入测试房间
- **THEN** 格子 (28,10) 与 (24,13) 各呈现一只播放 idle 动画的老鼠，且二者起始帧错开
