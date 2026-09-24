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
- [ ] 4.3 全量回归 — verify: `cargo test` 全绿 + 运行游戏人工确认点击移动、动画、相机、水面与现状一致

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
