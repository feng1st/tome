# Proposal: creature-identity

## Why

形象是显示关切，却因主角在代码常量中命名形象而滞留 core（OPEN_ISSUES #1）；生物身份目前只有"怪物种类"一根轴，撑不起既定方向（任务 NPC、unique 怪物、职业）。ToME2 考证给出答案：玩家 = race + class，怪物 = race（+ ego 模板），形象归属由 display 数据按身份解析。本 change 落定统一生物身份模型——一切生物皆 creature，身份由 race、class、unique ID 三个词表组件表达——以最小职业机制落地（词表 + 组件 + 绑定，不含选职业 UI），顺势完成 #1 的形象归位。

## What Changes

- **core 新增 creature 身份层**：race（必需）、class（可选，怪物也可携带）、unique ID（可选，任务 NPC 与 unique 怪物携带）三个词表句柄组件。race/class 由词表数据文件声明；unique id 不设独立词表文件——个体即内容，由内容文件声明（怪物条目的 `unique_id` 字段，npc.ron 落地时同法）；一个文件允许多个解析者各读各的部分，UniqueRegistry 以探针布局自构建。沿用既有"词表 → 注册表 → 句柄组件"模式（句柄按条目顺序分配、跨加载不稳定、文件间以字符串 id 引用）。
- **display 的形象绑定升级为模式匹配表**：绑定条目以 unique、race+class、race 三种键形态声明身份，解析按 unique → race+class → race 取最具体匹配；race 默认只对有裸个体的种族必需，人形种族允许只有组合键条目（无裸形象素材不强造），每个 race 至少被一条目覆盖，三层全未命中的身份挂载即失败。可选键结构为未来的 ego、亚种、子职业等修饰层留缝——新维度是条目上的新可选键，不是格式重构。**BREAKING**：怪物绑定文件被 creature 绑定文件取代。
- **形象词表归位 display，并与外观表合并**：figure 与 appearance 的分裂只因词表被困 core 而存在；词表迁入 display 后两表 1:1 闭合、分裂即成仪式。合并为一个域、一张数据文件（每个条目 = id + 贴图 + 图集 + 帧表，形如 `[ ( figure: "xxx", ..., anims: [ ( anim: ... ) ] ) ]`）、一个注册表（id→句柄与句柄→外观一体）；跨文件闭合校验这类错误在结构上不再可能。appearance 一词保留在外观数据值类型与挂载系统上（Appearance、attach_appearance），域与注册表统一为 figure。关闭 OPEN_ISSUES #1。**BREAKING**：词表与外观两张数据文件合并为一，内部归属与格式变更。
- **帧表词干统一为 anim**：AnimClip 更名 Anim、AnimClipEntry 更名 AnimEntry（含参数与变量名），数据文件字段 `clips` 更名 `anims`；sprite_animation 与 figure 两个域同步更名。
- **主角出生不再命名形象**：HERO_FIGURE 常量删除；主角携带 race + class，形象经绑定链解析，画面表现不变（warrior 形象数据原样保留）。
- **怪物词表保留并扩展**：怪物种类仍是独立的玩法词表——怪物不止身份，还是数值与行为数据的生长点（HP、速度、攻击方式等沿此生长）。条目新增 `race` 字段声明种族（如 `( monster: "giant_white_rat", race: "giant_white_rat" )`）；MonsterIndex 保留，怪物实体携带怪物句柄与 race 句柄。地图出生条目保持 `monster` 键，格式不变。
- **Race 不可变**：附身、变形、吸血鬼化等身体改造机制本期不做，将来单独立项设计；身份在挂载时一次解析，不做运行期重解析。

明确不做：开局选职业 UI（本期主角的 race/class 由出生代码以常量指定，与现状一致——core 代码可命名玩法 id，不可命名形象）；ego、亚种、子职业等修饰层；运行期身份变化；NPC 域（UniqueId 组件与绑定键先把缝留好）。

## Capabilities

### New Capabilities

- `creature-identity`: 生物身份模型——creature 的统一概念（玩家、NPC、怪物皆为 creature）；race、class、unique ID 三个词表的注册与运行时句柄；身份到形象的优先级绑定链（unique → race+class → race，取最具体匹配，race 层全覆盖强制、其余层可选）与加载期校验。
- `figure`: 形象的声明与呈现——形象词表与外观数据的合并表（每条目声明形象 id 及其贴图、图集、帧表），运行时句柄的分配与解析，生物按句柄挂载外观，加载期单表校验。

### Modified Capabilities

- `appearance`: capability 退役，需求经修订后整体迁入 `figure`（形象词表与外观表合并为一张 display 数据表；主角出生形象改经绑定链解析达成，不再由代码指定）。
- `monster`: 怪物词表条目扩展 `race` 字段并校验其引用；怪物出生实体加挂 race 句柄；形象绑定移交 creature-identity 绑定链的 race 层；巨白鼠内容与测试房间出生保持。

## Impact

- **新增代码**：core/creature 身份域（race/class/unique 三个词表注册表资源、三个句柄组件、词表解析校验，单一域按侧面分目录）；display 身份绑定（creature 绑定注册表、模式匹配解析、泛化的 attach_figure 系统）；display/figure 合并域（合并的形象注册表、形象表解析校验，attach_appearance 保留原名）。
- **修改代码**：core/hero 出生（挂 race+class，删 HERO_FIGURE）；core/monster 条目类型加 `race` 字段、出生系统加挂 race 句柄、加载校验扩展（race 引用必须已声明）；core/figure 域与 display/appearance 域整体撤销、并入 display/figure（Appearance 值类型与 attach_appearance 保留原名）；sprite_animation 域的 clip 词干全面更名 anim，引用指向合并注册表；core 与 display 两个侧根的 register 相应调整。core/map 与地图文件格式不动。
- **数据**：新增 data/core/races.ron、data/core/classes.ron；data/core/monsters.ron 条目加 `race` 字段（`class`、`unique_id` 可选）；data/core/figures.ron 与 data/graphic/appearances.ron 合并为 data/graphic/figures.ron；data/graphic/monster_figures.ron 由 data/graphic/creature_figures.ron 取代；data/maps/test_room.ron 不变。可选 id 字段经 RON `implicit_some` 扩展解析，无自定义 serde 代码。
- **文档**：新增 creature-identity 与 figure 两份 spec；appearance spec 退役（需求迁入 figure）；修订 monster spec；grid-map spec 不动；落地后删除 OPEN_ISSUES #1 条目。
- **依赖**：不新增 crate 依赖；Bevy 钉住 0.19.x 不变。
- **兼容性**：原型阶段无存档格式与对外 API，BREAKING 项均为内部结构与数据格式。
