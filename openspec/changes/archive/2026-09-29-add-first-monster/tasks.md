# Tasks: add-first-monster

## 1. 素材与形象数据

- [x] 1.1 复制 `.ref/pixel-dungeon/assets/rat.png` 至 `assets/rat.png`；`data/core/figures.ron` 增加 `"giant_white_rat"`；`data/graphic/appearances.ron` 增加对应条目（rat.png、16×15 帧、16 列×2 行、第二行灰白变体 idle `[16,16,16,17]` @ 2.0，仅 idle）。verify：`cargo test` 全绿（appearance_registry 的 spec-alignment 测试新增 giant_white_rat 条目数据断言：贴图、帧尺寸、行列数、idle 帧表与帧率）；`cargo run` 启动正常（词表与外观闭合校验通过）

## 2. core：出生数据与 monster 域

- [x] 2.1 `CellCoord` derive `Deserialize`；新增 `core/map/types/monster_spawn_entry.rs`（MonsterSpawnEntry，serde 布局）与 `core/map/types/monster_spawn.rs`（MonsterSpawn，内存类型）。verify：`cargo check --all-targets` 通过
- [x] 2.2 `MapFile` 增加 `monsters: Vec<MonsterSpawnEntry>`（serde default 兼容缺省）；`LocalMap` 增加 `spawns: Vec<MonsterSpawn>`；`parse_local_map` 解析出生条目并校验坐标越界（panic 指明地图文件与位置）。verify：current_map 解析单测通过——出生条目解析、未声明时为空、越界 panic；既有测试全绿
- [x] 2.3 新增 `core/monster/` 域：components/monster_index.rs（MonsterIndex）、types/monster_entry.rs（MonsterEntry）、resources/monster_registry.rs（MonsterRegistry，对齐 FigureRegistry 形状）、mod.rs（register：init_resource）；新增 `data/core/monsters.ron` 声明 `[ ( monster: "giant_white_rat" ) ]`。verify：词表单测通过（按名换句柄、句柄同 id 相等异 id 不等、重复 id 与空 id panic）；spec-alignment 测试通过（真实文件含 giant_white_rat）
- [x] 2.4 新增 `core/monster/entities/monsters.rs`（spawn_monsters：读 `CurrentMap.spawns`，经 MonsterRegistry 解析 id，未知 id panic 指明 id 与格子，生成 `(MonsterIndex, Position)`）；core/mod.rs 注册 monster 域。verify：`cargo test` 全绿不回归

## 3. display：monster 绑定域

- [x] 3.1 新增 `frontend/display/monster/` 域：types/monster_figure_entry.rs（MonsterFigureEntry）、resources/monster_figure_registry.rs（MonsterFigureRegistry，`HashMap<MonsterIndex, FigureIndex>`，FromWorld 拉取两个词表，闭合校验：未知怪物 id、未知形象 id、重复绑定、缺少绑定均 panic 指明文件与位置）、mod.rs（register：init_resource）；新增 `data/graphic/monster_figures.ron` 绑定 `( monster: "giant_white_rat", figure: "giant_white_rat" )`。verify：绑定解析单测通过（含四种校验 panic）；spec-alignment 测试通过（真实文件绑定指向正确）
- [x] 3.2 新增 `frontend/display/monster/systems/attach_figure.rs`（对 `Added<MonsterIndex>` 查绑定插 `FigureIndex`，文档注释说明与 attach_appearance 同相无序、最坏晚一帧上屏）；display/mod.rs 注册 monster 域。verify：`cargo test` 全绿不回归

## 4. 去同步与房间内容

- [x] 4.1 `attach_appearance` 挂载的 `AnimState.frame_offset` 改为按出生格坐标派生 `(cell.x + cell.y) as usize % idle 帧数`（更新注释：群体去同步由出生格坐标确定性派生）。verify：`cargo test` 全绿不回归
- [x] 4.2 `data/maps/test_room.ron` 增加 monsters 段：`( monster: "giant_white_rat", cell: ( x: 28, y: 10 ) )` 与 `( monster: "giant_white_rat", cell: ( x: 24, y: 13 ) )`。verify：current_map 的 spec-alignment 断言扩展通过（真实文件含两条出生条目、坐标正确）

## 5. 集成验收

- [x] 5.1 全量验证。verify：`cargo +nightly fmt --check`、`cargo clippy --all-targets`、`cargo check --all-targets`、`cargo test` 全绿零警告；`cargo run` 启动后测试房间 (28,10) 与 (24,13) 各呈现一只播放 idle 动画的老鼠且相位错开；点击任一老鼠所在格，主角正常寻路走过去并停留在该格（穿行）
