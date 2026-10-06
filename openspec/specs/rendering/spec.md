# rendering Specification | rendering 规格

## Purpose

The 2D rendering foundation of a pixel-art roguelike: layered tile map
rendering, shore autotiling, sprite sheet rendering, an animated
terrain layer (water), and the desktop window configuration.

提供像素风 roguelike 的 2D 渲染基础：tile 地图分层渲染、水岸自动拼合、
精灵表渲染、动画地形层（水面），以及桌面窗口配置。

## Requirements

### Requirement: Pixel-Art Direct Pipeline | 像素风直绘管线

Terminology: a world pixel (world) is the game world's unit of length
(a tile is 16×16); a window pixel (window) is the windowing system's
logical unit (window sizes and cursor positions come in it); a screen
pixel (screen) is the physical framebuffer pixel the GPU rasterizes
onto. Conversion: nominally 1 world pixel = 2 window pixels; 1 window
pixel = scale_factor screen pixels; **the effective magnification =
round(2 × scale_factor), always an integer** (150% → 3, 175%/215% →
4), the view size absorbing the rounding deviation.

术语：世界像素（world，游戏世界长度单位，tile 16×16）；窗口像素
（window，窗口系统的逻辑单位，窗口尺寸与光标位置用之）；屏幕像素
（screen，物理帧缓冲像素，GPU 光栅化之处）。换算：1 世界像素标称 = 2
窗口像素；1 窗口像素 = scale_factor 屏幕像素；**有效倍率 = round(2 ×
scale_factor)，恒为整数**（150% → 3、175%/215% → 4），视野尺寸吸收取
整偏差。

The game SHALL render every texture with nearest-neighbor sampling;
the tile size is 16×16 world pixels and the player sprite frame 12×15.
At any window size and display scale, still or moving, sprite and tile
edges SHALL NOT show stray lines sampled from other frames of the
texture. The guarantee rests on four disciplines:

游戏 SHALL 以最近邻采样渲染所有纹理；tile 尺寸为 16×16 世界像素，主角
精灵帧为 12×15。任何窗口尺寸与显示器缩放下、静止与移动中，精灵与
tile 边缘 SHALL NOT 出现取自贴图其他帧的杂色线。该保证由四条纪律成
立：

1. every texture sampled nearest-neighbor;
2. the single camera's MSAA off;
3. the world-to-screen magnification always an integer (effective
   magnification = round(ZOOM × scale_factor), at least 1) — so every
   frame's screen-pixel size is an integer and both edges can land on
   the screen grid at any display scale;
4. every presentation source's presented position SHALL snap to the
   screen-pixel grid — the lattice pitch re-derived live from the
   window's scale_factor (1 screen pixel = 1/effective-magnification
   world units) — so that no screen pixel center ever sits on a frame
   boundary (frame-boundary misreads happen only there).

1. 全部纹理最近邻采样；
2. 唯一相机 MSAA 关闭；
3. 世界到屏幕的放大倍率恒为整数（有效倍率 = round(ZOOM ×
   scale_factor)，至少为 1）——任何帧尺寸的屏幕像素尺寸因此恒为整
   数，任意显示器缩放档位下两条边都能落在屏幕网格上；
4. 一切呈现源的呈现位置 SHALL 吸附到屏幕像素网格——网格间距随窗口
   scale_factor 实时换算（1 屏幕像素 = 1/有效倍率 世界单位）——使任
   何屏幕像素中心都不落在帧边界上（帧边界的错误采样只在像素中心落
   于其上时发生）。

Snapped to the screen grid, an entity's smallest movement quantum is 1
screen pixel; mid-move, the entity's pixel blocks may sit offset from
the terrain's pixel blocks by a non-integral number of blocks — an
accepted, inherent appearance.

实体吸附屏幕像素网格后，其运动最小单位为 1 屏幕像素；运动中实体像素
块与地形像素块可按非整块距离错开，此为已接受的固有外观。

