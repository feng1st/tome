# monster Specification (delta)

## MODIFIED Requirements

### Requirement: 怪物词表

怪物的种类 SHALL 由 core 的数据文件定义：词表文件以词条列表声明全部怪物
id，每个条目 SHALL 声明该怪物的 speed（速率表索引原值）、hit_points
（骰式字符串）、armor_class（整数）、level（整数）与 blows（伤害骰列表，
每个元素含一个伤害骰式）。kind 即怪物的身份——不声明 race、class 或
unique id。加载时按条目出现顺序为每个 id 分配运行时句柄，并在加载期解析
每条的骰式。句柄只是进程内一次加载中的临时标识，MUST NOT 假定其跨加载
稳定；存档等跨加载场景 MUST 经"句柄 ↔ id"映射翻译。代码 MUST NOT 包含
怪物枚举，也不以句柄或 id 判断怪物身份。新增怪物 MUST 只需在词表文件、
形象绑定文件与形象表中各新增一个条目，不改动任何代码。

#### Scenario: 按名换取句柄

- **WHEN** 词表文件声明怪物 id 词条列表并加载
- **THEN** 每个 id 可换取一个句柄；同一 id 在同一次加载内换取的句柄相等，不同 id 的句柄不同

#### Scenario: 新增怪物不改代码

- **WHEN** 在词表文件、形象绑定文件与形象表中为一个新怪物各新增一个条目，并在地图文件声明其出生
- **THEN** 不改动任何代码即可加载该怪物并在游戏中呈现

#### Scenario: speed 缺失或越界拒绝启动

- **WHEN** 怪物条目缺失 speed 字段或 speed 超出速率表范围
- **THEN** 启动失败；缺失时错误信息指明出错文件与缺失字段，越界时进一步指明该怪物 id

#### Scenario: 战斗字段缺失或骰式非法拒绝启动

- **WHEN** 词表条目缺失任一战斗字段（hit_points、armor_class、level、blows），或其 hit_points、blows 的伤害骰不是合法骰式
- **THEN** 启动失败，错误信息指明出错文件、该怪物 id 与缺失字段或非法骰式

### Requirement: 怪物出生

进入游戏状态时 SHALL 按当前地图的出生表为每个出生条目生成一个怪物实体：
实体携带怪物句柄、出生格坐标、由条目 speed 解析的速度组件、回合槽组件与
生命值组件；生命值上限 SHALL 在出生期掷该条目的 hit_points 得出，当前值
初始等于上限。不携带 race、class 或 unique 句柄。呈现数据由前端按出生格
坐标在出生反应中挂载。出生条目的怪物 id 未在词表声明时 SHALL 导致启动
失败，错误信息指明该 id。

#### Scenario: 按地图声明出生

- **WHEN** 当前地图声明了若干怪物出生条目并进入游戏状态
- **THEN** 每个出生条目生成一个位于对应格子、携带怪物句柄、速度组件与回合槽的实体

#### Scenario: 未知怪物拒绝启动

- **WHEN** 当前地图的出生条目引用词表之外的怪物 id
- **THEN** 启动失败，错误信息指明该怪物 id

#### Scenario: 出生携带生命值

- **WHEN** 一个出生条目生成怪物实体
- **THEN** 实体携带生命值组件，当前值等于上限，且上限落在该条目生命骰的值域内（骰数到骰数×面数，含两端）

### Requirement: 巨白鼠数据

怪物词表 SHALL 声明条目 `( monster: "giant_white_rat", speed: 110,
hit_points: "2d2", armor_class: 7, level: 4, blows: [ ( damage: "1d3" ) ] )`；
其形象绑定与形象数据由 creature-identity 与 figure 能力承接。测试房间
SHALL 声明两只巨白鼠出生，格子分别为 (28,10) 与 (24,13)。

#### Scenario: 测试房间呈现两只老鼠

- **WHEN** 启动游戏进入测试房间
- **THEN** 格子 (28,10) 与 (24,13) 各呈现一只播放 idle 动画的老鼠，且二者起始帧错开

#### Scenario: 老鼠条目逐值

- **WHEN** 加载怪物词表
- **THEN** 巨白鼠条目的生命骰为 2d2、护甲 7、等级 4，且攻击列表只含一个 1d3 伤害骰
