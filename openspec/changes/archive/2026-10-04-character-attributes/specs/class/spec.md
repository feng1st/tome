历史归档，不符合先英文后中文规范，请勿参考

# class Specification

## Purpose

class 词表：职业的定义。每个条目声明职业 id、六维修正与生命骰，加载时分配运行时句柄并保留条目；生物以句柄声明职业，词表字段在出生期解析进属性与生命。

## ADDED Requirements

### Requirement: class 词表注册表

class 的种类 SHALL 由 core 的数据文件定义：每个条目声明职业 id、六维修正（恰好六项）与生命骰（正整数）。加载时按条目出现顺序为每个 id 分配运行时句柄，并保留条目供出生期解析。句柄只是进程内一次加载中的临时标识，MUST NOT 假定其跨加载稳定；存档等跨加载场景 MUST 经"句柄 ↔ id"映射翻译。代码 MUST NOT 包含职业枚举，也不以句柄或 id 判断职业身份。新增职业 MUST 只需在词表文件新增一个条目，不改动任何代码。

#### Scenario: 按名换取句柄与条目

- **WHEN** 词表文件声明若干职业条目并加载
- **THEN** 每个职业 id 可换取一个句柄及其条目（六维修正与生命骰）；同一 id 在同一次加载内换取的句柄相等，不同 id 的句柄不同

#### Scenario: 新增职业不改代码

- **WHEN** 在词表文件为一个新职业新增一个条目
- **THEN** 不改动任何代码即可加载该职业并供引用

### Requirement: class 身份组件

携带职业的 creature SHALL 携带 class 句柄以声明其职业；class 为可选组件，挂载后 SHALL NOT 在运行期变更。

#### Scenario: 主角出生职业

- **WHEN** 主角出生
- **THEN** 主角实体携带其职业的 class 句柄

### Requirement: class 数据校验

词表文件 MUST 在加载时校验。id 重复、id 为空、六维修正长度非六、生命骰非正，SHALL 导致启动失败，错误信息指明出错文件与出错位置。

#### Scenario: 六维修正长度非六拒绝启动

- **WHEN** 职业条目的六维修正不是恰好六项
- **THEN** 启动失败，错误信息指明出错文件与该职业 id

#### Scenario: 生命骰非正拒绝启动

- **WHEN** 职业条目的生命骰为零或负
- **THEN** 启动失败，错误信息指明出错文件与该职业 id

### Requirement: 当前 class 内容

class 词表 SHALL 声明 warrior：六维修正为 [5, -2, -2, 2, 2, -1]、生命骰为 9。

#### Scenario: warrior 条目数值逐值一致

- **WHEN** 读取仓库中的 class 词表
- **THEN** warrior 条目的六维修正为 [5, -2, -2, 2, 2, -1]、生命骰为 9
