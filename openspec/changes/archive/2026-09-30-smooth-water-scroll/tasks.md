历史归档，不符合先英文后中文规范，请勿参考

# Tasks: smooth-water-scroll

## 1. 屏幕空间动画地形层

- [x] 1.1 世界 pass 撤水：画布相机清屏透明，地形格成透明洞。verify：水格 alpha-0。
- [x] 1.2 `terrain_animation` 域复活（机制换代）：地图尺寸地形层垫在画布精灵之下，REPEAT + ColorMaterial，零着色器。verify：水面透出，像素硬块。
- [x] 1.3 世界锁定：follow_target 按 `world_anchor − target` 摆层；时间流动走 `uv_transform`。verify：镜头移动时水纹与世界同速同相；单测覆盖锁定定律与流动积分。

## 2. 动态画布

- [x] 2.1 尺寸规则 `canvas_size_for_window`（⌈窗口/2⌉ 取偶 + 余量）与纹理随窗重建 `sync_canvas_size`。verify：单测锁定取偶规则与重建；拖窗视野伸缩。
- [x] 2.2 鼠标映射简化（`window_to_canvas` 去 Option 守卫）。verify：点击准确。

## 3. 精灵纹素锚定

- [x] 3.1 `texel_aligned_anchor`：奇数尺寸帧轴半纹素偏移（y 向下、脚底压格底），引擎 `Anchor` 承载。verify：3 个单测；实机主角不再"额头长一行、腿短一行"。

## 4. 收尾

- [x] 4.1 实机总验收：拖窗伸缩视野、水锁地图流动、主角锐利、虚空深色、暂停水停。verify：用户多轮实机确认（含水层三版形态对比与主角锚定前后对照）。
- [x] 4.2 注释/测试/spec/design/tasks 统一（终态表述）；OPEN_ISSUES 同步。verify：全项目搜索无残留，strict 校验通过。
- [x] 4.3 全绿提交。verify：`cargo +nightly fmt --check`、`cargo clippy --all-targets`、`cargo check --all-targets`、`cargo test` 全过。
