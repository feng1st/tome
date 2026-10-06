# world-clock Specification | world-clock 规格

## Purpose

The gated world clock: game time is counted in integer ticks, and
every creature holds a persistent next-turn slot (the moment of its
next turn). At each frame's head the clock holds still while any
creature's picture is moving; otherwise it sweeps to the nearest due
turn — the world's pacing never parts from its pictures. This
capability defines the next-turn slot and the clock sweep, action
pricing and the rate table, the freeze and hold semantics, and the
interleaved cadence of creatures at different speeds — the foundation
under monster actions, combat, status, and every tactical mechanic.

门控世界时钟：游戏时间以整数 tick 表达，每只生物持有持久的回合槽（下
一回合时刻）。每帧帧首，任何生物的画面在移动则时钟保持不动，否则时钟
推进到最近的到期回合——世界逻辑的节奏永远与画面一致。本能力规定回合
槽与时钟推进、时长定价与速率表、冻结与停住语义、不同速度生物的回合节
奏交错，是怪物行动、战斗、状态等一切战术机制的基础。

## Requirements

### Requirement: Next-Turn Slot and World Clock | 回合槽与世界时钟

Every creature SHALL hold a persistent next-turn slot (the moment of
its next turn, in integer ticks): planning an action advances it to
"now + the action's duration", and the slot itself is never deleted.
At each frame's head, while any creature's picture is moving the clock
SHALL hold still; otherwise the clock SHALL advance to the nearest
slot value and MUST NOT move backward. The clock is pure integer
arithmetic and MUST NOT admit real time.

每只生物 SHALL 持有持久的回合槽（下一回合时刻，整数 tick）：规划行动
时推进为"当前时刻 + 该行动时长"，槽位本身永不删除。每帧帧首，任何生
物的画面在移动时时钟 SHALL 保持不动；否则时钟 SHALL 推进到全体回合槽
的最近值，且 MUST NOT 倒退。时钟为纯整数算术，MUST NOT 引入真实时间。

#### Scenario: Moving pictures freeze the clock | 画面移动冻结时钟

- **WHEN** any creature's picture is still moving | 任一生物的画面仍在移动时
- **THEN** the clock holds still that frame | 该帧时钟保持不动

#### Scenario: Sweep to the nearest turn | 扫到最近回合

- **WHEN** no picture is moving and the creatures' next-turn slots read 33, 50, and 100 | 没有任何画面在移动，且各生物回合槽分别为 33、50、100 时
- **THEN** the clock advances to 33 | 时钟推进到 33

#### Scenario: The clock never moves backward | 时钟不倒退

- **WHEN** the clock's current value is already past every next-turn slot | 时钟当前值已大于所有回合槽时
- **THEN** the clock keeps its current value | 时钟保持当前值

### Requirement: Action Pricing and the Rate Table | 时长定价与速率表

An action's duration SHALL be "standard duration × standard rate ÷
the creature's rate", rounded half up to an integer. The rate SHALL
be looked up in the 300-entry rate table, which matches the
established reference (tome2 `extract_energy`) entry for entry, with
110 the standard speed; a vocabulary's speed field SHALL state the raw
table index, validated at load, and an out-of-range value MUST fail
startup; fitting a formula instead of looking the table up MUST NOT
happen. Speed SHALL affect only how often a creature acts; the
picture's pace MUST stay constant (0.1 seconds per cell, independent
of direction). The table's cap guarantees a shortest action duration
of 20 ticks — no action is ever free.

行动时长 SHALL 按"标准时长 × 标准速率 ÷ 该生物速率"计算，整数四舍五
入（round-half-up）。速率 SHALL 查 300 项速率表取得，表与既定参考
（tome2 `extract_energy`）逐项一致，110 = 标准速率；词表 speed 字段
SHALL 直接声明表索引原值，加载时校验，越界 MUST 拒绝启动；MUST NOT 用公
式拟合代替查表。速度 SHALL 只影响行动频率；画面步速 MUST 恒定（每格
0.1 秒，与方向无关）。速率表上限保证最短行动时长为 20 tick——任何行
动都有代价。

#### Scenario: Standard speed | 标准速度

- **WHEN** a creature at standard speed (110) plans a standard action | 标准速度（110）的生物规划一次标准行动时
- **THEN** the duration is 100 ticks | 时长为 100 tick

#### Scenario: Fast and slow | 快与慢

- **WHEN** a speed-125 creature and a speed-100 creature each plan a standard action | 速度 125 与速度 100 的生物各规划一次标准行动时
- **THEN** the durations are 40 ticks and 200 ticks | 时长分别为 40 tick 与 200 tick

#### Scenario: No action is ever free | 任何行动都有代价

- **WHEN** the standard-action duration is computed for all 300 rate-table indices | 取遍速率表全部 300 个索引计算标准行动时长时
- **THEN** every duration is at least 20 ticks | 每个时长都不小于 20 tick

#### Scenario: The picture pace is constant | 画面步速恒定

- **WHEN** one creature steps diagonally while another steps straight | 一只生物斜向走一步，另一只直向走一步时
- **THEN** the two picture moves take the same real time | 两段画面移动的真实时长相同

### Requirement: Freeze and Hold | 冻结与停住

While any creature's picture is moving, the clock SHALL freeze;
planning does not wait for pictures to stop — a due turn plans at
once, different creatures' pictures may move in parallel, and only the
tick ledger is strictly serial. While the player has no queued path,
their turn SHALL NOT be spent; that unspent turn SHALL be the nearest
turn the clock sweeps to — the world holds at that moment, waiting for
input. Freezing applies to world logic only: idle animations SHALL
keep playing as usual.

任何生物的画面在移动时，时钟 SHALL 冻结；规划不等待画面停下——回合到
期即规划，不同生物的画面可以并行移动，严格串行的只是 tick 账。主角没
有排队路径时其回合 SHALL NOT 消耗；这个没有消耗掉的回合 SHALL 成为时
钟扫描到的最近回合——世界停在该时刻等待输入。冻结只作用于世界逻辑：
idle 动画 SHALL 照常播放。

#### Scenario: The world holds when the path runs out | 路径走完世界停住

- **WHEN** the player finishes the queued path and stops | 主角走完排队路径停下时
- **THEN** the clock holds at the player's next turn and no creature acts any more | 时钟停在主角的下一回合时刻，不再有生物行动

#### Scenario: Idle animations play through the freeze | 冻结中 idle 照播

- **WHEN** the world is in the held state | 世界处于停住状态时
- **THEN** every creature's idle animation keeps looping as usual | 所有生物的 idle 动画照常循环播放

### Requirement: Interleaved Turn Cadence | 回合节奏交错

Creatures of different speeds SHALL act interleaved according to their
own durations: creatures due within a frame act first, those not yet
due wait for their own turns; the interval between one creature's
consecutive actions is always its own action duration.

速度不同的生物 SHALL 按各自行动时长交错行动：一帧内到期的生物先行动，
未到期的等自己的回合；一只生物相邻两次行动的间隔恒为其行动时长。

#### Scenario: End-to-end cadence | 端到端节奏

- **WHEN** the player walks a 3-cell path at standard speed with a speed-125 rat on the scene | 主角以标准速度走一条 3 格路径，同场有一只速度 125 的老鼠时
- **THEN** the player stops after 3 steps at tick 300, and the rat has taken exactly 8 turns with its next turn scheduled at tick 320 | 主角 3 步后停在第 300 tick，老鼠恰好完成 8 个回合、下一回合排在第 320 tick
