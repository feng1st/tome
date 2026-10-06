历史归档，不符合先英文后中文规范，请勿参考

# Tasks: direct-render-pipeline

## 1. 吸附基础设施

- [x] 1.1 实现屏幕网格换算工具（world_scale_factor = ZOOM × window_scale_factor，window_scale_factor 实时读窗口、缺省 1.0）。verify：单测覆盖 SF=1.0→2.0、SF=1.5→3.0、SF=2.0→4.0 三例
- [x] 1.2 实现 min 角吸附函数：输入相机相对位置与 min 角偏移，输出吸附后的呈现位置（边缘落屏幕像素边界）。verify：单测覆盖任意小数输入、奇/偶帧尺寸两例、负坐标；断言吸附后帧边缘在整数屏幕像素上

## 2. 相机重构

- [x] 2.1 单 MainCamera 替换双相机装置：scale 恒 0.5（ScalingMode::WindowSize）、Msaa::Off 挂相机、渲染目标为窗口；删除 CanvasCamera/ScreenCamera 及双相机编排。verify：`cargo run` 启动出现世界画面（相位问题此时允许存在，2.2/3.1 收口），`cargo test` 编译通过
- [x] 2.2 相机吸附：cam = round(hero_pos × world_scale_factor) / world_scale_factor，每帧执行、每帧读 window_scale_factor。verify：单测覆盖浮点 hero 位置吸附后在网格上、主角偏心 ≤0.5 屏幕像素、SF 变化网格跟随

## 3. 呈现源接入

- [x] 3.1 实体吸附系统（Display 相位、排在相机吸附之后）：所有带 Position 的呈现实体经 min 角吸附写回 Transform。verify：单测断言吸附后 min 角在屏幕网格、Anchor 的"脚底压格底"语义保持（吸附前后脚底相对格底的世界偏移不变）
- [x] 3.2 地形 chunk 直绘窗口、水层改为世界锚定 + z 低于地形网格（删除 follow_target 的水层世界锁定写入）。verify：单测锁定 z 序常量与水层位置公式；含水域的测试地图加载不报错
- [x] 3.3 鼠标映射改写为 window→world 一处换算（cam + (光标 − 窗口逻辑中心)/2）；直绘后窗口即视野，原"落出视野返回 None"的余量环语义删除。verify：单测覆盖窗口中心点击映射到主角所在格、四角映射

## 4. 清理与收尾

- [x] 4.1 删除 canvas 域（画布纹理/精灵/尺寸重建/余量几何常量）；锚点函数保留并改名 `sprite_anchor`，职责收窄为"脚底压格底"（网格对齐职责已被 1.2 的 min 角吸附吸收）；命名终态统一（world/window/screen 三术语入 layout.rs 模块注释与 rendering spec，`ZOOM`/`world_scale_factor`/`SpriteSize`/`snap_sprites`/`screen_grid`）。verify：`cargo test` 全绿；`grep -rn "canvas\|Canvas" src/` 无残留引用
- [x] 4.2 `openspec validate direct-render-pipeline --strict` 通过；delta spec 与实现对读一遍（需求逐条有代码落点）。verify：validate 输出无 error
- [x] 4.3 有效倍率取整化（实机发现 175%/215% 档位老鼠头顶固定线：非整数倍率使帧屏幕尺寸为小数、远边压像素中心）：`world_scale_factor = round(ZOOM × scale_factor)` 恒为整数，投影 scale 每帧 = sf/wsf_eff，鼠标映射同步有效倍率，spec"分数缩放宽容"改写为"任意档位像素块均匀"。verify：单测覆盖 1.75/2.15 取整、投影 scale 换算、鼠标映射有效倍率；`cargo test` 全绿
