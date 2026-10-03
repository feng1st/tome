# Tasks: world-clock

design.md 的 Open Questions 均不影响本任务清单（PD 式步开始翻转、长动作剩余量发布都属后续立项）。依赖顺序：速率表 → 双层位置 → 世界时钟 → 怪物行动 → 集成。

## 1. 速率表与行动点基础

- [x] 1.1 新建 `src/core/time/` 域骨架与 `constants/action_point_rate.rs`（300 项原表 + 查表函数）。verify：`cargo test` 通过表值单测（E(110)=10、E(120)=20、E(100)=5、高速饱和 49、单调不减；全表由脚本从 tome2 `src/tables.c:1124` 提取）
- [x] 1.2 新增 `Speed`、`ActionPoints`、`ClockDriver` 组件并在 time 域注册。verify：`cargo check --all-targets` 通过，组件定义带 doc comment

## 2. 双层位置

- [x] 2.1 `CellCoord` 承担组件职责（自 map/types 挪入 movement/components/，map 域继续用同一类型做纯值）；主角与怪物出生时挂载。verify：`cargo test` 通过出生测试（主角、怪物实体均带整格 `CellCoord` 与浮点 `Position`，初值为出生格）
- [x] 2.2 `follow_path` 维护主角双层位置：步完成时翻转 `CellCoord`。verify：`cargo test` 通过单测——步完成翻转、步进途中保持出发格、一帧跨多步每步各翻一次

## 3. 世界时钟

- [x] 3.1 `WorldClock` 资源：`follow_path` 对带 `ClockDriver` 的实体发布每帧累积量与路径剩余量。verify：`cargo test` 通过——主角步进时帧进度非零、路径走完后归零
- [x] 3.2 time 域 `accumulate` 系统：按速率比值同步累积到每只怪物。verify：`cargo test` 通过——主角一帧累积 10 点则标准速度怪物 +10、2.5 倍速度 +25；主角不动则零累积
- [x] 3.3 `CorePhase` 链扩为 `Sense → Plan → Act → Settle → React → Present`；排序事实只出现在 `core/mod.rs` 根模块。verify：`cargo check` 通过且域 register 内无编排代码

## 4. 位置推导

- [x] 4.1 新增 `Heading` 承诺格组件与 `interpolate` 系统（Present 阶段）：呈现位 = 逻辑位 + 进度插值，无承诺格则静止于格心。verify：`cargo test` 通过——40 点在 40% 处、无承诺格停在格心
- [x] 4.2 `sync_animation` 读 `Heading` 切换走/停动画与朝向。verify：`cargo test` 通过既有动画同步测试

## 5. 怪物随机移动

- [x] 5.1 引入 `rand` crate 与 `src/core/random/` 域；随机源为资源，测试可注入固定种子。verify：`cargo test` 通过——同种子两次运行序列一致
- [x] 5.2 `monsters.ron` 条目增 `speed` 字段：解析、出生期入组件；缺失或超出速率表范围拒绝启动。verify：`cargo test` 通过解析与校验测试（含缺字段、越界两条拒绝启动用例）
- [x] 5.3 怪物出生挂载 `Speed` 与 `ActionPoints` 组件；主角出生挂载 `ClockDriver`。verify：`cargo test` 通过出生测试
- [x] 5.4 monster 域 `wander` 系统（入 `CorePhase::React`）：行动边界先完成当前承诺（逻辑位翻转），再决策——25% 随机选 8 向之一、可通行则承诺、75% 原地不动；预判不足不发起；冻结不决策。verify：`cargo test` 通过——固定种子序列确定、撞墙原地不动、预判不发起、冻结不决策
- [x] 5.5 `monsters.ron` 老鼠条目声明 `speed: 110`。verify：启动进入测试房间，两只老鼠正常呈现

## 6. 集成与收尾

- [x] 6.1 集成测试：主角 10 格长路径 + 2.5 倍速老鼠——账目守衡（赚到 = 行动 + 余数）；主角停下的瞬间世界冻结，老鼠停在整格上只持余数。verify：`cargo test` 通过
- [ ] 6.2 可观察行为：主角移动时老鼠随机移动、主角停下老鼠立即停在整格、冻结帧不再出现怪味滑动、idle 照播。verify：`cargo run` 人工观察确认
- [x] 6.3 删除 OPEN_ISSUES 原条目 1（渲染侧已由直出管线解决、设计侧由本模型兑现并落入 design.md）。verify：文件更新，条目移除并重编号
- [x] 6.4 全绿零警告。verify：`cargo +nightly fmt --check`、`cargo clippy --all-targets`、`cargo check --all-targets`、`cargo test` 四条全绿
