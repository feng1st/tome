# race Specification

## Purpose

race 词表：可被扮演的种族的定义。每个条目声明种族 id、六维修正与生命骰，加载时分配运行时句柄并保留条目；生物以句柄声明种族，词表字段在出生期解析进属性与生命。

## ADDED Requirements

### Requirement: race 词表注册表

race 的种类 SHALL 由 core 的数据文件定义：每个条目声明种族 id、六维修正（恰好六项）与生命骰（正整数）。加载时按条目出现顺序为每个 id 分配运行时句柄，并保留条目供出生期解析。句柄只是进程内一次加载中的临时标识，MUST NOT 假定其跨加载稳定；存档等跨加载场景 MUST 经"句柄 ↔ id"映射翻译。代码 MUST NOT 包含种族枚举，也不以句柄或 id 判断种族身份。新增种族 MUST 只需在词表文件新增一个条目，不改动任何代码。

#### Scenario: 按名换取句柄与条目

- **WHEN** 词表文件声明若干种族条目并加载
- **THEN** 每个种族 id 可换取一个句柄及其条目（六维修正与生命骰）；同一 id 在同一次加载内换取的句柄相等，不同 id 的句柄不同

#### Scenario: 新增种族不改代码

- **WHEN** 在词表文件为一个新种族新增一个条目
- **THEN** 不改动任何代码即可加载该种族并供引用

### Requirement: race 身份组件

人形 creature SHALL 携带 race 句柄以声明其种族；race 句柄挂载后 SHALL NOT 在运行期变更。怪物不携带 race 句柄——怪物身份由怪物词表的 kind 承担。

#### Scenario: 主角出生身份

- **WHEN** 主角出生
- **THEN** 主角实体携带其种族的 race 句柄

### Requirement: race 数据校验

词表文件 MUST 在加载时校验。id 重复、id 为空、六维修正长度非六、生命骰非正，SHALL 导致启动失败，错误信息指明出错文件与出错位置。

#### Scenario: 六维修正长度非六拒绝启动

- **WHEN** 种族条目的六维修正不是恰好六项
- **THEN** 启动失败，错误信息指明出错文件与该种族 id

#### Scenario: 生命骰非正拒绝启动

- **WHEN** 种族条目的生命骰为零或负
- **THEN** 启动失败，错误信息指明出错文件与该种族 id

### Requirement: 当前 race 内容

race 词表 SHALL 声明 human：六维修正为 [0, 0, 0, 0, 0, 0]、生命骰为 10。race 词表 SHALL NOT 声明不可被扮演的怪物家族。

#### Scenario: human 条目数值逐值一致

- **WHEN** 读取仓库中的 race 词表
- **THEN** human 条目的六维修正为 [0, 0, 0, 0, 0, 0]、生命骰为 10，且不包含怪物家族条目
