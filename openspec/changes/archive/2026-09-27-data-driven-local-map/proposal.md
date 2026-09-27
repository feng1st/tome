# Proposal: data-driven-local-map

## Why

房间地图的宽高、内容、地形类型目前全部硬编码在代码里：地形是枚举，新增一种地形要改动多个既有文件，违背开闭原则；地图内容离开代码就无法编辑。后续的多地图（野外/城镇/地牢）、随机生成、地图切换都建立在"地图与地形是数据"这一前提上，现在是打地基的时机。

## What Changes

- 地形类型由数据注册表定义：数据文件描述每个地形的字符串 id 与标志位（PASSABLE / LIQUID），代码中只有位标志常量与 `TerrainIndex(u16)` 运行时句柄，不存在地形枚举，逻辑只查询标志位、不判断地形身份。
- 房间地图由固定数据文件描述：legend（字符到地形 id 的映射表）+ rows（字符逐行铺图），地图宽高从文件内容得出。启动时写死加载测试房间，其内容复刻现有演示房间（48×32、四周墙体封闭、中央水池），运行表现无可见变化。
- 地图数据结构命名为 `LocalMap`，为将来的 WorldMap（格语义与移动机制不同的平行域）预留命名位置。
- 显示侧的渲染素材由两个配置文件描述：图集定义（文件名即 id、帧像素尺寸、行列数）与地形画法绑定（地形 id → 图集 + tile_index，autotile 为 true 时 tile_index 是 16 帧岸线组的首帧）。
- 岸线拼接规则由标志位推导：邻居不可走或为液体时不画岸线，与现有画面表现等价。
- 集合型资源命名统一为 Registry 后缀：既有的 `Appearances` 更名为 `AppearanceRegistry`（纯更名，无行为变化）。
- 数据文件格式采用 RON，经 serde 反序列化；新增直接依赖 `serde`、`ron`、`bitflags`。
- 不在本次范围：WorldMap、随机地图生成、前景物品、地图切换与持久化、Bevy Asset 管线（本次用文件系统同步读取，数据校验失败即 panic）。

## Capabilities

### New Capabilities

无。

### Modified Capabilities

- `grid-map`: 能力的定义从"代码硬编码的房间"演进为"数据驱动的 LocalMap"。需求层面：地形类型与可通行等属性由数据注册表定义（标志位）；地图由固定数据文件加载（legend + rows）；测试房间复刻原演示房间的空间性质（墙体封闭、水池阻断直线路径）。

## Impact

- **代码**：`core/map` 域（`GridMap` 更名为 `LocalMap`、宽高常量删除、地形枚举删除、新增地形注册表资源与数据加载）；`frontend/display/map`（写死的 tileset 帧号常量由地形画法绑定取代，chunk 构建与岸线拼接改走绑定与标志位）；`frontend/display/appearance`（`Appearances` 更名为 `AppearanceRegistry`）。
- **依赖**：`Cargo.toml` 新增 `serde`、`ron`、`bitflags`。
- **数据**：新增 `data/` 顶层目录（与 `assets/` 平级）：`data/core/terrains.ron`、`data/graphic/tilesets.ron`、`data/graphic/terrain_tiles.ron`、`data/maps/test_room.ron`。
- **行为**：无可见变化；启动后直接进入的测试房间与现有演示房间在空间性质上完全一致。
- **其他 spec**：`rendering`、`hero-movement`、`camera` 等描述可观察行为的需求均不受影响。
