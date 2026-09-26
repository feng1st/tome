## 1. core 迁移

- [x] 1.1 `core/movement`：新建 `Position` 组件（连续格坐标，整数=格中心）— verify: `cargo check` 通过，组件带 doc comment
- [x] 1.2 `core/map`：GridMap 迁入并删除像素换算（`cell_center`/`world_to_cell`/`TILE_SIZE` 依赖）— verify: `cargo test` 中地图测试适配为格子语义后通过
- [x] 1.3 `core/movement`：`Path` 改格坐标，`follow_path` 推进 `Position` — verify: `cargo check` 通过
- [x] 1.4 `core/hero`：新增 `MoveTo` 事件与 `resolve_goal` 系统（walkable 校验+寻路回归内核）；spawn 只挂 `Hero`+`Position` — verify: `resolve_goal` 单测（不可通行目标不挂 `Path`）通过

## 2. graphic 迁移

- [x] 2.1 `graphic/map`：chunks、array_texture、terrain_anims 迁入，新增格↔像素换算 — verify: `cargo check` 通过
- [x] 2.2 `graphic/sync`：`sync_position` 系统（Position→Transform，保留 z，不取整）— verify: `cargo check` 通过
- [x] 2.3 `graphic/animation`、`graphic/camera`：迁入；`animate` 朝向改用格坐标差值 — verify: `cargo check` 通过
- [x] 2.4 `graphic/hero`：帧表 constants、`Added<Hero>` 补挂显示组件 — verify: `cargo check` 通过

## 3. input 迁移

- [x] 3.1 `input/click`：相机投影 → 发 `MoveTo` 事件 — verify: `cargo check` 通过

## 4. 装配与回归

- [x] 4.1 `main.rs` 装配与执行链（click → resolve_goal → follow_path → sync_position → animate → follow_target）— verify: `cargo check` 通过
- [x] 4.2 注释规范收尾：新/移文件 `//!` 与公开类型 `///` 齐全 — verify: 逐文件检查
- [x] 4.3 全量回归 — verify: `cargo test` 全绿 + 运行游戏人工确认点击移动、动画、相机、水面与现状一致

## 5. 协议与装配修订

- [x] 5.1 spec/design 修订（`PrimaryAction` 两层协议、SystemSet 标签归 core）— verify: `openspec validate` 通过
- [x] 5.2 `core::sets` 三个标签；core/graphic 内部链各自入集合；input 真插件化（`click.rs` → `mouse.rs`，`MoveTo` → `PrimaryAction`）；main.rs 只剩 `configure_sets` — verify: `cargo check` 通过
- [x] 5.3 回归 — verify: `cargo test` 全绿 + `cargo clippy` 零警告

## 6. frontend 合并与协议收尾

- [x] 6.1 spec/design 修订（两方结构、界面层可替换、`InputSet/GameSet/RenderSet`、Open Questions）— verify: `openspec validate` 通过
- [x] 6.2 代码合并：`graphic/`+`input/` → `frontend/{graphic,input}/`；`input_events.rs` → `gestures.rs`；集合改名 `GameSet`/`RenderSet`；main.rs 与文档注释同步 — verify: `cargo check` 通过
- [x] 6.3 回归 — verify: `cargo test` 全绿 + `cargo clippy` 零警告

## 7. 意图协议与手势词族

- [x] 7.1 产物同步（意图协议在内核、手势解析在界面层、`CellPos` 词汇类型）— verify: `openspec validate` 通过
- [x] 7.2 core：`CellPos` 类型与全量传播（GridMap/find_path/Path/step_duration/Position）、`events/intents.rs`（`MoveToCell`）、`resolve_move` — verify: `cargo test` 全绿
- [x] 7.3 frontend：`gestures.rs` 落位（`PrimaryActionOnCell` + 两个 `#[allow(dead_code)]` 范式标记）、`dispatch_gestures` 系统、`coords` 返回 `CellPos` — verify: `cargo check` 通过
- [x] 7.4 回归 — verify: `cargo clippy` 零警告 + 冒烟运行无异常

## 8. 词汇定稿：命令协议

