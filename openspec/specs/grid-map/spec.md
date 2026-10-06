# grid-map Specification | grid-map 规格

## Purpose

The cell-based map data model and the terrain registry: the tile
coordinate system, terrain types and flag attributes defined by data
files — the shared foundation of rendering, pathfinding, and movement.

定义格子化的地图数据模型与地形注册表：tile 坐标系、由数据文件定义的地
形类型与标志位属性，作为渲染、寻路和移动的共同基础。

## Requirements

### Requirement: The Cell Map Model | 格子地图模型

The game SHALL represent the local map as a fixed-width,
fixed-height two-dimensional cell array, each cell storing a terrain
reference; terrain types are defined by the data registry (see
"Terrain Data Registry"). A cell is walkable if and only if its
terrain bears the PASSABLE flag; an out-of-bounds query MUST count as
impassable.

游戏 SHALL 用固定宽高的二维格子数组表示局部地图，每格存储一个地形引
用；地形类型由数据注册表定义（见"地形数据注册表"）。某格可通行当且
仅当其地形带 PASSABLE 标志；越界查询 MUST 视为不可通行。

#### Scenario: Querying a cell's walkability | 查询格子可通行性

- **WHEN** a system asks whether a cell is walkable | 系统查询某格是否可通行时
- **THEN** the answer is walkable when the cell's terrain bears PASSABLE (such as floor), impassable when it does not (such as wall or water), and impassable for an out-of-bounds cell | 该格地形带 PASSABLE 标志（如地板）时返回可通行，不带（如墙、水）时返回不可通行，越界格返回不可通行

### Requirement: Terrain Data Registry | 地形数据注册表

Terrain kinds and their attributes SHALL be defined by a data file:
each terrain has a string id and a set of flags (PASSABLE, LIQUID).
Code MUST NOT contain a terrain enum or logic branching on concrete
terrain ids; game logic MUST query terrain attributes through the
flags only. Adding a terrain MUST take only a new data entry.

地形的种类与属性 SHALL 由数据文件定义：每个地形具有字符串 id 与一组标
志位（PASSABLE、LIQUID）。代码 MUST NOT 包含地形枚举或针对具体地形 id
的逻辑分支；游戏逻辑 MUST 只通过标志位查询地形属性。新增地形 MUST 只
需新增数据条目。

#### Scenario: Querying attributes through flags | 按标志位查询属性

- **WHEN** game logic needs to know whether a cell is walkable or is a liquid | 游戏逻辑需要判断某格是否可通行、是否为液体时
- **THEN** the answer comes from the cell terrain's flags, independent of the terrain's concrete id | 结果由该格地形的标志位给出，与地形的具体 id 无关

#### Scenario: Adding a terrain changes no code | 新增地形不改代码

- **WHEN** a new terrain entry is added to the terrain data file and referenced in a map file | 在地形数据文件中新增一个地形条目，并在地图文件中引用它时
- **THEN** the terrain loads and works without any code change | 不改动任何代码即可加载并使用该地形

### Requirement: Map Data Files | 地图数据文件

A fixed map SHALL be described by a data file: a legend table maps
characters to terrain ids, and rows draws the map's content character
by character, one row per line. The map's width MUST equal each row's
length and its height MUST equal the row count. At startup the game
SHALL load the test room data file and enter the game with it as the
current map.

固定地图 SHALL 由数据文件描述：legend 表将字符映射到地形 id，rows 以
字符逐行铺出地图内容。地图的宽 MUST 等于 rows 中各行的长度，高 MUST
等于行数。游戏启动时 SHALL 加载测试房间数据文件，并将其作为当前地图
进入游戏。

#### Scenario: Parsing a map from its file | 从文件解析地图

- **WHEN** a map file whose rows hold 32 lines of 48 characters each is loaded | 加载一个 rows 为 32 行、每行 48 个字符的地图文件时
- **THEN** the result is a 48×32 cell array, each character cell resolved through the legend into its terrain | 得到 48×32 的格子数组，每个字符格按 legend 解析为对应地形

### Requirement: Map Data Validation | 地图数据校验

Map and terrain data files MUST be validated at load. Ragged rows, a
character the legend does not cover, or a reference to a terrain id
absent from the terrain registry SHALL fail startup with an error
naming the file and the position.

地图与地形数据文件 MUST 在加载时校验。行宽不齐、出现 legend 未覆盖的
字符、引用地形注册表中不存在的 id，SHALL 导致启动失败并给出指明出错
文件与位置的错误信息。

#### Scenario: Illegal data fails startup | 非法数据拒绝启动

- **WHEN** a map file references a terrain id that does not exist in the terrain registry | 地图文件引用了地形注册表中不存在的地形 id 时
- **THEN** startup fails, the error naming the file and the position | 启动失败，错误信息指明出错文件与出错位置

### Requirement: Room Boundary | 房间边界

The test room SHALL be larger than the game window's world-pixel size,
and SHALL be sealed on all four sides by impassable terrain cells, so
the player cannot leave the room.

测试房间 SHALL 大于游戏窗口的世界像素尺寸，且四周由不可通行的地形格
封闭，主角无法离开房间。

#### Scenario: The boundary walls block | 边界墙阻挡

- **WHEN** the player attempts to move past a wall cell at the room's edge | 主角尝试以房间边缘墙格外侧为目标移动时
- **THEN** pathfinding fails or the path ends before the wall cell, and the player stays inside the room | 寻路失败或路径终止于墙格之前，主角停留在房间内

### Requirement: The Water Obstacle | 水域障碍

The test room SHALL contain an interior pool obstacle that blocks the
straight-line path between some floor cells.

测试房间 SHALL 包含一个内部水池障碍区域，使部分地板格之间的直线路径
被阻断。

#### Scenario: Walking around the pool | 绕行水池

- **WHEN** the straight line between the player's position and the target cell is blocked by the pool | 主角当前位置与目标格之间的直线被水池阻断时
- **THEN** a walkable cell path around the pool exists | 存在一条绕过水池的可通行格子路径

### Requirement: Map Monster Spawn Entries | 地图怪物出生条目

A fixed map's data file SHALL be able to declare monster spawn
entries, each consisting of a monster id and a cell coordinate; a file
declaring none means no spawns. A spawn entry's coordinates MUST fall
inside the map; an out-of-bounds entry SHALL fail startup, the error
naming the file and the position.

固定地图的数据文件 SHALL 可声明怪物出生条目：每条由怪物 id 与格子坐
标组成；文件未声明时视为无出生条目。出生条目的坐标 MUST 落在地图范围
内，越界 SHALL 导致启动失败，错误信息指明出错文件与出错位置。

#### Scenario: Parsing spawn entries from the file | 从文件解析出生条目

- **WHEN** a map file declares several monster spawn entries | 地图文件声明若干怪物出生条目时
- **THEN** each entry resolves to a monster id and a cell coordinate, loaded together with the map data | 每个条目解析为怪物 id 与格子坐标，随地图数据一同加载

#### Scenario: An out-of-bounds spawn entry fails startup | 越界出生条目拒绝启动

- **WHEN** a spawn entry's coordinates fall outside the map | 出生条目的坐标越出地图范围时
- **THEN** startup fails, the error naming the file and the position | 启动失败，错误信息指明出错文件与出错位置
