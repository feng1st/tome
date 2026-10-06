历史归档，不符合先英文后中文规范，请勿参考

## MODIFIED Requirements

### Requirement: 怪物词表注册表

怪物的种类 SHALL 由 core 的数据文件定义：词表文件以词条列表声明全部怪物 id，每个条目 SHALL 声明该怪物的 race id，可声明 class id 与 unique id（unique id 即个体的名字，聚合进 unique 注册表）。加载时按条目出现顺序为每个 id 分配运行时句柄。句柄只是进程内一次加载中的临时标识，MUST NOT 假定其跨加载稳定；存档等跨加载场景 MUST 经"句柄 ↔ id"映射翻译。代码 MUST NOT 包含怪物枚举，也不以句柄或 id 判断怪物身份。新增怪物 MUST 只需在词表文件、形象绑定文件与形象表中各新增一个条目，不改动任何代码。

#### Scenario: 按名换取句柄

- **WHEN** 词表文件声明怪物 id 词条列表并加载
- **THEN** 每个 id 可换取一个句柄；同一 id 在同一次加载内换取的句柄相等，不同 id 的句柄不同

#### Scenario: 新增怪物不改代码

- **WHEN** 在词表文件、形象绑定文件与形象表中为一个新怪物各新增一个条目，并在地图文件声明其出生
- **THEN** 不改动任何代码即可加载该怪物并在游戏中呈现

#### Scenario: race 引用未知拒绝启动

- **WHEN** 怪物条目引用身份词表之外的 race id
- **THEN** 启动失败，错误信息指明出错文件与该 race id

#### Scenario: class 引用未知拒绝启动

- **WHEN** 怪物条目引用身份词表之外的 class id
- **THEN** 启动失败，错误信息指明出错文件与该 class id

### Requirement: 怪物出生

进入游戏状态时 SHALL 按当前地图的出生表为每个出生条目生成一个怪物实体：实体携带怪物句柄、其怪物条目所声明 race 的句柄、格子位置；条目声明了 class 或 unique id 时，实体 SHALL 一并携带对应句柄。除此无其他游戏数据。出生条目的怪物 id 未在词表声明时 SHALL 导致启动失败，错误信息指明该 id。

#### Scenario: 按地图声明出生

- **WHEN** 当前地图声明了若干怪物出生条目并进入游戏状态
- **THEN** 每个出生条目生成一个位于对应格子、携带对应怪物句柄与 race 句柄的实体；条目的 class 与 unique id 声明随实体一并挂载

#### Scenario: 未知怪物拒绝启动

- **WHEN** 当前地图的出生条目引用词表之外的怪物 id
- **THEN** 启动失败，错误信息指明该怪物 id

### Requirement: 巨白鼠数据

怪物词表 SHALL 声明条目 `( monster: "giant_white_rat", race: "giant_white_rat" )`；其形象绑定与形象数据由 creature-identity 与 figure 能力承接。测试房间 SHALL 声明两只巨白鼠出生，格子分别为 (28,10) 与 (24,13)。

#### Scenario: 测试房间呈现两只老鼠

- **WHEN** 启动游戏进入测试房间
- **THEN** 格子 (28,10) 与 (24,13) 各呈现一只播放 idle 动画的老鼠，且二者相位错开

## REMOVED Requirements

### Requirement: 怪物数据校验

**Reason**: 绑定文件的校验随形象绑定移交 creature-identity（其"身份数据校验"需求承接）；词表自身的校验（id 重复、id 为空、race 引用未声明）并入"怪物词表注册表"需求。
**Migration**: 见 creature-identity spec 的"身份数据校验"需求与本能力修订后的"怪物词表注册表"需求。

### Requirement: 怪物形象绑定

**Reason**: 形象绑定泛化为 creature-identity 的优先级绑定链（unique → race+class → race），怪物经其 race 句柄参与绑定，不再有怪物专属的单层绑定。
**Migration**: 见 creature-identity spec 的"形象绑定表"需求；巨白鼠的绑定内容见"当前身份与绑定内容"需求。
