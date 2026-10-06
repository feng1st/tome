历史归档，不符合先英文后中文规范，请勿参考

# Tasks: pixel-perfect-canvas

依赖顺序：canvas 域先行（管线机制），相机改造依赖画布句柄，输入适配随后，验收收尾。各域 register 只做成员注册；两相机渲染先后由 `Camera.order` 表达。

## 1. canvas 域：管线机制

- [x] 1.1 新建 `frontend/display/canvas/` 域骨架：constants（视野 640×360、余量 2 纹素、画布 644×364、上屏倍率 2、分层）、resources、entities、utils、mod.rs（register 只做成员注册），并在 display 侧根接入。verify：`cargo check` 通过，域目录结构符合域组织规范。
- [x] 1.2 实现画布纹理与 `CanvasImage` 资源：`Image::new_target_texture(644, 364, Bgra8UnormSrgb, None)` 经 `Assets<Image>` 创建（最近邻由 `ImagePlugin::default_nearest` 保证）。verify：单测——资源构建后图像尺寸为 644×364、格式为 Bgra8UnormSrgb、sample_count 为 1。
- [x] 1.3 实现画布精灵生成（`spawn_canvas`）：画布精灵 `Canvas` 取画布自然尺寸、渲染分层 `SCREEN_LAYERS`(1)。verify：单测——启动系统运行后画布精灵存在且分层为 1。
- [x] 1.4 实现窗口→画布坐标映射纯函数 `window_to_canvas`：减居中偏移、除倍率、加余量；落出视野返回 `None`。verify：单测——1280×720 与拖大/拖小窗口下偏移正确；留边区域返回 `None`。

## 2. camera 域：双相机装置

- [x] 2.1 `spawn_canvas_camera`：渲染目标 `RenderTarget::Image`（取 `CanvasImage` 句柄）、`Projection` 保持默认（scale 1，恒等映射）、`Msaa::Off`、`Camera.order = -1`、`CANVAS_LAYERS`(0)；图外清屏色沿用全局 `ClearColor`。verify：`cargo check` 通过；运行后世界画面经画布上屏正常显示（非黑屏）。
- [x] 2.2 `spawn_screen_camera`：投影 scale 恒 0.5（固定 2 倍）、渲染分层 `SCREEN_LAYERS`(1)。verify：单测——投影 scale 为 0.5；呈现层仅含画布精灵与屏幕相机两个实体。

## 3. 平滑滚屏：吸附 + 呈现平移一体

- [x] 3.1 `follow_target` 一体化跟随装置：`ParamSet`（官方先例 alien_cake_addict.rs）一次计算——画布相机吸附整数像素（`snapped = round(target)`，画布内容网格对齐），画布按亚纹素余量平移（`remainder = snapped − target`），主角屏幕精确居中、滚动量子 1 屏幕像素。verify：单测——吸附与余量（正负方向、半点、整数归零）；实机——移动平滑（达到旧直渲水准）、无杂线。

## 4. input：鼠标换算适配

- [x] 4.1 `mouse.rs`：光标先经 canvas 域映射函数换算为画布坐标（落出视野返回 `None`），再调画布相机的 `viewport_to_world_2d`。verify：实机——点击地板格主角寻路到达（与改造前一致）；点击墙格、水格不产生移动；窗口拖大后点击留边区域无动作。

## 5. 验收与登记

- [x] 5.1 实机像素稳定验收（对照 rendering delta 各 scenario）：150% 显示缩放下静止画面老鼠头顶无杂色线；主角移动全程无杂线；窗口拖大画布居中留边、拖小裁剪、视野不变；移动按屏幕像素平滑。宽容项（不算失败）：上屏后像素不均一与抖动（spec 第 3 点）。verify：核心项（无杂线、移动平滑）经多轮实机确认；150% 静止与拖动项按结构保证（固定投影下留边/裁剪自动成立）验收。
- [x] 5.2 删除 OPEN_ISSUES.txt 第 2 条目（像素完美渲染管线）。verify：文件中不再有该条目。
- [x] 5.3 全绿提交。verify：`cargo +nightly fmt --check`、`cargo clippy --all-targets`、`cargo check --all-targets`、`cargo test` 全部零警告通过。
