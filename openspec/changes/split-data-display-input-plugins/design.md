## Context

现状五个域（`map`/`hero`/`movement`/`animation`/`camera`）中，`hero` 与 `map` 把游戏数据、显示数据、输入处理混在同一模块；主角的权威位置只有 `Transform`（世界像素），移动系统直接补间像素；点击系统在自己内部做可通行校验与寻路。动机见 proposal.md - Why。

约束：Bevy 0.19（钉住）；插件三层分类原则（内核是普通模块，可整体替换的实现抽成插件）；域组织规范（侧面子目录、register 约定、归属判定）。

## Goals / Non-Goals

**Goals:**

- core / graphic / input 三方代码边界；内核不引用插件具体类型（编译期可验证）。
- 内核以连续格坐标 `Position` 为权威位置；像素换算、Y 翻转、z 层全部归入 graphic。
- 行为与现状逐帧一致：渲染分层、动画帧、移动手感、点击寻路不变。

**Non-Goals:**

- 不交付第二套显示或输入实现；MUD 文字显示仅作为边界设计的检验参照。
- 不改任何游戏行为、数值与素材。
- 地图数据文件化（后续 change 处理）。

## Decisions

### Decision 1: 三方目录与依赖方向

```mermaid
graph TD
    input[input/ 输入插件] --> graphic[graphic/ 显示插件]
    input --> core
    graphic --> core[core/ 内核]
    core --> nothing[不依赖 graphic / input]
```

```
src/
├── core/
│   ├── map/        GridMap、TileKind、pathfinding（纯格子语义，无像素概念）
│   ├── hero/       Hero 标记、生成、目标格寻路
│   └── movement/   Position、Path、follow_path、step_duration
├── graphic/
│   ├── map/        chunk 渲染、贴图重排、地形动画、像素换算
│   ├── hero/       精灵、动画帧表（对 Added<Hero> 补挂显示组件）
│   ├── animation/  AnimState/AnimClips/AnimTimer、animate
│   ├── camera/     MainCamera、CameraTarget、follow_target
│   └── sync/       Position → Transform 同步
└── input/
    └── systems/    mouse.rs：鼠标主操作 → PrimaryAction 手势
```

`map`、`hero` 横跨两侧，同名共存，靠顶层目录区分。`input` 依赖 `graphic`（相机投影与像素换算，见 Decision 4 的取舍说明）。内核各域沿用裸函数 `register` 约定；graphic、input 各有一个总 register 调用子域 register。

### Decision 2: 权威位置为连续格坐标

`core/movement/components/position.rs` 定义 `Position(Vec2)`：1 单位 = 1 格，x = 列、y = 行（向下为正，与地图数组一致）。整数坐标即格中心，`position.round()` 即所在格；非整数坐标只在移动过程中出现，是平滑移动的唯一来源。

- `follow_path` 推进 `Position`，不再触碰 `Transform`；`Path` 的 `step_from`/`step_target` 改为格坐标。每步时长模型不变（直走/斜走时长不同、直线速度恒定，还原原版手感——参考 pixel-dungeon `HeroSprite.java` 的移动节奏）。
- `GridMap` 删除像素换算（`cell_center`/`world_to_cell`），只保留格子语义 API（`get`/`walkable`）；格坐标 ↔ 像素的换算函数归入 `graphic/map/utils/`。
- `graphic/sync/systems/sync_position.rs` 每帧把 `Position` 写入 `Transform`：`x = pos.x × TILE_SIZE`，`y = -pos.y × TILE_SIZE`，保留既有 z。不额外取整——连续像素补间与现状逐帧一致（原版 PD 的移动同样是连续像素而非格跳）。
- `animate` 的朝向判断改用格坐标：`Path.step_target` 与 `Position` 的差值符号决定 `flip_x`（参考 bevy `examples/2d/sprite_animation.rs` 的帧动画组织，翻转逻辑现状已有）。

### Decision 3: 两层输入协议——手势跨侧，分发在内核

同一个指向输入按游戏状态可能意味着不同动作（空格=移动、物品格=拾取、怪物格=攻击）。解析需要游戏状态，天然属于内核；输入插件若做解析，就得查询物品/怪物等游戏状态，协议面随可交互对象种类膨胀。因此协议分两层：

```mermaid
flowchart LR
    mouse[input/systems/mouse: 左键] -->|PrimaryAction cell 手势| dispatch[core/hero: resolve_primary_action]
    dispatch -->|可通行格| path[挂 Path]
    path --> follow[core/movement: follow_path]
    follow --> pos[Position 推进]
    pos --> sync[graphic/sync: sync_position]
    sync --> trans[Transform]
    trans --> anim[graphic/animation: animate]
    trans --> cam[graphic/camera: follow_target]
```

