# plugin-architecture Specification | plugin-architecture 规格

## Purpose

The boundary and replaceability contract between the core and the
frontend: the frontend carries everything player-facing — world
rendering, HUD, input modalities — and can be replaced as a whole (a
sprite-graphical interface, a text interface, …) with the core
untouched.

定义内核（core）与界面层（frontend）的边界与可替换性契约：frontend 承
载一切面向玩家的内容——世界渲染、HUD、输入模态——可以整体替换（精灵
图形界面、文字界面等）而内核不变。

## Requirements

### Requirement: Two-Side Boundary and One-Way Dependency | 两方边界与单向依赖

Game code SHALL divide into two sides, the core and the frontend. The
core holds game data and general logic and MUST NOT reference concrete
frontend types; the frontend SHALL depend one-way on the protocols the
core holds (components and data types). Replacing the frontend
implementation MUST touch only the orchestration layer, with zero
changes to core code.

游戏代码 SHALL 分为内核与界面层两方。内核持有游戏数据与通用逻辑，MUST
NOT 引用界面层的具体类型；界面层 SHALL 单向依赖内核持有的协议（组件与
数据类型）。替换界面层实现 MUST 只需修改编排层，内核代码零改动。

#### Scenario: The core never depends on frontend implementations | 内核不依赖界面层实现

- **WHEN** every dependency reference of the core code is inspected | 检查内核代码的全部依赖引用时
- **THEN** no reference to frontend types (textures, sprites, animation frame tables, cameras, input modalities) exists | 不存在对界面层类型（贴图、精灵、动画帧表、相机、输入模态）的引用
- **AND** the core only exposes the game-state protocol for frontends to read | 内核仅暴露游戏状态协议供界面层读取

#### Scenario: Replacement never touches the core | 替换不触及内核

- **WHEN** the current frontend implementation is replaced by another (such as swapping the sprite-graphical interface for a text one) | 用另一套界面层实现替换现有实现（如精灵图形界面换为文字界面）时
- **THEN** every modification happens inside the frontend itself and in the orchestration layer | 所有修改都发生在界面层自身与编排层
- **AND** the core code is unchanged | 内核代码无任何改动

### Requirement: Rendering-Independent Entity Position | 显示无关的实体位置

The core SHALL maintain the logical position as an integer cell
coordinate — the one position game logic reads. The display side SHALL
maintain the presentation position in continuous cell units (1 unit =
1 cell, floating point): at rest it MUST sit on an integer cell
center, and it may leave integers only while a move is under way — the
single source of smooth motion. Neither side SHALL express an entity's
position in pixels or render transforms, and presentation state MUST
NOT be written back into the core.

内核 SHALL 以整数格坐标维护逻辑位置——游戏逻辑读取的唯一位置。显示侧
SHALL 以连续格坐标维护呈现位置（1 单位为 1 格，浮点）：停留时 MUST 恒
为格中心（整数格坐标），非整数格坐标只允许在移动过程中出现——平滑移
动的唯一来源。任何一侧 MUST NOT 用像素单位或渲染变换表达实体位置，呈
现状态 MUST NOT 回写内核。

#### Scenario: Integer cells at rest | 静止时位置为整数格

- **WHEN** an entity is not moving | 实体不在移动中时
- **THEN** its presentation position sits on integer cell coordinates on both axes | 其呈现位置的横纵坐标均为整数格坐标

#### Scenario: Fractional cells mid-move | 移动中经过非整数格坐标

- **WHEN** an entity walks from one cell to an adjacent one | 实体沿路径从一格走向相邻格时
- **THEN** its presentation position transitions continuously between the two cells, passing through fractional cell coordinates | 其呈现位置在两格之间连续过渡，经过非整数格坐标
- **AND** the position is back on integer cell coordinates upon arrival | 到达目标格后坐标回到整数格

#### Scenario: Each frontend converts on its own | 界面层各自量化

- **WHEN** a frontend implementation reads the positions | 界面层实现读取实体的位置时
- **THEN** a graphical interface converts the presentation position into pixel coordinates, and a text interface reads the core's cell coordinate | 图形界面将呈现位置换算为像素坐标，文字界面读取内核的格坐标
- **AND** conversion happens only on the frontend side, never written back into the core's state | 换算只发生在界面层，不回写内核位置

### Requirement: The Frontend Is Replaceable as a Whole | 界面层可整体替换

The frontend SHALL be replaceable as a whole — it may be a graphical
interface built on sprite sheets and tilesets, or a text interface
(reporting position, coordinates, and surrounding terrain, driven by
text commands). Both implementations SHALL consume the same core game
state and produce the same command protocol. Replacement MUST happen
through the orchestration layer alone. Replaceable units inside the
frontend (textures, animations, HUD, input modalities) are bounded by
directories and do not constitute plugins of their own.

界面层 SHALL 可整体替换——既可以是基于精灵表与 tileset 的图形界面，也
可以是文字界面（报告所处位置、坐标与周围地形，以文字命令输入）。两种
实现 SHALL 消费同一份内核游戏状态并产出同一套命令协议。替换 MUST 只
通过编排层完成。界面层内部的替换单元（贴图、动画、HUD、输入模态）以
目录为界，不构成独立插件。

#### Scenario: A text interface consumes the same game state | 文字界面消费同一游戏状态

- **WHEN** the orchestration layer swaps the graphical interface for a text one | 编排层将图形界面替换为文字界面时
- **THEN** the text interface reads entity positions and map terrain from the core protocol and outputs descriptions, producing commands directly from text instructions | 文字界面从内核协议读取实体位置与地图地形并输出描述，从文字指令直接产生命令
- **AND** core behaviors — map data, pathfinding, movement — are unaffected | 地图数据、寻路、移动等内核行为不受影响