- [x] 8.1 跨侧协议正名为 command：`core/hero/events/intents.rs` → `core/hero/commands/movement.rs`；`frontend/input/events/gestures.rs` → `frontend/input/gestures.rs` — verify: `cargo check` 通过
- [x] 8.2 产物措辞同步（意图→命令）— verify: `openspec validate` 通过
- [x] 8.3 三级动词定稿（translate/resolve/execute）；dispatch 拆为 `systems/resolve/` 一种手势一个 resolver — verify: `cargo check` 通过
- [x] 8.4 core 侧执行器改名定型：`resolve_move` → `systems/commands/move_to_cell.rs#execute`；`commands/movement.rs` → `commands/move_to_cell.rs`；消息注册下沉 `commands/mod.rs#register`；resolver 函数定名 `resolve` — verify: `cargo test` 全绿
- [x] 8.5 预防性目录拆分：`gestures.rs` → `gestures/`（一类型一文件，含两个范式标记）、`mouse.rs` → `mouse/left_click.rs` — verify: `cargo check` 通过、clippy 零警告
- [x] 8.6 输入翻译按设备定轴：`systems/devices/mouse.rs#translate`（单击/双击消歧是设备级状态逻辑，必须同处一个系统；bindings 表到来时只改内部查询） — verify: `cargo test` 全绿、冒烟正常
- [x] 8.7 注册下沉与子相位归位：devices/resolver 组各持 register，`InputPhase::{Translate, Resolve}` 归 frontend/input（core 只持顶层三相位） — verify: `cargo check` 通过、冒烟正常
- [x] 8.8 帧相位统一为枚举：`core/system_sets.rs`（三 struct）→ `core/frame_phase.rs`（`FramePhase::{Input, Game, Render}`），与 `InputPhase` 形状对齐 — verify: `cargo test` 全绿、冒烟正常
- [x] 8.9 词汇类型收尾：`CellPos` → `CellCoord`（具名字段 x/y）、`Position` 具名字段化、`types/` 侧面确立（`cell_coord.rs`/`grid_map.rs`）、`GridMap` 拆为纯数据 + `CurrentMap` 资源（`map()` 访问器保地图切换接缝） — verify: fmt/clippy/test 全绿、冒烟正常
- [x] 8.10 加载机制定型：资产 fire-and-forget——`load_appearances` 在 OnEnter 一次性构建注册表，渲染器等待像素（接受短暂 pop-in）；设计原则记入 config — verify: fmt/clippy/test 全绿、冒烟正常
- [x] 8.11 编排/注册分层：编排（链、门控、configure_sets）只在四个根部；成员注册（资源、消息、系统入集合）下沉到域/组——相位标签自带顺序时（`InputPhase`）组 register 只加一行 — verify: fmt/clippy/test 全绿、冒烟正常
- [x] 8.12 对齐引擎主调度：`FramePhase::Render` → `FramePhase::Display`（GPU 渲染在独立 SubApp，相位实为呈现准备）；设备翻译曾下沉 PreUpdate，因需推导的 `.after(InputSystems)` 约束而回退 Update + `InputPhase` 子相位（保守：结构保证优于推导约束） — verify: fmt/clippy/test 全绿、冒烟正常

## 9. 词汇与域定稿（整理期收尾）

- [x] 9.1 帧相位枚举定名 `GameLoop::{Input, Core, Display}`（原 `FramePhase`）；`CorePhase::{Sense, Plan, Act}` 三分——命令执行器入 Plan（定向：设置/修改 target），`follow_path` 入 Act（执行），Sense 空置待 AI — verify: `cargo check` 通过
- [x] 9.2 无状态帧动画：帧 = f(全局虚拟时间)，删 `AnimTimer`；`AnimKind` 枚举词汇超集；`AnimClips` 并入 `Appearance::clip()`（Idle 回退）；Sync（播放意图+朝向）/Animate（纯播放）相位分工 — verify: `cargo test` 全绿
- [x] 9.3 appearance 域：`AppearanceKind` 纯键（core）+ `Appearances` 注册表 + `load_appearances`（OnEnter）+ `attach_appearance`；新增 `DisplayPhase::Attach` 承载 `Added<>` 结构补挂 — verify: `cargo check` 通过
- [x] 9.4 域名定稿：display/sync → display/movement；animation → sprite_animation；map 拆出 terrain_animation（`TerrainAnim` 定义/`TerrainAnimState` 组件，`animate()`/`spawn_anim_layers`）；display/hero 取消（相机挂接归 camera `attach_target`） — verify: fmt/clippy/test 全绿
- [x] 9.5 demo room 缩为 48×32（视野 40×22.5 格）；水池与英雄出生点改从 MAP_W/MAP_H 派生；寻路/命令测试同步修复 — verify: `cargo test` 全绿
- [x] 9.6 期末收尾：注释逐文件对照实现刷新（含去除已不存在的 `DespawnOnExit` 生命周期描述）、纯逻辑单测补齐（build_chunk_data、follow_path、clip 回退、step_duration 等）、SDD 产物同步 — verify: `cargo test` 全绿 + `openspec validate` 通过
- [x] 9.7 手势词族收敛：world/local 地图手势并入统一的 `PrimaryActionOnCell`（地图归属留给 resolver 读游戏状态判断），`PrimaryActionOnMonster` 保留为词族范式标记 — verify: `cargo check` 通过、冒烟正常
- [x] 9.8 对齐收尾：注释与 SDD 产物对照实现刷新（加载机制、手势词族、相位命名、位置换算）；spec-locking 测试补齐（墙/越界目标、不可达目标、对角切角、左斜向翻转、水面 UV 滚动）并加强（绕池最短路径、移动中重寻路）— verify: fmt/clippy/test 全绿 + `openspec validate` 通过
