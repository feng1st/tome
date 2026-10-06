# Tasks: monster-combat-profile

## 1. dice 域

- [x] 1.1 新建 `src/core/dice/`：`types/dice.rs`（`Dice { n: i32, m: i32 }`）、`utils/parse.rs`（"NdM" 严格解析 → `Result<Dice, String>`：小写 d、N ≥ 1、M ≥ 1、无多余字符）、`utils/roll.rs`（`roll_with(dice, rng: &mut StdRng) -> i32`：N 个 1..=M 求和）；`src/core/mod.rs` 声明模块（纯函数域，无资源无系统，不设 register）。verify: `cargo test dice` ——合法骰式解析正确；"0d2"、"2d0"、"2D2"、"2d"、"d6"、"2d2x" 全部被拒且错误说明合法形态；值域断言 N..=N×M；同种子两次掷骰结果相等

## 2. rng 域与消费者改造

- [x] 2.1 新建 `src/core/rng/`：`GameRng` Resource 包 `StdRng`，`FromWorld` 熵播种 + `seeded(u64)` 定种构造器；`register` 内 `init_resource`；`src/core/mod.rs` 调用注册。verify: `cargo test rng` ——同种子两实例序列一致、不同种子不一致
- [x] 2.2 消费者改走集中源：`roll_base_stats` 带 `rng` 参数（`spawn_player` 以 `ResMut<GameRng>` 借出传入）；`plan_wander` 以 `ResMut<GameRng>` 替换 `rand::rng()`，行为规则不变（75% 站住、25% 随机移动）；全仓不得再出现另建随机源。verify: `cargo test` ——stats 与 monster systems 全部测试通过；`plan_wander` 新增定种测试精确断言规划结果

## 3. 怪物词表扩展

- [x] 3.1 `MonsterEntry` 增四字段（hit_points: String、armor_class: i32、level: i32、blows: Vec<MonsterBlow>，serde 层 damage 为字符串）；新建 `types/monster_blow.rs`（`MonsterBlow { damage: Dice }`）；`MonsterKind` 扩展为 speed + 解析后的 Dice、armor_class、level、blows。verify: `cargo check --all-targets` 零错误
- [x] 3.2 注册表加载期解析与校验：hit_points 与 blows 逐元素解析骰式，非法骰式 panic 且消息带文件、条目 id 与缘由；字段缺失由 serde 报错（文件、字段名与位置）——必填字段一律纯类型，不设 Option 管道。verify: `cargo test monster` ——新增 should_panic 用例（缺 hit_points/speed/blows 走格式错误、坏骰式 "2D2" 与 "0d2" 带条目 id）与既有用例全部通过

## 4. 出生接线与数据

- [x] 4.1 `spawn_monsters` 以 `ResMut<GameRng>` 掷条目生命骰，实体挂 `HitPoints { current: max, max }`。verify: `cargo test monsters` ——定种测试精确断言生命值；值域断言（2d2 → 2..=4）；当前值等于上限
- [x] 4.2 `data/core/monsters.ron` 巨白鼠条目扩为 `( monster: "giant_white_rat", speed: 110, hit_points: "2d2", armor_class: 7, level: 4, blows: [ ( damage: "1d3" ) ] )`；词表对齐测试逐值断言。verify: `cargo test` ——真实词表加载用例断言 2d2、护甲 7、等级 4、单一 1d3 伤害骰

## 5. 收口

- [x] 5.1 四绿：`cargo +nightly fmt --check`、`cargo clippy --all-targets`、`cargo check --all-targets`、`cargo test`。verify: 四条命令零警告零失败
- [x] 5.2 交付物语言自查：本变更 openspec 产物全量过语言黑名单。verify: 黑名单 grep 脚本零命中
