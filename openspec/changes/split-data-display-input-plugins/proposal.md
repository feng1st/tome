## Why

当前 `hero`、`map` 域把游戏数据（Hero 标记、GridMap）、显示数据（贴图、动画帧表、Sprite）、输入处理（鼠标点击）混在同一模块内，显示和输入都无法单独替换。后续要支持更换贴图/动画/HUD、以及手机触控等替代输入方案，需要先确立内核与可替换插件的边界。

## What Changes

- 源码重组为三个顶层目录：`core/`（游戏数据与通用逻辑，普通模块，不定义 Plugin 结构体）、`graphic/`（可替换显示插件）、`input/`（可替换输入插件）。
- 现有域按侧拆分迁入：
  - `map` → `core/map`（GridMap、TileKind、寻路）+ `graphic/map`（chunk 渲染、贴图重排、地形动画）
  - `hero` → `core/hero`（标记组件与生成逻辑）+ `graphic/hero`（精灵、动画帧表）
  - `movement` → `core/movement`；`animation`、`camera` → `graphic/`；点击处理 → `input/`
- 内核定义显示无关的权威位置组件：连续格坐标（f32，1 单位 = 1 格），停留点恒为整数格（格中心），非整数值只在移动过程中出现——平滑移动的唯一来源。graphic 显示将其换算并取整到像素，文字类显示取整到格。
- `core/movement` 推进该位置，不再直接写 `Transform`；`GridMap` 移除像素换算（`cell_center`/`world_to_cell`、`TILE_SIZE` 依赖）。
- graphic 侧新增 位置→Transform 同步：格坐标 ×TILE_SIZE、Y 翻转、像素取整；`animate` 的朝向判断改用格坐标位移。
- 内核持有协议（组件与数据类型约定），graphic、input 插件单向依赖内核；**内核不引用插件的具体类型**，替换实现只需改 `main.rs` 的装配。
- 游戏行为不变：渲染分层、动画帧序列、移动手感、点击寻路均与现状一致。

## Capabilities

### New Capabilities

- `plugin-architecture`：定义 core / graphic / input 三方边界与可替换性要求——内核不依赖插件具体类型、插件单向依赖内核、内核位置协议为连续格坐标、显示与输入实现可经装配层替换。

### Modified Capabilities

（无。现有 5 个 spec 的 requirement 均为行为级描述，本 change 不改变任何行为。）

## Impact

- `src/` 全面重组：新增 `core/`、`graphic/`、`input/` 顶层目录；`map/`、`hero/` 按侧拆分，`movement/`、`animation/`、`camera/` 整体迁入，`hero/systems/click.rs` 迁入 `input/`。
- `core/map` 不再含像素概念；`graphic/` 新增位置同步系统。
- `src/main.rs`：装配改为 内核各域 register + graphic 插件 + input 插件。
- 素材与许可：本 change 不新增素材；`warrior.png`、`tiles0.png` 等源自 pixel-dungeon（GPLv3），路径可能随重组微调，许可义务不变（项目发布须兼容 GPLv3）。
