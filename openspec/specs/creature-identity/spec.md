# creature-identity Specification

## Purpose

生物身份模型：玩家、NPC、怪物皆为 creature，身份由 race（必需）、class（可选）、unique ID（可选）三个词表组件表达，core 的词表文件声明 id 并分配运行时句柄；形象由 display 的绑定表按 unique → race+class → race 的顺序取最具体匹配解析。

## Requirements


### Requirement: 身份词表注册表

race、class 的词表 SHALL 各自由 core 的数据文件定义：词表文件以词条列表声明全部 id，加载时按条目出现顺序为每个 id 分配运行时句柄。unique ID SHALL 不设独立词表文件——个体即内容，由内容文件声明（怪物条目的 `unique_id` 字段；npc.ron 条目落地时同法）；一个文件 SHALL 允许多个解析者各读各的部分，加载时按来源顺序聚合注册。句柄只是进程内一次加载中的临时标识，MUST NOT 假定其跨加载稳定；存档等跨加载场景 MUST 经"句柄 ↔ id"映射翻译。代码 MUST NOT 包含身份枚举，也不以句柄或 id 判断身份。新增身份 MUST 只需在对应词表文件新增一个条目，不改动任何代码。

#### Scenario: 按名换取句柄

- **WHEN** 词表文件声明 id 词条列表并加载
- **THEN** 每个 id 可换取一个句柄；同一 id 在同一次加载内换取的句柄相等，不同 id 的句柄不同

#### Scenario: 新增身份不改代码

- **WHEN** 在某一词表文件为一个新身份新增一个条目
- **THEN** 不改动任何代码即可加载该身份并供引用

#### Scenario: 无 unique 内容即为空注册表

- **WHEN** 所有内容词表均未声明 unique id
- **THEN** unique 注册表为空且加载成功，没有任何 creature 携带 unique 句柄

#### Scenario: unique id 重复拒绝启动

- **WHEN** 两个内容条目声明了相同的 unique id
- **THEN** 启动失败，错误信息指明该 id

### Requirement: 生物的身份组件

每个 creature 实体 SHALL 携带 race 句柄以声明其种族；class 句柄与 unique 句柄 SHALL 为可选组件，class 对玩家与怪物均开放挂载。身份句柄组件挂载后 SHALL NOT 在运行期变更；形象绑定在身份组件挂载时一次解析，不做运行期重解析。附身、变形、吸血鬼化等改变身体的机制不属于本能力，将来单独立项。

#### Scenario: 玩家出生身份

- **WHEN** 游戏进入对局、主角出生
- **THEN** 主角实体携带 race 句柄与 class 句柄

#### Scenario: 怪物出生身份

- **WHEN** 怪物实体按地图出生表出生
- **THEN** 该实体携带怪物句柄，以及其怪物条目所声明 race 的句柄

### Requirement: 形象绑定表

creature 到形象的映射 SHALL 由 display 的数据文件定义：每个绑定条目以 unique、race+class、race 三种键形态之一声明身份，并指向一个形象 id。解析 SHALL 按 unique → race+class → race 的顺序取最具体匹配的条目，未命中更具体的键时落回较不具体的键。无 class 个体的种族（动物）SHALL 有 race 键默认条目；成员必带职业的种族（人形）可以只有组合键条目——无"裸"形象。每个声明的 race MUST 至少被一条条目（race 键或组合键）覆盖；一个身份三层均未命中时 SHALL 在挂载时以指明该身份的错误失败。代码 MUST NOT 包含身份到形象的映射。

#### Scenario: unique 键最优先

- **WHEN** creature 携带 unique 句柄，且绑定表存在以该 unique id 为键的条目
- **THEN** 该实体呈现该 unique 条目指向的形象

#### Scenario: 组合键优先于 race 默认

- **WHEN** creature 携带 race 与 class 句柄，绑定表无其 unique 条目、存在以该 (race, class) 为键的条目
- **THEN** 该实体呈现该组合条目指向的形象

#### Scenario: 落回 race 默认

- **WHEN** creature 无 unique 条目命中且无 (race, class) 条目命中
- **THEN** 该实体呈现其 race 的 race 键条目指向的形象

#### Scenario: race 无任何绑定拒绝启动

- **WHEN** race 词表声明的某个 race 在绑定表中既无 race 键条目也无任何组合键条目
- **THEN** 启动失败，错误信息指明出错文件与该 race id

#### Scenario: 三层均未命中的身份挂载即失败

- **WHEN** creature 的身份在绑定表中三层均无匹配（如无 class 的人形成员）
- **THEN** 挂载时以指明该身份的错误失败

### Requirement: 身份数据校验

词表文件与绑定文件 MUST 在加载时校验。词表中同一 id 重复出现、id 为空；绑定条目引用词表之外的 race、class 或 unique id、引用形象表之外的形象 id、同一键形态同一身份出现多条绑定条目，SHALL 导致启动失败，错误信息指明出错文件与出错位置。

#### Scenario: 绑定引用未知身份拒绝启动

- **WHEN** 绑定条目引用词表之外的 race、class 或 unique id
- **THEN** 启动失败，错误信息指明出错文件与该 id

#### Scenario: 绑定引用未知形象拒绝启动

- **WHEN** 绑定条目引用形象表之外的形象 id
- **THEN** 启动失败，错误信息指明出错文件与该形象 id

#### Scenario: 重复绑定键拒绝启动

- **WHEN** 绑定表中同一键形态下的同一身份出现多条条目
- **THEN** 启动失败，错误信息指明出错文件与该身份

### Requirement: 当前身份与绑定内容

race 词表 SHALL 声明 `human` 与 `giant_white_rat`；class 词表 SHALL 声明 `warrior`；当前内容 SHALL NOT 声明任何 unique 个体。主角 SHALL 携带 race `human` 与 class `warrior`。绑定表 SHALL 包含：(human, warrior) 的组合键条目指向形象 `warrior`；race `giant_white_rat` 的 race 键条目指向形象 `giant_white_rat`。无 class 的 human 无形象可绑（无对应素材），绑定表 SHALL NOT 包含 human 的 race 键条目。

#### Scenario: 测试房间的形象呈现

- **WHEN** 启动游戏进入测试房间
- **THEN** 主角呈现 warrior 形象，两只老鼠呈现 giant_white_rat 形象
