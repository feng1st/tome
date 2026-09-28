# Proposal: data-driven-appearance

## Why

生物外观数据（贴图、图集网格、动画帧表）目前硬编码在 display 侧的常量文件里，协议键是 core 的封闭枚举：新增一种外观要改动多个既有代码文件，违背开闭原则。上一轮 data-driven-local-map 确立了"词表注册表 + 运行时句柄 + 显示侧绑定"的数据化范式并把 appearance 显式留作后续；生物图鉴、护甲换肤、存档都建立在"外观是数据"这一前提上，现在是补上这一块地基的时机。

## What Changes

- core 外观词表数据化：`data/core/appearances.ron` 以字符串列表声明全部外观 id；core 新增词表注册表资源，按条目顺序分配 `AppearanceIndex(u16)` 运行时句柄。句柄即组件（生物以句柄声明自己穿戴的外观），`AppearanceKind` 枚举删除。沿用编号纪律：编号不是身份、不具备稳定性，代码不以编号或 id 判断外观身份；字符串 id 只出现在数据文件互相引用、加载解析、错误信息与将来的存档映射表。
- display 外观画法数据化：`data/graphic/looks.ron` 单表描述每个外观的贴图路径、帧像素尺寸、行列数与动画帧表（帧表按 `AnimKind` 的 serde 名给出帧序列与 fps）；加载期校验：每个声明的外观有且仅有一条图形条目、Idle 帧表必备、帧号不越出网格。
- display 的外观注册表改为从图形文件构建（`FromWorld` + `World::get_resource_or_init` 递归拉取 core 词表注册表，与其他注册表同一形态）；warrior 条目逐值复刻现有常量表（12×15 帧、21×8 网格、idle/run 帧序列与帧率），运行表现无可见变化。
- `AnimClip.frames` 由 `'static` 切片改为数据持有的定长帧表。
- 出生点代码以字符串 id `"warrior"` 向词表注册表换句柄——这是全代码库唯一一处以 id 指名外观的地方（出生内容是代码常量，随将来的生物数据化退场）。
- **BREAKING**（仅代码内协议，无外部消费方）：`AppearanceKind` 枚举及其全部引用点删除。
- 不在本次范围：`terrain_animation` 域配置化、生物数据化（bestiary）、护甲阶层换肤机制、存档、Bevy Asset 管线与热重载。

## Capabilities

### New Capabilities

- `appearance`: 生物外观的声明与解析——core 词表注册表（id 声明、句柄分配、重复校验）、display 图形注册表（逐 id 绑定贴图/网格/帧表、加载校验与缺失报错）、warrior 外观复刻现行为。

### Modified Capabilities

无。`hero-animation` 描述的是可观察行为（帧序列、帧率、翻转），本次逐值不变。

## Impact

- **代码**：`core/appearance`（枚举删除，新增词表注册表资源与 `AppearanceIndex` 句柄组件）；`core/hero`（出生点以 id 换句柄）；`frontend/display/appearance`（warrior 常量文件删除，注册表改从文件加载，新增 serde 条目类型）；`frontend/display/sprite_animation`（`AnimClip.frames` 类型随迁）；`animate`/`sync_animation`/`attach_appearance` 系统的查询类型随迁。
- **依赖**：无新增（`serde`、`ron` 已在）。
- **数据**：新增 `data/core/appearances.ron`、`data/graphic/looks.ron`。
- **行为**：无可见变化；warrior 的待机呼吸与奔跑循环逐值等同现状。
- **其他 spec**：`hero-animation`、`hero-movement`、`rendering`、`camera` 均不受影响。
