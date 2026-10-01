# Tasks: creature-identity

## 1. core/creature 身份域

- [x] 1.1 新建 src/core/creature/ 域：race/class/unique 三个词表的注册表资源（resources/）、句柄组件（components/ RaceIndex/ClassIndex/UniqueIndex）、条目类型（types/），沿用"词表文件 → FromWorld 拉取构建 → u16 句柄"模式；词表文件含解析与校验（id 重复、id 为空）。verify：`cargo test` 通过三个词表的合成文档单测（按名换取句柄、重复/空 id panic）。
- [x] 1.2 新增数据文件 data/core/races.ron（`[ ( race: "human" ), ( race: "giant_white_rat" ) ]`）、data/core/classes.ron（`[ ( class: "warrior" ) ]`）、data/core/uniques.ron（空表 `[]`）；core/mod.rs register 挂载 creature 域。verify：`cargo check --all-targets` 通过，且含"uniques 空表可加载"单测。

## 2. core/monster 与 core/hero 接入身份

- [x] 2.1 monster 域条目类型加 `race` 字段；加载校验扩展：条目引用未声明的 race id 时 panic（指明文件与 id）；data/core/monsters.ron 更新为 `[ ( monster: "giant_white_rat", race: "giant_white_rat" ) ]`。verify：`cargo test` 通过 race 引用校验的合成文档单测。
- [x] 2.2 monster 出生系统加挂 race 句柄（按条目 race 解析 RaceIndex，与 MonsterIndex、Position 一同插入）。verify：`cargo test` 通过出生组件构成的单测。
- [x] 2.3 hero 出生改写：删除 HERO_FIGURE 常量与 FigureRegistry 依赖，挂 RaceIndex(human) + ClassIndex(warrior) + Position。verify：`cargo check --all-targets` 通过，hero 域不再触碰形象概念（`rg -i "figure" src/core/hero` 无匹配；"warrior" 仅作为 class 玩法 id 出现）。

## 3. display/figure 合并域

- [x] 3.1 core/figure 域迁入 display/figure，与 appearance 域合并：FigureRegistry 一体提供 id→句柄与句柄→外观；Appearance、attach_appearance 保留原名移入；figures.ron 与 appearances.ron 合并为 data/graphic/figures.ron（条目 = figure id + texture + frame_size + columns/rows + anims）。verify：`cargo test` 通过形象表单测（解析、Idle 回退、缺 Idle/帧号越界/帧率非正 panic）。
- [x] 3.2 clip 词干全面更名 anim：AnimClip→Anim、AnimClipEntry→AnimEntry、anim_clip.rs→anim.rs、字段 clips→anims，含参数与变量名，横跨 figure 与 sprite_animation 两域。verify：`rg -i "clip" src/ data/` 无匹配；`cargo test` 全绿。
- [x] 3.3 撤销 display/appearance 域与 core/figure 域；display/mod.rs 与 core/mod.rs register 调整。verify：`cargo clippy --all-targets` 零警告。

## 4. display/creature 绑定域

- [x] 4.1 新建绑定注册表：data/graphic/creature_figures.ron 以可选字段表达三种键形态（unique / race+class / race）；加载期校验（键形态合法、引用已声明 id、同键不重复、race 键全覆盖）。内容：rat 默认→rat，human 默认→warrior，(human, warrior)→warrior。verify：`cargo test` 通过绑定表合成文档单测（三层解析、落回、各校验分支）。
- [x] 4.2 attach_figure 泛化：响应 `Added<RaceIndex>`，查询 (RaceIndex, Option<ClassIndex>, Option<UniqueIndex>)，按 unique → race+class → race 解析并插入 FigureIndex；撤销 display/monster 域。verify：`cargo test` 通过优先级解析单测；`cargo clippy --all-targets` 零警告。

## 5. spec 对齐与收尾

- [x] 5.1 spec 对齐测试：真实数据文件断言——races/classes/uniques 词表内容、monsters.ron 条目、creature_figures.ron 三条绑定、figures.ron 的 warrior 与 giant_white_rat 条目逐值一致、test_room.ron 两只老鼠出生格。verify：`cargo test` 通过全部对齐测试。
- [x] 5.2 删除 OPEN_ISSUES.txt 的 #1 条目。verify：文件中不再存在 figure 词表归属条目。
- [x] 5.3 全量门槛：`cargo +nightly fmt --check`、`cargo clippy --all-targets`、`cargo check --all-targets`、`cargo test` 全绿；启动游戏可见主角与两只相位错开的老鼠（画面与现状一致）。

## 6. 修订：unique 来源聚合化（无独立词表文件）

