# Proposal: add-first-monster

## Why

游戏目前只有主角一个生物。本 change 引入第一只怪物，打通"怪物种类词表 → 地图出生数据 → 形象绑定 → 外观呈现"的完整管线：结构面向终局（后续怪物种类、AI、战斗均沿此管线生长），本期内容仅为两只在测试房间里显示并播放 idle 动画的老鼠——不移动、不攻击、不占位、无碰撞，主角可穿行。

## What Changes

- **core 新增 monster 域**：怪物词表数据文件（`data/core/monsters.ron`，词条式，本期词条仅含 id）、`MonsterRegistry` 资源、`MonsterIndex` 组件、`spawn_monsters` 启动系统（进入 Game 状态时按地图出生表生成怪物实体，仅携带种类句柄与位置，是纯游戏数据）。
- **地图文件携带怪物出生段**：固定地图数据文件可声明怪物出生条目（怪物 id + 格子坐标）；加载期校验坐标越界，未知怪物 id 在出生时报错（map 域不依赖 monster 域，出生条目以原始 id 存储、由 monster 域在出生时解析）。test_room.ron 出生两只老鼠，格子 (28,10) 与 (24,13)。
- **display 新增 monster 域**：怪物形象绑定数据文件（`data/graphic/monster_figures.ron`，monster → figure，加载期闭合校验）、`MonsterFigureRegistry` 资源、`attach_figure` 系统（Attach 相，为怪物实体解析绑定并挂载形象句柄）。形象句柄到外观的解析由现有 appearance 管线承担，不新增机制。
- **词表与外观数据**：figures.ron 增加 `giant_white_rat`；appearances.ron 增加对应外观条目（16×15 帧、16 列×2 行图集、idle 帧表 `[0,0,0,1]` @ 2fps，仅 idle 一种动画）；新增 rat.png 素材（原型期复制自参考素材，许可 GPLv3）。
- **群体 idle 去同步**：生物挂载外观时按出生格坐标派生动画相位偏移（`(x+y) % idle 帧数`），同帧出生的多只同种怪物 idle 相位错开；不新增依赖。

明确不做：怪物移动、攻击、AI、占位与碰撞（主角可穿行）、对怪物的点击拾取、血条；hero 域及其形象绑定方式不动；怪物的 run/attack/die 帧号仅记入 design.md 备查，不进数据文件。

## Capabilities

### New Capabilities

- `monster`: 怪物种类的词表注册（core 数据文件声明 id、加载分配运行时句柄）、地图出生数据的语义（进入游戏时按声明出生为纯数据实体、未知 id 报错）、怪物到形象的绑定（display 数据文件、加载期闭合校验）、怪物的呈现与占位语义（播放 idle、群体相位错开、不阻碍移动）。

### Modified Capabilities

- `grid-map`: 地图数据文件扩展——可声明怪物出生条目（怪物 id + 格子坐标），坐标越界在加载期报错。

## Impact

- **新增代码**：`src/core/monster/`（components/monster_index.rs、resources/monster_registry.rs、types/monster_entry.rs、entities/monsters.rs、mod.rs）；`src/frontend/display/monster/`（resources/monster_figure_registry.rs、types/monster_figure_entry.rs、systems/attach_figure.rs、mod.rs）。
- **修改代码**：`src/core/map/`（MapFile、LocalMap 携带出生段，新增 types/monster_spawn.rs，解析期越界校验）；`src/frontend/display/appearance/systems/attach_appearance.rs`（动画相位由出生格坐标派生）；两个侧根 register 各加一行（src/core/mod.rs、src/frontend/display/mod.rs）。
- **数据与素材**：新增 data/core/monsters.ron、data/graphic/monster_figures.ron、assets/rat.png；修改 data/maps/test_room.ron、data/core/figures.ron、data/graphic/appearances.ron。
- **依赖**：不新增 crate 依赖；Bevy 钉住 0.19.x 不变。
- **兼容性**：地图文件的 monsters 段以 serde default 兼容缺省；无对外 API。
