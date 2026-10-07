# camera delta for combat-feedback | camera 增量：combat-feedback

## ADDED Requirements

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
