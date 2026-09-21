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
