# Tasks: data-driven-local-map

## 1. 依赖与数据文件

- [ ] 1.1 `Cargo.toml` 新增 `serde`（derive）、`ron`、`bitflags`；verify: `cargo check` 通过
- [ ] 1.2 创建 `data/core/terrains.ron`（floor: PASSABLE；wall: 无标志；water: LIQUID）、`data/graphic/tilesets.ron`（tiles0.png，16×16 帧，16×16 格）、`data/graphic/terrain_tiles.ron`（floor→1、wall→4、water→48+autotile，layer 分别为 floor/wall/floor）；verify: 三个文件可被 ron 解析（随 2.2/4.1 的加载测试覆盖）
- [ ] 1.3 创建 `data/maps/test_room.ron`：legend 三个字符 + rows 32 行 × 48 字符，复刻现演示房间（四周墙、中央偏下 8×5 水池）；verify: 行数与行宽符合，且与 `GridMap::demo_room()` 的坐标布局逐格一致（随 5.2 的测试覆盖）

## 2. core/map 数据模型

- [ ] 2.1 新增 `constants/terrain_flags.rs`（bitflags：PASSABLE、LIQUID）、`types/terrain.rs`（Terrain、TerrainId）、`types/terrain_file.rs`、`types/map_file.rs`（serde 结构）；verify: `cargo check` 通过，公开类型均带 doc comment
- [ ] 2.2 新增 `resources/terrain_registry.rs`、`utils/terrain_loading.rs`、`utils/map_loading.rs`：地形表加载 + 校验（重复 id、未知标志名），地图文件加载 + 校验（行宽不齐、legend 未覆盖字符、未知地形 id），报错带文件与位置；verify: 新增单测覆盖正常加载与每类非法输入的 panic 信息
- [ ] 2.3 `types/grid_map.rs` 更名为 `types/local_map.rs`：`LocalMap` 格子存 `TerrainId`，宽高来自地图文件，`get` 越界返回 `None`，不暴露 `walkable`；verify: `cargo check` 通过
- [ ] 2.4 `utils/pathfinding.rs` 与移动调用链改为三段式查询（`LocalMap.get` → `TerrainRegistry.get` → `flags.contains`），签名带 `&LocalMap + &TerrainRegistry`；verify: 现有寻路与移动单测随迁后 `cargo test` 绿
- [ ] 2.5 `core/map/mod.rs` register 阶段同步加载两个 core 数据文件并插入 `TerrainRegistry` 与 `CurrentMap`；verify: `cargo test` 绿（构造 App 的测试可取到两个资源）

## 3. hero 测试随迁

- [ ] 3.1 `core/hero/systems/commands/move_to_cell.rs` 测试改用内存小注册表 + `LocalMap` 构造测试地图；verify: `cargo test` 绿

## 4. display/map 数据绑定

- [ ] 4.1 新增 `types/tileset_file.rs`、`types/terrain_tile_file.rs`、`constants/chunk_layer.rs`、`resources/tileset_registry.rs`（TilesetRegistry）、`resources/terrain_tile_registry.rs`（TerrainTileRegistry）、`utils/tileset_loading.rs`、`utils/terrain_tile_loading.rs`：加载与校验（帧尺寸 == TILE_SIZE、tile_index 不越界、绑定引用的地形与 tileset 存在、每个地形都有绑定、同 layer 单 tileset），register 阶段经 world 拿 AssetServer 建 Handle；verify: 新增单测覆盖正常加载与每类非法输入
- [ ] 4.2 `utils/chunk_data.rs` 按 `TerrainTileRegistry` 绑定逐格路由到 layer 对应 chunk，`utils/autotile.rs` 按"邻居不可通行或 LIQUID 则置位"规则算掩码；verify: 改写后的 chunk_data/autotile 单测断言与现状逐值等价（帧号、掩码、透明开阔水格）
- [ ] 4.3 `entities/chunks.rs` 的 tileset 句柄改从 `TilesetRegistry` 取；`constants/layout.rs` 删除 `TILE_FLOOR/TILE_WALL/TILE_SHORE_BASE`；verify: `cargo check` 通过且 grep 无残留引用
- [ ] 4.4 appearance 域更名：`Appearances` → `AppearanceRegistry`，资源文件 `resources/appearances.rs` → `resources/appearance_registry.rs`，全部引用点随迁；verify: `grep -rn "Appearances" src/` 仅剩 `AppearanceRegistry` 命中，`cargo test` 绿

## 5. 清理与验收

- [ ] 5.1 删除 `core/map/constants/tile_kind.rs` 与 `core/map/constants/layout.rs`；verify: `grep -rn "TileKind\|MAP_W\|MAP_H" src/` 无残留
- [ ] 5.2 新增规格对齐测试：读取仓库真实的 `data/core/terrains.ron` 与 `data/maps/test_room.ron`，断言 48×32、四周不可通行封闭、中央水池阻断直线路径且存在绕行路径、`HERO_START` 格可通行；verify: `cargo test` 绿
- [ ] 5.3 全绿验收：`cargo +nightly fmt --check`、`cargo clippy --all-targets`、`cargo check --all-targets`、`cargo test` 全部零警告通过
- [ ] 5.4 运行验证：`cargo run` 后画面与操作手感同基线（围墙边房间、中央水池岸线与滚动水面、点击寻路绕行水池、相机跟随）；verify: 人工确认无可见差异