### Requirement: The Core Holds the Command Protocol and Executes It | 命令协议由内核持有并校验执行

The core SHALL hold the concrete command messages (such as
`MoveToCell(cell)`: walk to the target cell) as the cross-side
protocol, validating and executing what it receives. Every request the
frontend makes to change core state MUST go through a command message;
it MUST NOT write core state directly. Command and gesture payloads
MUST use the cell coordinate type `CellCoord`, never bare math
vectors.

内核 SHALL 持有具体命令消息（如 `MoveToCell(cell)`：走向目标格）作为跨
侧协议，收到后校验并执行。界面层对内核状态的一切变更请求 MUST 经命令
消息，MUST NOT 直接改写内核状态。命令与手势的载荷 MUST 使用格子坐标
类型 `CellCoord`，MUST NOT 使用裸数学向量。

#### Scenario: An impassable command produces no movement | 不可通行命令不产生移动

- **WHEN** a command's target cell is impassable (wall, water, off the map) or unreachable | 命令指向的格子不可通行（墙、水、图外）或不可达时
- **THEN** the core produces no movement | 内核不产生移动

#### Scenario: The frontend never writes core state directly | 界面层不直接改写内核状态

- **WHEN** the frontend code's operations on core data are inspected | 检查界面层代码对内核数据的操作时
- **THEN** only read-only queries and command messages exist; no direct writes to core components or resources | 只存在只读查询与命令消息，不存在对内核组件/资源的直接写入

### Requirement: Gesture Resolution Belongs to the Frontend | 手势解析归属界面层

Translating raw input into commands SHALL happen on the frontend, in
two stages. **Hit testing** — whether the pointer lands on a sprite
mask or passes through to the ground — depends on presentation data
(masks, occlusion order) and MUST be the frontend's call; it produces
gestures typed by target (`PrimaryActionOnCell`,
`PrimaryActionOnMonster`, and so on — one gesture type per hittable
target kind). **Policy resolution** reads the gesture and the core
state and decides the concrete command. Input modalities MUST stay
free of game semantics: they only report what the pointer hit, never
query game state to decide what it means. Adding a hittable target
kind MUST take only a new gesture type and its resolver, never
modifying existing modality code.

原始输入到命令的解析 SHALL 在界面层完成，分两级：**命中判定**（指针落
在精灵遮罩上还是穿透到地面——依赖遮罩、遮挡顺序等呈现数据，MUST 由界
面层判断）产出按目标分类的手势（`PrimaryActionOnCell`、
`PrimaryActionOnMonster` 等，一种可命中目标一个类型）；**策略解析**读
手势与内核状态，决定具体命令。输入模态 MUST 保持无游戏语义：只报告指
针命中了什么，不查询游戏状态决定含义。新增可命中目标种类 MUST 只新增
手势类型与对应 resolver，不修改既有模态代码。

#### Scenario: Pointing input translates into target-typed gestures | 指向输入翻译为按目标分类的手势

- **WHEN** an input modality receives one valid pointing input (mouse click, keyboard cursor confirm, touch tap) | 输入模态收到一次有效指向输入（鼠标点击、键盘光标确认、触摸点按）时
- **THEN** hit testing emits the gesture of the matching target type (a ground cell → `PrimaryActionOnCell`) | 命中判定后发出对应目标类型的手势（点地面格 → `PrimaryActionOnCell`）
- **AND** the gesture carries only game coordinates or entities, never screen or pixel concepts | 手势仅含游戏坐标或实体，不含屏幕或像素概念

#### Scenario: A new hittable target changes no modality | 新增可命中目标不改模态

- **WHEN** the frontend gains a hittable target kind (monster, item) | 界面层新增可命中目标种类（怪物、物品）时
- **THEN** a new gesture type and resolver are added, with zero changes to input modality code | 新增对应手势类型与 resolver，输入模态代码零改动

#### Scenario: Alternative interfaces may bypass gestures | 替代界面可绕过手势

- **WHEN** an alternative implementation such as a text interface produces commands directly from explicit instructions (such as `attack kobold`) | 文字界面等替代实现从明确指令（如 `attack kobold`）直接产生命令时
- **THEN** the gesture stage is not needed | 不需要手势层
- **AND** the core's command protocol and execution behavior are unchanged | 内核的命令协议与执行行为不变

### Requirement: Set-Based Orchestration | 集合化编排

The orchestration layer SHALL arrange the main-loop stage order by
referencing only abstract system set labels (SystemSet) —
`GameLoop::{Input, Core, Display}` (command production, core logic,
presentation) — and MUST NOT reference concrete system functions. The
set labels MUST be held by the core as part of the protocol; each side
registers its systems into the matching sets inside its own register
and orchestrates its own intra-set order.

编排层 SHALL 只引用抽象系统集合标签（SystemSet）排定主循环阶段顺序——
`GameLoop::{Input, Core, Display}`（命令产出、内核逻辑、呈现）——MUST
NOT 引用具体系统函数。集合标签 MUST 由内核持有，作为协议的一部分；各
方在自己的 register 内把系统注册进对应集合，并自行编排集合内部顺序。

#### Scenario: The orchestration layer names no concrete system | 编排层不指名具体系统

- **WHEN** the orchestration layer's (main.rs) stage-ordering code is inspected | 检查编排层（main.rs）的阶段排序代码时
- **THEN** only set-label ordering appears, with no concrete system function referenced | 只出现集合标签的顺序编排，无任何具体系统函数引用

#### Scenario: Replacing an implementation leaves the orchestration untouched | 替换实现不动编排层

- **WHEN** a frontend plugin is replaced by a new implementation that registers into the same set labels | 用注册了同一集合标签的新实现替换界面层插件时
- **THEN** the orchestration layer code is unchanged | 编排层代码零改动
