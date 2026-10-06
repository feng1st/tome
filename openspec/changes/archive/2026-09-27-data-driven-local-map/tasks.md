历史归档，不符合先英文后中文规范，请勿参考

# Tasks: data-driven-local-map

## 1. 依赖与数据文件

- [x] 1.1 `Cargo.toml` 新增 `serde`（derive）、`ron`、`bitflags`；verify: `cargo check` 通过
- [x] 1.2 创建 `data/core/terrains.ron`（floor: PASSABLE；wall: 无标志；water: LIQUID）、`data/graphic/tilesets.ron`（tiles0.png，16×16 帧，16×16 格）、`data/graphic/terrain_tiles.ron`（floor→1、wall→4、water→48+autotile，layer 分别为 floor/wall/floor）；verify: 三个文件可被 ron 解析（随 2.2/4.1 的加载测试覆盖）
- [x] 1.3 创建 `data/maps/test_room.ron`：legend 三个字符 + rows 32 行 × 48 字符，复刻现演示房间（四周墙、中央偏下 8×5 水池）；verify: 行数与行宽符合，且与 `GridMap::demo_room()` 的坐标布局逐格一致（随 5.2 的测试覆盖）

## 2. core/map 数据模型

- [x] 2.1 新增 `constants/terrain_flags.rs`（bitflags：PASSABLE、LIQUID）、`types/terrain.rs`（Terrain、TerrainIndex）、`types/terrain_file.rs`、`types/map_file.rs`（serde 结构）；verify: `cargo check` 通过，公开类型均带 doc comment
- [x] 2.2 新增 `resources/terrain_registry.rs`、`utils/terrain_loading.rs`、`utils/map_loading.rs`：地形表加载 + 校验（重复 id、未知标志名），地图文件加载 + 校验（行宽不齐、legend 未覆盖字符、未知地形 id），报错带文件与位置；verify: 新增单测覆盖正常加载与每类非法输入的 panic 信息
- [x] 2.3 `types/grid_map.rs` 更名为 `types/local_map.rs`：`LocalMap` 格子存 `TerrainIndex`，宽高来自地图文件，`get` 越界返回 `None`，不暴露 `walkable`；verify: `cargo check` 通过
- [x] 2.4 `utils/pathfinding.rs` 与移动调用链改为三段式查询（`LocalMap.get` → `TerrainRegistry.get` → `flags.contains`），签名带 `&LocalMap + &TerrainRegistry`；verify: 现有寻路与移动单测随迁后 `cargo test` 绿
- [x] 2.5 `core/map/mod.rs` register 阶段同步加载两个 core 数据文件并插入 `TerrainRegistry` 与 `CurrentMap`；verify: `cargo test` 绿（构造 App 的测试可取到两个资源）

## 3. hero 测试随迁

- [x] 3.1 `core/hero/systems/commands/move_to_cell.rs` 测试改用内存小注册表 + `LocalMap` 构造测试地图；verify: `cargo test` 绿

## 4. display/map 数据绑定

- [x] 4.1 新增 `types/tileset_file.rs`、`types/terrain_tile_file.rs`、`constants/chunk_layer.rs`、`resources/tileset_registry.rs`（TilesetRegistry）、`resources/terrain_tile_registry.rs`（TerrainTileRegistry）、`utils/tileset_loading.rs`、`utils/terrain_tile_loading.rs`：加载与校验（帧尺寸 == TILE_SIZE、tile_index 不越界、绑定引用的地形与 tileset 存在、每个地形都有绑定、同 layer 单 tileset），register 阶段经 world 拿 AssetServer 建 Handle；verify: 新增单测覆盖正常加载与每类非法输入
- [x] 4.2 `utils/chunk_data.rs` 按 `TerrainTileRegistry` 绑定逐格路由到 layer 对应 chunk，`utils/autotile.rs` 按"邻居不可通行或 LIQUID 则置位"规则算掩码；verify: 改写后的 chunk_data/autotile 单测断言与现状逐值等价（帧号、掩码、透明开阔水格）
- [x] 4.3 `entities/chunks.rs` 的 tileset 句柄改从 `TilesetRegistry` 取；`constants/layout.rs` 删除 `TILE_FLOOR/TILE_WALL/TILE_SHORE_BASE`；verify: `cargo check` 通过且 grep 无残留引用
- [x] 4.4 appearance 域更名：`Appearances` → `AppearanceRegistry`，资源文件 `resources/appearances.rs` → `resources/appearance_registry.rs`，全部引用点随迁；verify: `grep -rn "Appearances" src/` 仅剩 `AppearanceRegistry` 命中，`cargo test` 绿

## 5. 清理与验收

- [x] 5.1 删除 `core/map/constants/tile_kind.rs` 与 `core/map/constants/layout.rs`；verify: `grep -rn "TileKind\|MAP_W\|MAP_H" src/` 无残留
- [x] 5.2 新增规格对齐测试：读取仓库真实的 `data/core/terrains.ron` 与 `data/maps/test_room.ron`，断言 48×32、四周不可通行封闭、中央水池阻断直线路径且存在绕行路径、`HERO_START` 格可通行；verify: `cargo test` 绿
- [x] 5.3 全绿验收：`cargo +nightly fmt --check`、`cargo clippy --all-targets`、`cargo check --all-targets`、`cargo test` 全部零警告通过
- [x] 5.4 运行验证：`cargo run` 后画面与操作手感同基线（围墙边房间、中央水池岸线与滚动水面、点击寻路绕行水池、相机跟随）；verify: 人工确认无可见差异（墙改绑 ground 后不遮挡英雄为 6.13 的预期内差异）

