# Proposal: mvp-room-hero-movement

## Why

`new-tome2` 是一个基于 Bevy 引擎（Rust）重实现的 roguelike 项目，复用 Pixel Dungeon 的素材。项目从零开始，需要一个可运行的最小骨架来验证核心技术路线（2D 渲染、精灵动画、格子地图、寻路移动、滚屏相机、动态背景），为后续游戏系统（战斗、物品、关卡生成）打底。

## What Changes

- 初始化 Bevy 0.19 桌面工程（`new-tome2/`），引入 pixel-dungeon 素材（`warrior.png`、`tiles0.png`、`water0.png`），像素风无损缩放渲染
- 实现有边界的格子房间（大于窗口），地块带可通行属性：外墙不可通行；房内水池不可通行（偏离原版：原版水可通行，MVP 简化为障碍以便演示寻路绕行）
- 实现动态背景：水面覆盖层持续滚动 + 透明度脉动（还原原版 `SkinnedBlock` 水动画）
- 实现主角：待机/奔跑帧动画，鼠标点击目标格 → A\* 寻路 → 沿 8 方向格子路径逐格移动
- 实现相机跟随主角，主角始终居中（滚屏）

## Capabilities

### New Capabilities

- `rendering`: 像素风 2D 渲染基础——tile 地图渲染（背景地板层 / 前景墙层 z 分层）、精灵表渲染、水面滚动动画层、窗口配置
- `grid-map`: 格子地图数据模型——tile 坐标系、可通行属性、房间边界与障碍物（水池）
- `hero-movement`: 主角移动——鼠标点击选目标格、A\* 寻路、8 方向逐格移动
- `hero-animation`: 主角动画——待机/奔跑帧动画，随移动状态切换
- `camera`: 相机——跟随主角，主角始终居中（滚屏）

### Modified Capabilities

（无——项目首个 change）

## Impact

- 新增工程全部代码：`new-tome2/Cargo.toml`、`src/main.rs` 及域目录 `map/`（含 `water/` 子域）、`hero/`、`camera/`、`movement/`、`animation/`
- 复制素材到 `new-tome2/assets/`（`warrior.png`、`tiles0.png`、`water0.png`，源自 pixel-dungeon，GPLv3——项目需以 GPLv3 兼容方式发布）
- 依赖：Bevy 0.19（crates.io），本地 `bevy` checkout 仅作 examples 参考
