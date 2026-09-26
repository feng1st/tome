## Why

当前 `hero`、`map` 域把游戏数据（Hero 标记、GridMap）、显示数据（贴图、动画帧表、Sprite）、输入处理（鼠标点击）混在同一模块内，显示和输入都无法单独替换。后续要支持更换贴图/动画/HUD、以及手机触控等替代输入方案，需要先确立内核与可替换插件的边界。

## What Changes

- 源码重组为两方：`core/`（游戏数据与通用逻辑，普通模块，不定义 Plugin 结构体）、`frontend/`（可整体替换的界面层插件，内含 `display/` 世界画面、`input/` 输入，未来 `ui/`（菜单、HUD）；界面层内部以目录为替换单元）。
- 现有域按侧拆分迁入：
  - `map` → `core/map`（GridMap、TileKind、寻路）+ `frontend/display/map`（静态瓦片渲染、岸线自动拼合、像素换算）+ `frontend/display/terrain_animation`（地形动画层）
  - `hero` → `core/hero`（标记组件、生成、命令执行）；外观由 `frontend/display/appearance`（形象注册表 + 补挂）承担，相机跟随挂接归 `frontend/display/camera`（不设独立 display/hero 域）
  - `movement` → `core/movement`；动画 → `frontend/display/sprite_animation`（生物帧表动画）；`camera` → `frontend/display/camera`；点击处理 → `frontend/input/`
- 内核定义显示无关的权威位置组件：连续格坐标（f32，1 单位 = 1 格），停留点恒为整数格（格中心），非整数值只在移动过程中出现——平滑移动的唯一来源。图形界面将其换算为像素坐标，文字界面取整到格。格址的词汇类型为 `CellCoord`（具名整数字段；`Cell` 一名保留给未来的实体概念；与连续位置 `Position` 词根相异）。
- 输入协议分两层：模态将原始输入解析为按目标分型的**手势**（`PrimaryActionOnCell` 等，frontend 内部）；策略解析依据游戏状态将手势译为具体**命令**（`MoveToCell` 等，跨侧协议，内核持有并校验执行）。
- `core/movement` 推进该位置，不再直接写 `Transform`；`GridMap` 移除像素换算（`cell_center`/`world_to_cell`、`TILE_SIZE` 依赖）。
- display 侧新增 位置→Transform 同步（`frontend/display/movement`）：格中心坐标换算像素（含半格偏移与 Y 翻转）；朝向判断（`sprite_animation` 的 `sync_animation`）改用格坐标位移。
- **内核不引用界面层的具体类型**，替换实现只需改 `main.rs` 的装配；装配层只编排 `GameLoop` 集合标签，不引用具体系统函数。
- 游戏行为不变：渲染分层、动画帧序列、移动手感、点击寻路均与现状一致。

## Capabilities

### New Capabilities

- `plugin-architecture`：定义 core / frontend 两方边界与可替换性要求——内核不依赖界面层具体类型、界面层单向依赖内核、内核位置协议为连续格坐标、命令协议由内核持有、手势解析归属界面层、装配层只编排集合标签、界面层实现可经装配层整体替换。

### Modified Capabilities

（无。现有 5 个 spec 的 requirement 均为行为级描述，本 change 不改变任何行为。）

## Impact

- `src/` 全面重组：新增 `core/`、`frontend/` 顶层目录；`map/`、`hero/` 按侧拆分，`movement/`、`animation/`、`camera/` 整体迁入 `frontend/display/`，`hero/systems/click.rs` 迁入 `frontend/input/`。
- `core/map` 不再含像素概念；`frontend/display/` 新增位置同步（`movement`）、形象注册表（`appearance`）与双动画域（`sprite_animation`/`terrain_animation`）；`frontend/input/` 持有手势与 resolver。
- `src/main.rs`：装配改为 core register + frontend register（display 与 input 各持 register）。
- 素材与许可：本 change 不新增素材；`warrior.png`、`tiles0.png` 等源自 pixel-dungeon（GPLv3），路径可能随重组微调，许可义务不变（项目发布须兼容 GPLv3）。
