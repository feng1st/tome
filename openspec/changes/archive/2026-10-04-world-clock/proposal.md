历史归档，不符合先英文后中文规范，请勿参考

# Proposal: world-clock

## Why

游戏目前没有世界时钟：主角实时自由移动，怪物出生即定格——没有速度、没有行动频率、没有时间概念，怪物行动、战斗、状态等一切战术机制都无依附。词表数值要对齐 tome2（r_info 的 speed 等字段以时钟为前提），OPEN_ISSUES 预登记的"速度 = 行动频率"准则也在等这块地基落地。

## What Changes

- 引入**整数回合槽世界时钟**：每只生物持有持久的回合槽（下一回合时刻）；每帧帧首，任何生物的画面在移动则时钟保持，否则时钟推进到最近的到期回合。纯整数算术，无累积误差。
- **时长定价**：行动时长 = 标准时长 × 标准速率 ÷ 查表速率；速率表 300 项与 tome2 `extract_energy` 逐项一致，词表 speed 直写索引原值（110 = 标准）。速度只影响行动频率；画面步速恒定每格 0.1s、方向无关。
- **世界等画面、主角泊车**：`IsMoving` 协议组件（display 写、core 读）门控时钟与主角规划；主角无路径则回合不消耗，其未花费的未来回合成为最近回合，世界停在其上等输入；冻结只冻世界逻辑，idle 动画照播。
- **命令先于规划落盘**：`CorePhase` 链为 `Advance → Command → PlayerPlan → PlayerAct → WorldPlan → WorldAct`；命令验证与寻路在 `Command` 阶段落盘，规划读到的必是已确定状态。
- **双层位置各归其侧**：逻辑位 `CellCoord`（整格，行动生效即翻转，一切判定只读它）在 core；呈现位置 `CurrPosition`（连续，恒定步速移向逻辑格）在前端。
- **老鼠成为第一个时钟消费者**：按 tome2 `RAND_25` 语义随机移动——到期回合 75% 原地、25% 至多四次独立重选方向，全堵原地，回合照常消耗。
- 词表：`monsters.ron` 条目增 `speed` 字段（老鼠 110）；主角固定标准速度，不立词表字段。不做显式"等待"动作；不做可种子化随机源（无消费场景）。

## Capabilities

### New Capabilities

- `world-clock`：门控世界时钟——回合槽与时钟推进、时长定价与速率表、冻结与泊车语义。

### Modified Capabilities

- `player-movement`：主角步进由回合驱动——到期回合规划一步，行动生效即翻转逻辑位，呈现位置以恒定步速追上；移动命令先于规划落盘。点击选目标、寻路、逐格移动的手感不变。
- `monster`：词表增 speed 字段；怪物入时钟——到期回合按 RAND_25 随机移动，带启动门与自身旗标门。"怪物不阻碍移动"等既有行为不变。

## Impact

- 新增 `src/core/world_clock/` 域（`WorldClock` 资源、`NextTurn`/`WorldDriver` 组件、`advance` 系统）与 `src/core/speed/` 域（`Speed` 组件、速率表与时长常量、`action_duration`）。
- 新增 `src/core/display/` 协议域（`IsMoving` 组件：display 写、core 读）。
- `movement` 域：新增 `Move` 组件与 `act_move` 执行器；`Path` 保留；`follow_path`、`Position`、`step_duration` 移除；`CellCoord` 挪入 `map/components/`。
- `hero` 域改名 `player`：新增 `plan_move`；命令执行器 `execute` 入 `Command` 阶段。
- 前端新增 motion 域（`CurrPosition`、`move_sprite`、位置工具函数）；`sync_animation` 判据更新为路径或位置差。
- `CorePhase`：五阶段扩为六阶段（新增 `Command`）。
- 数据：`data/core/monsters.ron` 条目增 `speed` 字段。
- 依赖：引入 `rand` crate。
- 文档：`OPEN_ISSUES` 原条目 1 删除、race 词表拆分登记；`config.yaml` 增命名与参数顺序规则。
- 终局方向（记录于 design.md）：race/class/monster 三表字段对齐 tome2 的 player_race/player_class/r_info，终值 = kind 束 + race 修正 + class 修正，出生期合成入组件；词表层不对称、运行期组件对称。
