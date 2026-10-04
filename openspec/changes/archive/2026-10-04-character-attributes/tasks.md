# Tasks: character-attributes

依赖顺序：race/class 域 → stats 域 → health 域 → 出生接入 → 身份拆分 → 数据与集成。

## 1. race 与 class 域

- [x] 1.1 新建 `core/race/` 与 `core/class/` 域：词表条目加 `stat_modifiers: [i32; 6]`、`hit_die: u16`；注册表升级为保留条目（`RaceKind`/`ClassKind` + `race_kind`/`class_kind` 访问器，仿 `MonsterRegistry`）。verify：`cargo test` 通过——按名换取句柄与条目、新增条目不改代码
- [x] 1.2 加载校验：id 重复/为空、`stat_modifiers` 长度非六、`hit_die` 非正均拒绝启动。verify：`cargo test` 通过四条拒绝启动用例，错误信息指明文件与位置
- [x] 1.3 `RaceIndex`/`ClassIndex` 组件迁入新域；`core/creature` 的 race/class 注册表与 `player`/`monster`/display 的引用改到新路径。verify：`cargo check --all-targets` 通过且行为不变（`cargo test` 既有用例仍绿）

## 2. stats 域

- [x] 2.1 新建 `core/stats/`：`Stat` 枚举（Strength/Intelligence/Wisdom/Dexterity/Constitution/Charisma）与 `Stats { max, current }` 组件。verify：`cargo test` 通过——六维可按六个固定维度各自取得 max 与 current
- [x] 2.2 掷点纯函数（每维 `5 + d3 + d4 + d5`、总和 42–57 否则整组重掷）与修正并入纯函数（`stat_modifiers` 相加，18 以上分段非线性，默认路径带随机步长）。verify：`cargo test` 通过——每维 8–17、总和 42–57、正修正不低于基准加修正、负修正不高于基准且下限 3（区间不变量）
- [x] 2.3 `constitution_hp_bonus` 38 项加成表（128 为基线）与按体质查表。verify：`cargo test` 通过——landmark（3→−5、18→+3、18/100→+10、18/220+→+27）与表长 38

## 3. health 域

- [x] 3.1 新建 `core/health/`：`HitPoints { current, max }` 组件。verify：`cargo test` 通过——组件承载 current 与 max
- [x] 3.2 生命派生纯函数：`max = hit_die + constitution_hp_bonus(con) / 2`（整数除法，`hit_die` 为 race 与 class 生命骰之和）。verify：`cargo test` 通过——上限为体质与生命骰的确定函数

## 4. 出生接入

- [x] 4.1 `spawn_player` 串起出生：掷点、并入 race/class 修正写 `Stats`，派生写 `HitPoints`（`current = max`）。verify：`cargo test` 通过——主角出生携带 `Stats`（各维落在掷点与修正可达区间）与 `HitPoints`（`current == max`、max 为体质与生命骰的确定函数）

## 5. 身份拆分

- [x] 5.1 `monsters.ron` 条目与 `MonsterKind` 减字段（去 race/class/unique_id）；`spawn_monsters` 不再挂 race/class/unique 句柄。verify：`cargo test` 通过——出生实体携带怪物句柄、速度、格坐标、回合槽，不携带 race/class/unique 句柄
- [x] 5.2 figure 绑定键加 `monster` 形态、去 unique 键，优先级 monsterId > race+class > race；注册表加 `by_monster`；attach 拆两系统（怪物 `Added<MonsterIndex>`、人形 `Added<RaceIndex>`）。verify：`cargo test` 通过——monster 键解析、组合键优先、race 默认回退、race/monster 无绑定拒绝启动、未命中挂载失败
- [x] 5.3 unique 整套移除（`UniqueRegistry`/`UniqueIndex`/`unique_id_entry` 与出生聚合）；`core/creature` 解散。verify：`cargo check --all-targets` 通过且 grep 无 `Unique`/`core::creature` 残留

## 6. 数据与集成

- [x] 6.1 数据文件：`races.ron`/`classes.ron` 加 `stat_modifiers`/`hit_die`（human [0,0,0,0,0,0]/10、warrior [5,-2,-2,2,2,-1]/9）；`monsters.ron` 减为 `( monster: "giant_white_rat", speed: 110 )`；`creature_figures.ron` 老鼠条目换 `monster` 键。verify：`cargo test` 的 spec 对齐用例通过（四张词表数值逐值一致）
- [x] 6.2 端到端：启动进测试房间，主角带 `Stats`/`HitPoints`、呈现 warrior 形象，两只老鼠按 kind 键呈现且起始帧错开。verify：`cargo run` 人工观察确认
- [x] 6.3 `OPEN_ISSUES` 更新：#2 裁决登记（race 收缩为可被扮演的种族、怪物以 kind 为身份、unique 移除）。verify：条目表更新
- [x] 6.4 全绿零警告。verify：`cargo +nightly fmt --check`、`cargo clippy --all-targets`、`cargo check --all-targets`、`cargo test` 四条全绿
