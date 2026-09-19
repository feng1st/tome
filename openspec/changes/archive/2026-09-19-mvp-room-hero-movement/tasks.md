# Tasks: mvp-room-hero-movement

## 1. 工程骨架

- [x] 1.1 创建 `Cargo.toml`（bevy = "0.19"）与 `src/main.rs` 骨架：1280×720 窗口、`ImagePlugin::default_nearest()`；域目录 map/（含 water/ 子域）hero/camera/movement/animation，各域 register 由 main.rs 装配 — verify: `cargo build` 通过
- [x] 1.2 复制 `warrior.png` / `tiles0.png` / `water0.png` 到 `assets/`；`.gitignore` 加 `target/` — verify: `ls assets/` 有三个文件

## 2. 地图（map/ 域）

- [x] 2.1 实现 `GridMap` 资源与 `TileKind`（Floor/Wall/Water，`walkable()`），硬编码生成 64×48 房间（外墙封闭 + 中央偏下 8×5 水池） — verify: 单元测试——墙/水不可通行、地板可通行、越界不可通行
- [x] 2.2 运行时把 tiles0.png 网格图转成堆叠数组纹理（256×64 → 16×1024 共 64 层） — verify: 单元测试——转换后各层像素与源网格一致
- [x] 2.3 生成地板 chunk（z=0，地板=瓦片 1）与墙 chunk（z=3，墙=瓦片 4），整个房间一个 chunk — verify: `cargo run` 可见房间局部，tile 32px 清晰不模糊

## 3. 主角（hero/ 域）

- [x] 3.1 生成主角精灵（warrior.png，12×15 帧，tier 0），挂 animation 域的动画组件（Idle [0,0,0,1,0,0,1,1]@8fps，Run [2-7]@20fps） — verify: 运行可见待机呼吸动画
- [x] 3.2 实现 A\* 寻路（8 方向，代价 10/14，octile 启发，斜走只查目标格），归 map 域（`map/utils/pathfinding.rs`） — verify: 单元测试——绕水池路径存在且代价合理；不可达目标返回 None
- [x] 3.3 点击系统（光标 → 世界坐标 → 格子，不可通行忽略）+ 逐格补间移动（TILE_TIME=0.15s × 步长）+ 朝向翻转；点击→移动→动画→相机按 `.chain()` 排序 — verify: 点击远处地板，主角绕水池逐格到达；点击水/墙不动

## 4. 地形动画（配置化，map 域）

- [x] 4.1 地形动画配置表（TerrainAnimSpec：贴图、帧偏移、fps、透明度范围、TileKind、z 层）+ 通用 offset_frames 纯函数；水为第一个条目（water0.png，偏移 [0,8,16,24]，4fps，alpha 0.8–1.0，z=1），生成水面覆盖 chunk — verify: 运行可见水面铺在池上
- [x] 4.2 通用地形动画系统（TerrainAnim 组件 + 一个系统驱动所有动画地形 chunk：帧循环 + alpha 脉动） — verify: 运行可见水面流动

## 5. 相机（camera/ 域）

- [x] 5.1 相机跟随主角，主角始终居中 — verify: 主角任意位置都居中；贴边时房间外显示背景色

## 6. 结构重构（域目录规范）

- [x] 6.1 域目录规范落地：侧面用子目录（components/resources/entities/systems/utils/constants/），mod.rs 只写 mod 声明和 register；跨文件复用的枚举/常量归 constants/，单处使用的随归属类型同文件；水面归配置化地形动画（无子域）；抽出 movement 域（Path、follow_path）和 animation 域（AnimState、AnimClips、AnimTimer、animate，含 flip_x）；各域 register 在 mod.rs 里，由 main.rs 装配 — verify: `cargo test` 全过、行为不变

## 7. 集成验证

- [x] 7.1 `cargo run` 端到端人工验证：滚屏、点击移动、绕水寻路、idle/run 切换、水面流动、层序、主角始终居中全部符合 specs — verify: 对照 5 份 spec 的场景逐条人工确认