- **手势层（跨侧协议）**：`PrimaryAction(IVec2)`，表达"主操作落在此格"。鼠标点击、键盘光标+确认键、触摸点按都映射到它；模态状态（键盘光标位置、按键映射）留在 input 插件内部，不进协议。消息落在 `core/hero/events/primary_action.rs`——域组织规范没有 events 侧面，本次扩展一个 `events/` 侧面。
- **意图分发（内核）**：`resolve_primary_action` 读手势 + 游戏状态，决定具体动作；当前只有"可通行则挂 Path"。未来第一种"一点多义"出现时，分发逻辑在这里生长，input 侧零改动。
- 屏幕 → 世界 → 格的换算仍在 input（查询 graphic 相机，参考 bevy `examples/2d/2d_viewport_to_world.rs`）；可通行校验与寻路在内核。
- 与原版 PD 的差异：PD 在输入侧直接产出具体意图（`HeroAction.Move/Attack/PickUp`，见 GameScene 的点击分发）；本设计把上下文解析移入内核，换取输入插件的可替换性——跨侧协议只说"主操作落在哪格"，具体意图词汇表是内核内部分发结果。

### Decision 4: input 直接依赖 graphic（经确认的取舍）

点击换算需要相机投影，方案备选是内核持有 `CellPicker` trait、graphic 注册实现。采用更简单的直接依赖：input 查询 graphic 的相机组件与换算函数。代价是换 MUD 文字显示（无相机）时 input 需配套替换——实际上文字显示的输入本来就不是屏幕指向，配套替换是合理代价。内核协议不因此沾染显示概念。

### Decision 5: hero 生成拆为内核生成 + 显示补挂

- `core/hero/entities/hero.rs`（Startup）：spawn `Hero` 标记 + `Position(HERO_START)`。
- `graphic/hero`：系统查询 `Added<Hero>`，补挂 `Sprite`、`TextureAtlas`、`AnimClips`、`AnimTimer`、`AnimState`、`CameraTarget` 与初始 `Transform`（z = `LAYER_ACTOR`）。显示数据（warrior 12×15 精灵表、tier 0 帧表、IDLE/RUN 帧序列）留在 `graphic/hero/constants/`，帧表还原原版（参考 pixel-dungeon `HeroSprite.java`：idle 在 0/1 间呼吸，run 循环 2–7）。
- 该模式即"内核生成游戏实体、显示插件注册外观"的协议实例，未来怪物等实体沿用。

### Decision 6: 集合化装配，集合标签归内核持有

装配层若引用具体系统函数，替换任一实现（input 不再是鼠标、graphic 没有动画）都要改 main.rs。Bevy 的 `SystemSet` 正是为此设计（官方 `examples/ecs/ecs_guide.rs`）：各侧把系统注册进抽象集合标签，装配层只编排标签。

- 三个标签 `InputSet` / `CoreSet` / `GraphicSet` 定义在 `core::sets`。它们是跨侧顺序编排的协议，按"内核持有协议"原则归内核；放在装配层会形成双向依赖（装配层引用插件的 register，插件反向引用装配层的标签）。
- 各侧 register 自行展开内部顺序：core 内 `(resolve_primary_action, follow_path).chain().in_set(CoreSet)`；graphic 内 `(attach_appearance, sync_position, animate, follow_target).chain().in_set(GraphicSet)`；input 内 `mouse_primary_action.in_set(InputSet)`。
- main.rs 只剩一句编排：`configure_sets(Update, (InputSet, CoreSet, GraphicSet).chain())`。

## Risks / Trade-offs

- [input 依赖 graphic：换成无相机的显示实现时 input 需配套替换] → 取舍已经确认（Decision 4）；input 对 graphic 的依赖收敛为相机组件与一个换算函数两个接触点。
- [`Position` 与 `Transform` 两份位置可能不一致] → `Transform` 的 x/y 每帧被 `sync_position` 从 `Position` 重写，不再是位置真相来源；z 由显示侧生成时设定并保持。
- [集合内顺序由单侧自治，跨侧只有标签级先后] → 跨侧顺序需求只有"输入→内核→显示"一层；未来出现更细粒度跨侧约束时，在 core::sets 增设子标签。
- [目录大规模移动期间编译断点较多] → 一次性迁移，以 `cargo check` 收敛；现有 `grid_map`、`pathfinding` 单元测试保持不变作为回归网。

## Migration Plan

一次性重组：先移 core（纯数据与逻辑，测试先行通过），再移 graphic 与 input，最后改装配与执行链。验证方式：`cargo test` 全绿 + 运行游戏手动确认点击移动、动画、相机跟随、水面效果与现状一致。无存档与数据迁移。