- [x] 6.1 MonsterEntry 加 `class` 与 `unique_id` 可选字段；MonsterRegistry 加载期解析 class（引用未声明 panic）、存储 unique id 原文并按条目序暴露迭代。verify：`cargo test` 通过 class/unique_id 解析与校验单测。
- [x] 6.2 UniqueRegistry 改为聚合注册：`empty()` + `extend(ids)`，由 core 侧根在 monster 域注册后聚合（单向依赖：内容域 → 装配层 → 身份注册表）；删除 uniques.ron 与 UniqueEntry。verify：`cargo test` 通过聚合/查重单测与"无个体声明"对齐测试。
- [x] 6.3 出生系统按条目挂载可选句柄（class/unique）；可选 id 字段经 RON `implicit_some` 扩展解析（裸字符串，无 Some 包装、无自定义 serde 代码）。verify：`cargo test` 通过出生构成单测（老鼠无 class/unique，合成 unique 怪物全件携带）；全量门槛全绿。

## 7. 修订：race 默认非强制（无裸人形形象素材）

- [x] 7.1 绑定表删除 human 的 race 键默认条目；校验规则改为"每个 race 至少被 race 键或组合键条目覆盖"；`figure()` 返回 Option，三层全未命中为 None。verify：`cargo test` 通过"组合键独占的 race 可达""无绑定 race 拒绝启动""裸人形解析为 None"单测与真实文件对齐测试。
- [x] 7.2 attach_figure 对未命中身份 panic（指明身份件）；spec/design/proposal 同步修订。verify：全量门槛全绿，openspec strict 校验通过。

## 8. 修订：UniqueRegistry 多解析者自构建 + MonsterRegistry 行式存储

- [x] 8.1 UniqueRegistry 自构建：探针布局直读来源文件（UNIQUE_SOURCE_PATHS，今天为 monsters.ron），不经其他注册表转手；creature 域恢复 init_resource 自注册，core 根聚合行删除。verify：`cargo test` 通过聚合/查重单测、双解析一致性测试（探针与全条目对真实文件断言一致）与启动冒烟。
- [x] 8.2 MonsterRegistry 转行式存储：Vec<Monster> + by_id，MonsterIndex 加 pub(crate) `index()` 访问器；列式 HashMap 与 unique_id_list 删除，`unique_ids()` 迭代器随聚合移除。verify：`cargo test` 全绿，`cargo clippy --all-targets` 零警告。
- [x] 8.3 FigureRegistry 同样转行式：Vec<Appearance> + by_id，FigureIndex 加 `index()` 访问器，appearances HashMap 删除。verify：`cargo test` 全绿，零警告。
- [x] 8.4 UniqueIdProbe 更名 UniqueIdEntry 并迁出注册表文件：落位 `core/creature/types/unique_id_entry.rs`（`*Entry` 惯例，serde 布局归 types/）。verify：`cargo test` 全绿，零警告。

## 9. 修订：参数命名与出生构成测试

- [x] 9.1 参数命名对齐规则：`cell_to_world(pos)` → `position`；`CurrentMap::new(map)` 与测试辅助 walkable 的 `map`/`registry`/`cell` → `local_map`/`terrain_registry`/`cell_coord`。verify：全量门槛全绿。
- [x] 9.2 spawn_hero 构成测试：断言主角携带 Hero 标记、race/class 句柄与出生格位置。verify：`cargo test` 通过（120）。
- [x] 9.3 `Monster` 更名 `MonsterKind`（ECS 里"怪物"是实体，行是种类的数据），访问器 `monster()` → `monster_kind()`；config 设计原则补"值入组件"的豁免半句（展示侧共享只读播放表除外）。verify：全量门槛全绿。
- [x] 9.4 config.yaml 零上下文可读性修复：值入组件的例子不再指名未来类型；"侧根"首次出现处给路径、"装配层"锚定 main()；SDD 展开一次；归属判定与文件即契约各补一个工作示例；参数命名补豁免（同类型对称参数、引擎原语语义名）；"播放表"改"播放数据"。verify：openspec 解析正常，strict 校验通过。

## 10. 修订：终审补强

- [x] 10.1 注释覆盖补全：animate/sync_animation 两系统补 fn 级 doc；AnimQuery/AnimSyncQuery/IdentityQuery 三个 QueryData 类型补 ///；LAYER_GROUND/LAYER_OVERHEAD 补 ///。verify：注释扫描零缺口。
- [x] 10.2 spec 场景补齐：creature 绑定注册表补"unique 键重复拒绝启动"测试；attach_appearance 补系统级挂载测试（精灵/图层/播放状态）。verify：`cargo test` 通过（122），全量门槛全绿。
- [x] 10.3 MonsterKind 变量名消歧：测试里行变量 `rat`/`grip` → `rat_kind`/`grip_kind`（句柄变量不动），同名异类消除。verify：全量门槛全绿。
- [x] 10.4 注册表字段名对齐元素类型：`MonsterRegistry.monsters` → `monster_kinds`、`FigureRegistry.figures` → `appearances`（字段名 = 元素类型复数）。verify：全量门槛全绿。
