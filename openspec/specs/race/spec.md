# race Specification | race 规格

## Purpose

The race vocabulary: the definition of playable races. Each entry
declares a race id, six statistic modifiers, and a hit die; loading
assigns runtime handles and keeps the entries; a creature declares its
race by carrying the handle, and vocabulary fields resolve into
attributes and hit points at birth.

race 词表：可被扮演的种族的定义。每个条目声明种族 id、六维修正与生命
骰，加载时分配运行时句柄并保留条目；生物以句柄声明种族，词表字段在出
生期解析进属性与生命。

## Requirements

### Requirement: Race Vocabulary Registry | race 词表注册表

Race kinds SHALL be defined by a core data file: each entry declares a
race id, statistic modifiers (exactly six), and a hit die (a positive
integer). Loading assigns each id a runtime handle in entry order and
keeps the entries for birth-time resolution. A handle is a transient
identifier within one load of the process and MUST NOT be assumed
stable across loads; cross-load scenarios such as saves MUST translate
through the "handle ↔ id" mapping. Code MUST NOT contain a race enum,
nor test a handle or an id for race identity. Adding a race MUST take
only a new entry in the vocabulary file — no code change.

race 的种类 SHALL 由 core 的数据文件定义：每个条目声明种族 id、六维修
正（恰好六项）与生命骰（正整数）。加载时按条目出现顺序为每个 id 分配
运行时句柄，并保留条目供出生期解析。句柄只是进程内一次加载中的临时标
识，MUST NOT 假定其跨加载稳定；存档等跨加载场景 MUST 经"句柄 ↔ id"
映射翻译。代码 MUST NOT 包含种族枚举，也不以句柄或 id 判断种族身份。
新增种族 MUST 只需在词表文件新增一个条目，不改动任何代码。

#### Scenario: Ids resolve to handles and entries | 按名换取句柄与条目

- **WHEN** the vocabulary file declares several race entries and loads | 词表文件声明若干种族条目并加载时
- **THEN** each race id resolves to a handle and its entry (statistic modifiers and hit die); the same id resolves to equal handles within one load, different ids to different ones | 每个种族 id 可换取一个句柄及其条目（六维修正与生命骰）；同一 id 在同一次加载内换取的句柄相等，不同 id 的句柄不同

#### Scenario: Adding a race changes no code | 新增种族不改代码

- **WHEN** a new entry is added to the vocabulary file for a new race | 在词表文件为一个新种族新增一个条目时
- **THEN** the race loads and can be referenced without any code change | 不改动任何代码即可加载该种族并供引用

### Requirement: Race Identity Component | race 身份组件

A humanoid creature SHALL carry a race handle to declare its race;
once attached, the race handle SHALL NOT change at runtime. Monsters
carry no race handle — a monster's identity is its kind in the monster
vocabulary.

人形 creature SHALL 携带 race 句柄以声明其种族；race 句柄挂载后 SHALL
NOT 在运行期变更。怪物不携带 race 句柄——怪物身份由怪物词表的 kind
承担。

#### Scenario: The player's birth race | 主角出生身份

- **WHEN** the player is born | 主角出生时
- **THEN** the player entity carries the race handle of its race | 主角实体携带其种族的 race 句柄

### Requirement: Race Data Validation | race 数据校验

The vocabulary file MUST be validated at load. A duplicate id, an
empty id, a modifier list other than six long, or a non-positive hit
die SHALL fail startup, the error naming the file and the offending
position.

词表文件 MUST 在加载时校验。id 重复、id 为空、六维修正长度非六、生命
骰非正，SHALL 导致启动失败，错误信息指明出错文件与出错位置。

#### Scenario: A modifier list other than six fails startup | 六维修正长度非六拒绝启动

- **WHEN** a race entry's statistic modifiers are not exactly six | 种族条目的六维修正不是恰好六项时
- **THEN** startup fails, the error naming the file and that race id | 启动失败，错误信息指明出错文件与该种族 id

#### Scenario: A non-positive hit die fails startup | 生命骰非正拒绝启动

- **WHEN** a race entry's hit die is zero or negative | 种族条目的生命骰为零或负时
- **THEN** startup fails, the error naming the file and that race id | 启动失败，错误信息指明出错文件与该种族 id

### Requirement: Current Race Content | 当前 race 内容

The race vocabulary SHALL declare human: statistic modifiers
[0, 0, 0, 0, 0, 0] and hit die 10. The race vocabulary SHALL NOT
declare unplayable monster families.

race 词表 SHALL 声明 human：六维修正为 [0, 0, 0, 0, 0, 0]、生命骰为
10。race 词表 SHALL NOT 声明不可被扮演的怪物家族。

#### Scenario: The human entry matches value for value | human 条目数值逐值一致

- **WHEN** the repository's race vocabulary is read | 读取仓库中的 race 词表时
- **THEN** the human entry's statistic modifiers are [0, 0, 0, 0, 0, 0] and its hit die is 10, and no monster family entries are present | human 条目的六维修正为 [0, 0, 0, 0, 0, 0]、生命骰为 10，且不包含怪物家族条目