Frames SHALL be anchored so the figure's feet sit flush on its cell:
when a frame's width or height is odd in texels, centering would leave
the sprite half a world pixel off the cell's edge in that direction,
so the anchor nudges half a world pixel along it (down on y, feet
flush with the cell's bottom edge). Grid alignment is not the anchor's
business — the snap discipline lands every quad's edges on
screen-pixel boundaries whatever the anchor offset's fractional part
is.

帧的锚定 SHALL 使形象站立时脚底与所在格的边缘贴齐：帧的宽或高为奇数
像素（texel）时，居中会让精灵在该方向上偏离格边缘半个世界像素，锚
点因此在该方向偏移半个世界像素（竖直方向向下，脚底贴齐格底边）。对
齐屏幕网格不是锚点的职责——无论锚点偏移的小数部分如何，吸附纪律都
保证四边形边缘落在屏幕像素边界上。

#### Scenario: No stray lines | 杂色线不出现

- **WHEN** any frame is rendered, still or moving, at any window size and display scale | 静止或移动中、任意窗口尺寸与显示器缩放设置下渲染任意一帧时
- **THEN** sprite and tile edges show no stray lines sampled from other frames of the texture | 精灵与 tile 边缘不出现取自贴图其他帧的杂色线

#### Scenario: No blur on magnification | 纹理放大不模糊

- **WHEN** world content is presented magnified through nearest-neighbor sampling | 世界内容经最近邻放大呈现时
- **THEN** pixel edges are sharp, with no bilinear blur | 像素边缘锐利，无双线性插值模糊

#### Scenario: Slow entities move smoothly | 慢速实体平滑移动

- **WHEN** an entity moves slower than 1 world pixel per frame | 实体以低于 1 世界像素/帧的速度移动时
- **THEN** its presentation translates in quanta of 1 screen pixel, never stepping by whole world pixels (several screen pixels) at once | 其呈现以 1 屏幕像素为最小单位平移，不出现按世界像素（多个屏幕像素）跳格的步进

#### Scenario: Frame edges land on pixel boundaries | 帧边缘落在像素边界

- **WHEN** any sprite frame is rendered, still or moving | 渲染任意精灵帧（静止或移动中）时
- **THEN** the frame's edges land on screen-pixel boundaries — no row or column is misread, duplicated, or lost from an edge straddling a pixel center | 帧边缘落在屏幕像素边界上，不因边缘跨在像素中心上而出现行或列的错误采样、复制、丢失

#### Scenario: Uniform pixel blocks at any display scale | 任意缩放档位像素块均匀

- **WHEN** the display scale is fractional (such as 175% or 215%) | 显示器缩放设置为分数值（如 175%、215%）时
- **THEN** the effective magnification rounds to an integer, pixel blocks stay uniform, and sprite and tile edges still show no stray lines | 有效倍率取整到整数，像素块保持均匀；精灵与 tile 边缘仍不出现取自贴图其他帧的杂色线

### Requirement: Layered Map Rendering | 地图分层渲染

The game SHALL render at fixed z layers: the ground plane (z=0), the
actor layer (z=2), and the overhead layer (z=3). The overhead layer
sits above the actor layer as a structural guarantee (the player's
12×15 sprite and the 16px wall tile never overlap in pixels, so no
dynamic sorting is needed).

游戏 SHALL 按固定 z 层渲染：地板层（z=0）、主角层（z=2）、墙层
（z=3）。墙层 z 高于主角层，作为结构保证（主角 12×15 精灵与 16px 墙
格像素不重叠，无需动态排序）。

#### Scenario: Fixed layer order | 层序固定

- **WHEN** any frame is rendered | 渲染任意一帧时
- **THEN** the overhead layer draws above the actor layer | 墙层绘制在主角层之上

### Requirement: Shore Autotiling | 水岸自动拼合

A liquid cell SHALL render as one of 16 shoreline variants chosen by a
4-bit mask of its four orthogonal neighbors — the bit set when the
neighbor is unwalkable, liquid, or off the map; shoreline variants
carry semi-transparent edge pixels. A cell fully surrounded by liquid
MUST render as the fully transparent open-liquid tile, letting the
animated terrain layer show through.

水域格 SHALL 按四正交邻居的 4 位掩码（邻居不可通行、为液体或在图外则
置位）渲染为 16 种岸线变体之一；岸线变体含半透明边缘像素。完全被水包
围的格子 MUST 渲染为全透明开放水瓦片，使动画地形层透出。

#### Scenario: Shorelines follow the neighbors | 岸线随邻居变化

- **WHEN** a liquid cell adjacent to wall or off-map cells is rendered | 渲染与墙或图外相邻的水域格时
- **THEN** the cell draws the matching shoreline variant; a cell fully surrounded by liquid draws the fully transparent open-liquid tile | 该格绘制对应的岸线变体；完全被水包围的格子绘制全透明开放水瓦片

### Requirement: Animated Terrain Layer | 动画地形层

The game SHALL render an animated terrain layer in world space:
animated-terrain cells render as alpha-0 cells in the terrain mesh,
and the animated terrain layer lies beneath the mesh showing through.
The layer SHALL stay locked to the map (panning with the camera as
world content, never sliding against the world) and flow at a constant
velocity; texture sampling SHALL be nearest-neighbor (hard pixel
blocks). The flow MUST stop while the game is paused (it follows the
game's virtual time).

游戏 SHALL 在世界空间中渲染动画地形层：动画地形格在地形网格上渲染为
alpha-0 单元格，动画地形层垫在地形网格之下透出。地形层 SHALL 与地图
锁定（作为世界内容随镜头平移，相对世界无滑动），并以恒定速度持续流
动；纹理采样 SHALL 为最近邻（像素硬块）。暂停时流动 MUST 停止（跟随
游戏虚拟时间）。

#### Scenario: Water flows continuously | 水面持续流动

- **WHEN** liquid cells exist in the scene and the game is not paused | 场景中存在水域格且游戏未暂停时
- **THEN** the water shows through the terrain mesh's liquid cells, flowing at a constant velocity with sharp pixel edges | 水面透过地形网格的水格显露，以恒定速度持续流动，像素边缘锐利

#### Scenario: The layer is world-locked | 地形层与世界锁定

- **WHEN** the player moves and the camera follows | 主角移动、镜头跟随时
- **THEN** the animated terrain layer pans with the map at the same speed and phase, never sliding against the world; the flow rides on top | 动画地形层与地图同速同相平移，相对世界无滑动；流动叠加其上

#### Scenario: Flow stops on pause | 暂停时停止

- **WHEN** the game pauses | 游戏暂停时
- **THEN** the animated terrain layer's flow stops | 动画地形层的流动停止

### Requirement: Desktop Window | 桌面窗口

The game SHALL start with a 1280×720 window-pixel window (PC desktop,
resizable by dragging). The world SHALL render straight to the window:
the single camera presents it at an integer magnification (effective
magnification = round(ZOOM × scale_factor)) with the camera position
snapped to the screen-pixel grid. The view tracks the window (roughly
window size ÷ 2, varying slightly with the magnification rounding —
about 640×360 world pixels, about 40×22.5 cells at startup): a larger
window shows more world, a smaller one less. One world pixel always
presents as an integer screen-pixel block.

游戏 SHALL 以 1280×720 窗口像素启动（PC 桌面平台，允许拖动改尺寸）。
世界 SHALL 直接渲染到窗口：唯一相机以整数倍放大呈现（有效倍率 =
round(ZOOM × scale_factor)），镜头位置吸附屏幕像素网格。视野随窗口变
化（约窗口尺寸 ÷ 2 并随有效倍率取整略有增减，启动时约 640×360 世界像
素、约 40×22.5 格）：窗口拖拽放大看到更多世界，拖拽缩小看到更少。1 世界像素
恒呈现为整数屏幕像素块。

#### Scenario: The startup window | 启动窗口

- **WHEN** the game starts | 启动游戏时
- **THEN** a 1280×720 window-pixel window opens, the world presented at an integer magnification showing part of the room (about 40×22.5 cells), tiles appearing as integer pixel blocks (nominally 32×32 window pixels) | 打开 1280×720 窗口像素窗口，世界按整数倍放大呈现，显示房间的一部分（约 40×22.5 格），tile 在屏幕上呈整数像素块（标称 32×32 窗口像素）

#### Scenario: Movement smooths by screen pixels | 移动按屏幕像素平滑

- **WHEN** the player or an entity moves at low speed | 主角移动或实体低速移动时
- **THEN** world scrolling and entity movement present smoothly in quanta of 1 screen (physical) pixel, never stepping by world pixels (2 window pixels) | 世界滚动与实体移动以 1 屏幕（物理）像素为最小单位平滑呈现，不按世界像素（2 窗口像素）跳格

#### Scenario: Dragging the window larger | 窗口拖拽放大

- **WHEN** the window is dragged beyond 1280×720 | 窗口被拖拽放大到超过 1280×720 时
- **THEN** the view grows with it, showing more world; one world pixel still presents as an integer screen-pixel block | 视野随之变大，看到更多世界内容；1 世界像素仍呈整数屏幕像素块

#### Scenario: Dragging the window smaller | 窗口拖拽缩小

- **WHEN** the window is dragged below 1280×720 | 窗口被拖拽缩小到不足 1280×720 时
- **THEN** the view shrinks with it, showing less world; one world pixel still presents as an integer screen-pixel block | 视野随之变小，看到更少世界内容；1 世界像素仍呈整数屏幕像素块
