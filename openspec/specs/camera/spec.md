# camera Specification | camera 规格

## Purpose

The 2D camera's behavior: the camera follows the player and scrolls
the view, keeping the player at the view's center; the view may extend
beyond the map, and the void outside shows the clear color (matching
the reference Pixel Dungeon).

定义 2D 相机行为：相机跟随主角滚动视野，主角始终保持在画面中心；视野
允许超出地图范围，图外区域显示为清屏背景色（与原版 PD 一致）。

## Requirements

### Requirement: Camera Follow | 相机跟随

The camera SHALL follow the player: its position snaps to the
screen-pixel grid (the lattice pitch re-derived live from the window's
scale_factor), so the player never drifts more than 0.5 screen pixels
from the view's center. The view may extend beyond the map, and the
region outside shows the clear color (matching the reference Pixel
Dungeon).

相机 SHALL 跟随主角：镜头位置吸附屏幕像素网格（网格间距随窗口
scale_factor 实时换算），主角与视野中心的偏差因此不超过 0.5 屏幕像
素。视野可以超出地图范围，地图外的区域显示为清屏背景色（与原版 PD 一
致）。

#### Scenario: The camera follows a moving player | 主角移动时相机跟随

- **WHEN** the player moves anywhere inside the room | 主角在房间内任意位置移动时
- **THEN** the camera moves in step, and the player stays within 0.5 screen pixels of the view's center | 相机同步移动，主角距视野中心不超过 0.5 屏幕像素

#### Scenario: The player in a room corner | 主角在房间角落

- **WHEN** the player moves into a corner of the room | 主角移动到房间角落时
- **THEN** the player still sits at the view's center (within 0.5 screen pixels), and the part of the view beyond the room shows the clear color | 主角仍位于视野中心（偏差不超过 0.5 屏幕像素），视野中房间外的部分显示为背景色

### Requirement: Camera Shake | 相机震动

On a shake request — a magnitude and a duration — the camera SHALL
offset its position each frame by a uniform random draw within
±magnitude per axis, linearly damped to zero across the duration;
once the duration has passed, the offset SHALL be exactly zero. The
offset SHALL be applied before the pixel-grid snap, so a shaking view
stays on the grid instead of smearing.

震动请求——幅度与时长——到达时，相机 SHALL 每帧按 ±幅度内的均匀
随机取各轴偏移，并随时长线性衰减至零；时长结束后偏移 SHALL 恰为
零。偏移 SHALL 在像素网格吸附之前施加，震动中的视野保持整齐不拖
影。

#### Scenario: The shake decays to zero | 震动衰减至零

- **WHEN** a shake of magnitude 4 and duration 0.3 seconds runs | 幅度 4、时长 0.3 秒的震动运行时
- **THEN** each frame's per-axis offset stays within 4 world pixels, shrinks as time passes, and is exactly zero after 0.3 seconds | 每帧各轴偏移不超过 4 世界像素，随时间收窄，0.3 秒后恰为零

#### Scenario: The shaking view stays on the grid | 震动的视野保持网格整齐

- **WHEN** a shake is running | 震动运行中时
- **THEN** the camera's rendered position still lands on the screen-pixel grid | 相机的渲染位置仍落在屏幕像素网格上
