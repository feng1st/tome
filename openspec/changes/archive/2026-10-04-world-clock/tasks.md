历史归档，不符合先英文后中文规范，请勿参考

# Tasks: world-clock

依赖顺序：速率表 → 世界时钟 → 主角步进 → 画面层 → 怪物行动 → 集成。

## 1. speed 域与速率表

- [x] 1.1 新建 `src/core/speed/` 域：`SPEED_RATE_TABLE`（300 项原表）、`STANDARD_SPEED`、`STANDARD_SPEED_RATE`（从表推导）、`STANDARD_ACTION_DURATION`、`action_duration`（整数四舍五入，下限 20 tick）。verify：`cargo test` 通过——landmark（110→100、125→40、100→200）、四舍五入、全表下限、单调不减与高速饱和
- [x] 1.2 `Speed` 组件；怪物词表 speed 在出生期解析入组件，缺失或越界拒绝启动。verify：`cargo test` 通过解析与校验用例（缺字段、越界两条拒绝启动）

## 2. 世界时钟

- [x] 2.1 新建 `src/core/world_clock/` 域：`WorldClock` 资源、`NextTurn` 与 `WorldDriver` 组件、`advance` 系统（画面冻结、扫到最近回合、泊车、不倒退）。verify：`cargo test` 通过——画面冻结时钟、扫到最近回合、未花费回合泊车、空世界静止
- [x] 2.2 `CorePhase` 链为 `Advance → Command → PlayerPlan → PlayerAct → WorldPlan → WorldAct`；编排只在 `core/mod.rs` 根模块，域 register 只做成员注册。verify：`cargo check --all-targets` 通过且域 register 内无编排代码

## 3. 主角步进

- [x] 3.1 `CellCoord` 挪入 `map/components/` 承担逻辑位组件职责；呈现位置归前端 `CurrPosition`。verify：`cargo test` 通过出生测试（主角、怪物出生携带整格 `CellCoord`）
- [x] 3.2 `plan_move`（到期 + 全静止 + 有路径 → 产 `Move` 并消耗回合）与 `act_move`（翻逻辑格、弹路径头、消耗 `Move`；路径头不变量有 debug_assert 护栏）。verify：`cargo test` 通过——到期规划、未到期不动、全局静止门、无路径不规划、翻格弹格
- [x] 3.3 命令执行 `execute` 独在 `Command` 阶段：验证与寻路先于规划落盘。verify：`cargo test` 通过——改目标落在规划帧时沿新路径走出、每跳相邻（`retarget_on_the_planning_frame_follows_the_new_path`）

## 4. 画面层

- [x] 4.1 协议域 `src/core/display/`：`IsMoving` 组件（display 写、core 读）。verify：`cargo check --all-targets` 通过，类型依赖无环
- [x] 4.2 前端 motion 域：`CurrPosition` 与 `move_sprite`——每格 0.1s 匀速、直走斜走同时长、预测式放行（提前一帧摘旗）。verify：`cargo test` 通过——直走斜走同时长、旗标跟踪、提前一帧摘旗、改目标不停顿、静止无旗
- [x] 4.3 `sync_animation` 以路径或位置差为行走判据并翻转朝向。verify：`cargo test` 通过既有动画同步测试

## 5. 怪物随机移动

- [x] 5.1 引入 `rand` crate；`plan_wander`：启动门、自身旗标门、75/25、至多四次独立重选、全堵原地、回合照常消耗。verify：`cargo test` 通过——未启动不规划、到期消耗回合、未到期不动、自身画面在动不动、全堵原地耗回合
- [x] 5.2 `monsters.ron` 老鼠条目声明 `speed: 110`。verify：启动进入测试房间，两只老鼠正常呈现

## 6. 集成与收尾

- [x] 6.1 端到端：主角 3 步泊车于第 300 tick，老鼠（速度 125）恰好完成 8 个回合、下一回合排在第 320 tick。verify：`cargo test` 通过 `the_world_parks_with_a_ready_driver`
- [x] 6.2 命名清剿（hero→player 等）与 `OPEN_ISSUES` 更新：原条目 1 删除、race 词表拆分登记为条目 2。verify：grep 无旧词残留，条目表更新
- [ ] 6.3 可观察行为：主角移动时老鼠随机移动、主角停下世界泊车、idle 照播。verify：`cargo run` 人工观察确认
- [x] 6.4 全绿零警告。verify：`cargo +nightly fmt --check`、`cargo clippy --all-targets`、`cargo check --all-targets`、`cargo test` 四条全绿
