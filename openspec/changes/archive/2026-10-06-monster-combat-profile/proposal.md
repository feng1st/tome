# Proposal: monster-combat-profile

## Why

最小战斗闭环（OPEN_ISSUES 条目 1-6）的第一步：怪物要有生命才可被击杀。
health spec 已预留"怪物生命值上限将来由其生命骰派生，不经六维"，本条兑现它。
生命骰与伤害骰共用同一骰式形态，骰式的解析与掷骰是本条与后续战斗条目
（玩家攻击、怪物攻击）的公共基础，随本条一次落位。随机消耗由此前的各自
播种收敛为集中可播种随机源，测试可指定种子得到可复现结果。

## What Changes

- 新增 dice 域：NdM 骰式（正整数 N、M，小写 d，无其他语法）的解析与掷骰。
  解析严格：`N < 1`、`M < 1`、多余字符、大写 D 一律拒绝；词表加载期解析，
  坏骰式导致启动失败，错误信息带文件与条目 id。掷骰入口 `roll_with(dice, rng)`
  接受注入的随机源。上限函数（按旗标取骰最大值）不入本条。
- 新增 rng 域：集中可播种随机源（Resource），出生期从操作系统熵播种一次，
  正式游戏每局随机；测试可用固定种子构造，得到确定结果。现有随机消费者
  `roll_base_stats`（六维出生掷骰）与 `plan_wander`（怪物随机移动）一并改走
  集中源。随机状态落存档押后。
- 怪物词表扩展四个字段：`hit_points`（骰式字符串）、`armor_class`（整数）、
  `level`（整数）、`blows`（列表，元素为 `MonsterBlow { damage }`，本条只含
  伤害骰）。巨白鼠条目逐值补齐：生命骰 2d2、护甲 7、等级 4、伤害骰 1d3。
- 怪物出生：掷 `hit_points` 得生命上限，实体挂 `HitPoints { current: max, max }`。
- 不入本条（支撑系统不存在，见 OPEN_ISSUES 条目 7-14）：旗标（FORCE_MAXHP、
  出生速度扰动）、blows 的 method/effect 列、感知/睡眠/稀有度/掉落/经验字段。

## Capabilities

### New Capabilities

- `dice`: 骰式的词表字符串格式、掷骰语义、坏骰式拒绝。
- `rng`: 集中可播种随机源——播种时机与方式、消费者的取数方式、测试定种。

### Modified Capabilities

- `monster`: 怪物词表 requirement 扩展四字段并新增坏骰式拒绝场景；怪物出生
  requirement 增加生命值组件；巨白鼠数据 requirement 逐值扩展。
- `health`: 新增"怪物生命值派生"requirement——怪物生命上限由词表生命骰掷出，
  不经六维（对仗"玩家生命值派生"）；Purpose 中"将来"的说法随之成为现状。

## Impact

- 新域：`src/core/dice/`（types、utils，纯函数，不持 ECS 状态）、
  `src/core/rng/`（Resource 与注册）。
- 怪物域：`monster_entry.rs`（词表 serde 布局）、`monster_kind.rs` 与新增
  blow 值类型、`monster_registry.rs`（加载期解析与校验）、`monsters.rs`
  （出生挂生命组件）。
- 消费者改造：`src/core/stats/utils/roll.rs`、
  `src/core/monster/systems/plan_wander.rs` 改走集中随机源。
- 数据与装配：`data/core/monsters.ron` 扩展老鼠条目；`src/core/mod.rs`
  注册两个新域。依赖不变（`rand` 已在用，可播种生成器属其既有能力）。
- spec 侧：新增 `specs/dice/spec.md`、`specs/rng/spec.md`；修改
  `specs/monster/spec.md`、`specs/health/spec.md`。