## 6. 评审修正

- [x] 6.1 get 纪律统一：三个注册表 `get` 改返回 `Option`（运行时不变量处 `expect`）、删 find 分工、`TerrainRegistry::id` → `get_index`、`Terrain` 增加 `walkable()`/`liquid()` 查询方法、删除 `utils/cell_queries.rs`（三段式组合就地闭包/内联）；verify: `cargo test` 绿
- [x] 6.2 tileset 下沉为独立域 `frontend/display/tileset/`（types/resources/utils 齐备），display 根先 register tileset 再 register map；`TilesetEntry` 删 `tile_width/tile_height`（帧尺寸==TILE_SIZE 从校验变定义，loader 直接用常量）；verify: `cargo test` 绿
- [x] 6.3 值类型归位：`TerrainTile`/`Tileset`/`ChunkData` 落 `types/`，serde 结构文件随结构名（`terrain_entry.rs`/`tileset_entry.rs`/`terrain_tile_entry.rs`）；verify: `cargo check` 通过
- [x] 6.4 `TILE_SIZE` 与 `LAYER_*` 上移 `frontend/display/constants/layout.rs`（跨域共享常量的归属），全部引用点改道；verify: `grep -rn "map::constants::layout" src/` 无残留
- [x] 6.5 chunk 动态分组：`build_chunk_data` 按 `(layer, tileset, alpha)` 聚合（删除同 layer 单 tileset 校验与 `layer_tilesets` 聚合），`TerrainTile.tileset` 恢复，alpha 配置化进绑定（`alpha: blend`，默认 opaque），`AlphaMode` 常量入 `display/map/constants/`；verify: 分组单测含混图集多 chunk 用例
- [x] 6.6 命名随迁：加载文件 `*_registry_loading.rs`、`load_appearances` → `load_appearance_registry`、`autotile_index` → `shore_index`、参数名取类型完整蛇形名（`terrain_registry`/`current_map`/`local_map`）；verify: grep 无旧名残留
- [x] 6.7 `AppearanceRegistry` 加载时机与其他注册表统一（删 `OnEnter(Game)` 构建系统），appearance 域文档语病修正；verify: `cargo test` 绿
- [x] 6.8 三条命名/归属原则（get 纪律、值类型落 types、参数完整蛇形名）落入 `openspec/config.yaml`；verify: 文件包含三条
- [x] 6.9 返修后全绿验收：`cargo +nightly fmt --check`、`cargo clippy --all-targets`、`cargo check --all-targets`、`cargo test` 全部零警告通过
- [x] 6.10 加载形态定稿：五个资源（四个注册表 + `CurrentMap`）全部走 `FromWorld` + `World::get_resource_or_init` 递归拉取（构造依赖非时序依赖，域 register 只 `init_resource` 一行，注册顺序无关）；`OnEnter(Game)` 全为消费者、无需排序设施（曾引入的 Startup+sets 与 GameEntry 编排均已删）；verify: `cargo test` 绿且四件套零警告
- [x] 6.11 加载逻辑与注册表一体一文件：删 `utils/*_registry_loading.rs` 与 `load_*` 组合层（不做函数套函数），`FromWorld`（fs + 装配）与 `parse_*`/`build_*`（校验核心、测试接缝）及单测全部并入 resource 文件；appearance 与 tileset 域的 utils/ 目录随之删除；verify: `cargo test` 绿且 grep 无 `*_loading` 残留
- [x] 6.12 `TerrainId` → `TerrainIndex` 并独立成文件 `types/terrain_index.rs`；词族随迁：`get_id` → `get_index`、句柄参数/闭包/变量 `id` → `terrain_index`（文件边界的字符串 id 保留）、测试辅助命名对齐完整蛇形纪律（`registry()`/`map`/`registry` → `terrain_registry()`/`local_map`/`terrain_registry`、`tiles` → `terrain_tile_registry`）；verify: `cargo test` 绿且四件套零警告
- [x] 6.13 `ChunkLayer` 按高度角色更名 `Floor/Wall` → `Ground/Overhead`（z 常量 `LAYER_SCROLL/FLOOR/WALL` → `LAYER_UNDERGROUND/GROUND/OVERHEAD`）；测试房间全部地形绑定 `ground`（墙读作墙根，不遮挡角色）；verify: `cargo test` 绿且 grep 无 `LAYER_FLOOR`/`ChunkLayer::Floor` 残留
- [x] 6.14 从严审查修正：分组键序 `(tileset, alpha, layer)` 注释三处同步；`Tileset` doc 删除已死的"加载时校验帧尺寸"措辞；`parse_tilesets` → `parse_tileset_entries`（同族按产物命名）；`attach_appearance`/`anim_layers` 参数与局部变量对齐完整蛇形纪律；`build_chunk_data` 分组键改 `&str` 借用（消除逐格 String clone）；design D5 图悬空节点 L3 修复；verify: `cargo test` 绿且四件套零警告
