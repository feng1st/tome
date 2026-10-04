# creature-identity Specification Delta

## REMOVED Requirements

### Requirement: 身份词表注册表

**Reason**: race 与 class 拆分为独立能力，词表注册表与字段归各自 spec；unique 整套移除（现在没人用）。

**Migration**: 见 race 与 class 能力。

### Requirement: 身份数据校验

**Reason**: 词表校验归 race 与 class 各自的词表校验。

**Migration**: 见 race 与 class 能力。

### Requirement: 形象绑定表

**Reason**: 绑定键去掉 unique、新增 monster 形态；原 unique 场景随之移除。

**Migration**: 由本 delta 新增的同名 requirement 承接。

## ADDED Requirements

### Requirement: 形象绑定

creature 到形象的映射 SHALL 由 display 的数据文件定义：每个绑定条目以 monster、race+class、race 三种键形态之一声明身份，并指向一个形象 id。怪物按 monster 键解析；人形按 race+class → race 的顺序取最具体匹配，优先级为 monster > race+class > race。成员必带职业的种族可以只有组合键条目，无"裸"形象。每个声明的 race MUST 至少被一条条目（race 键或组合键）覆盖，每个声明的 monster MUST 至少被一条 monster 键条目覆盖；一个身份未命中任何条目时 SHALL 在挂载时以指明该身份的错误失败。代码 MUST NOT 包含身份到形象的映射。

#### Scenario: monster 键解析怪物

- **WHEN** 怪物实体携带 monster 句柄，且绑定表存在以该 monster id 为键的条目
- **THEN** 该实体呈现该 monster 条目指向的形象

#### Scenario: 组合键优先于 race 默认

- **WHEN** 人形 creature 携带 race 与 class 句柄，绑定表存在以该 (race, class) 为键的条目
- **THEN** 该实体呈现该组合条目指向的形象

#### Scenario: 回退到 race 默认

- **WHEN** 人形 creature 无 (race, class) 条目命中
- **THEN** 该实体呈现其 race 的 race 键条目指向的形象

#### Scenario: race 无任何绑定拒绝启动

- **WHEN** race 词表声明的某个 race 在绑定表中既无 race 键条目也无任何组合键条目
- **THEN** 启动失败，错误信息指明出错文件与该 race id

#### Scenario: monster 无任何绑定拒绝启动

- **WHEN** 怪物词表声明的某个 monster 在绑定表中无 monster 键条目
- **THEN** 启动失败，错误信息指明出错文件与该 monster id

#### Scenario: 身份未命中绑定挂载即失败

- **WHEN** creature 的身份在绑定表中无匹配（如无 class 的人形成员）
- **THEN** 挂载时以指明该身份的错误失败

## MODIFIED Requirements

### Requirement: 生物的身份组件

人形 creature SHALL 携带 race 句柄以声明其种族，class 句柄为可选组件；怪物不携带 race 或 class 句柄，其身份由怪物词表的 kind 承担。身份句柄组件挂载后 SHALL NOT 在运行期变更；形象绑定在身份组件挂载时一次解析，不做运行期重解析。附身、变形、吸血鬼化等改变身体的机制不属于本能力，将来单独立项。

#### Scenario: 玩家出生身份

- **WHEN** 游戏进入对局、主角出生
- **THEN** 主角实体携带 race 句柄与 class 句柄

#### Scenario: 怪物出生身份

- **WHEN** 怪物实体按地图出生表出生
- **THEN** 该实体携带怪物句柄，不携带 race 或 class 句柄

### Requirement: 当前身份与绑定内容

race 词表 SHALL 声明 human，class 词表 SHALL 声明 warrior，怪物词表 SHALL 声明 giant_white_rat；当前内容 SHALL NOT 声明任何 unique 个体。主角 SHALL 携带 race human 与 class warrior。绑定表 SHALL 包含：(human, warrior) 的组合键条目指向形象 warrior；monster giant_white_rat 的 monster 键条目指向形象 giant_white_rat。无 class 的 human 无形象可绑（无对应素材），绑定表 SHALL NOT 包含 human 的 race 键条目。

#### Scenario: 测试房间的形象呈现

- **WHEN** 启动游戏进入测试房间
- **THEN** 主角呈现 warrior 形象，两只老鼠呈现 giant_white_rat 形象
