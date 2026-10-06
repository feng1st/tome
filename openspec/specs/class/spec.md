# class Specification | class 规格

## Purpose

The class vocabulary: the definition of classes. Each entry declares a
class id, six statistic modifiers, and a hit die; loading assigns
runtime handles and keeps the entries; a creature declares its class
by carrying the handle, and vocabulary fields resolve into attributes
and hit points at birth.

class 词表：职业的定义。每个条目声明职业 id、六维修正与生命骰，加载时
分配运行时句柄并保留条目；生物以句柄声明职业，词表字段在出生期解析进
属性与生命。

## Requirements

### Requirement: Class Vocabulary Registry | class 词表注册表

Class kinds SHALL be defined by a core data file: each entry declares
a class id, statistic modifiers (exactly six), and a hit die (a
positive integer). Loading assigns each id a runtime handle in entry
order and keeps the entries for birth-time resolution. A handle is a
transient identifier within one load of the process and MUST NOT be
assumed stable across loads; cross-load scenarios such as saves MUST
translate through the "handle ↔ id" mapping. Code MUST NOT contain a
class enum, nor test a handle or an id for class identity. Adding a
class MUST take only a new entry in the vocabulary file — no code
change.

class 的种类 SHALL 由 core 的数据文件定义：每个条目声明职业 id、六维修
正（恰好六项）与生命骰（正整数）。加载时按条目出现顺序为每个 id 分配
运行时句柄，并保留条目供出生期解析。句柄只是进程内一次加载中的临时标
识，MUST NOT 假定其跨加载稳定；存档等跨加载场景 MUST 经"句柄 ↔ id"
映射翻译。代码 MUST NOT 包含职业枚举，也不以句柄或 id 判断职业身份。
新增职业 MUST 只需在词表文件新增一个条目，不改动任何代码。

#### Scenario: Ids resolve to handles and entries | 按名换取句柄与条目

- **WHEN** the vocabulary file declares several class entries and loads | 词表文件声明若干职业条目并加载时
- **THEN** each class id resolves to a handle and its entry (statistic modifiers and hit die); the same id resolves to equal handles within one load, different ids to different ones | 每个职业 id 可换取一个句柄及其条目（六维修正与生命骰）；同一 id 在同一次加载内换取的句柄相等，不同 id 的句柄不同

#### Scenario: Adding a class changes no code | 新增职业不改代码

- **WHEN** a new entry is added to the vocabulary file for a new class | 在词表文件为一个新职业新增一个条目时
- **THEN** the class loads and can be referenced without any code change | 不改动任何代码即可加载该职业并供引用

### Requirement: Class Identity Component | class 身份组件

A creature with a class SHALL carry a class handle to declare it; the
class component is optional, and once attached SHALL NOT change at
runtime.

携带职业的 creature SHALL 携带 class 句柄以声明其职业；class 为可选组
件，挂载后 SHALL NOT 在运行期变更。

#### Scenario: The player's birth class | 主角出生职业

- **WHEN** the player is born | 主角出生时
- **THEN** the player entity carries the class handle of its class | 主角实体携带其职业的 class 句柄

### Requirement: Class Data Validation | class 数据校验

The vocabulary file MUST be validated at load. A duplicate id, an
empty id, a modifier list other than six long, or a non-positive hit
die SHALL fail startup, the error naming the file and the offending
position.

词表文件 MUST 在加载时校验。id 重复、id 为空、六维修正长度非六、生命
骰非正，SHALL 导致启动失败，错误信息指明出错文件与出错位置。

#### Scenario: A modifier list other than six fails startup | 六维修正长度非六拒绝启动

- **WHEN** a class entry's statistic modifiers are not exactly six | 职业条目的六维修正不是恰好六项时
- **THEN** startup fails, the error naming the file and that class id | 启动失败，错误信息指明出错文件与该职业 id

#### Scenario: A non-positive hit die fails startup | 生命骰非正拒绝启动

- **WHEN** a class entry's hit die is zero or negative | 职业条目的生命骰为零或负时
- **THEN** startup fails, the error naming the file and that class id | 启动失败，错误信息指明出错文件与该职业 id

### Requirement: Current Class Content | 当前 class 内容

The class vocabulary SHALL declare warrior: statistic modifiers
[5, -2, -2, 2, 2, -1] and hit die 9.

class 词表 SHALL 声明 warrior：六维修正为 [5, -2, -2, 2, 2, -1]、生命
骰为 9。

#### Scenario: The warrior entry matches value for value | warrior 条目数值逐值一致

- **WHEN** the repository's class vocabulary is read | 读取仓库中的 class 词表时
- **THEN** the warrior entry's statistic modifiers are [5, -2, -2, 2, 2, -1] and its hit die is 9 | warrior 条目的六维修正为 [5, -2, -2, 2, 2, -1]、生命骰为 9
