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

The world opens at zero: every creature's first turn slot SHALL sit
at the world's zero, so a creature due at the opening plans as the
world opens — no planner checks for anything. From then on the driver's
unspent turn is the nearest one, and the clock rests on it until input
produces the next action — pure schedule arithmetic.

A dead creature's slot SHALL leave the schedule — the sweep MUST NOT
sweep onto it — with one exception: the driver's. A dead driver's
never-spent turn is what holds the world at the game over; any other
dead turn MUST NOT pin anything, or the world would freeze on a corpse.

每只生物 SHALL 持有持久的回合槽（下一回合时刻，整数 tick）：规划行动
时推进为"当前时刻 + 该行动时长"，槽位本身永不删除。每帧帧首，任何生
物的画面在移动时时钟 SHALL 保持不动；否则时钟 SHALL 推进到全体回合槽
的最近值，且 MUST NOT 倒退。时钟为纯整数算术，MUST NOT 引入真实时间。

世界自零点开张：一切生物的首个回合槽 SHALL 落在世界零点，开张时
到期的生物随即照常策划——任何策划系统都不为此做检查。其后驾驶者
未消耗的回合即最近槽，时钟停驻其上直至输入产生下一行动——纯凭调
度算术。

死亡生物的回合槽 SHALL 退出调度——扫描 MUST NOT 扫到它——唯一例外
是驾驶者：死亡驾驶者那永不消耗的回合正是游戏结束时世界的停驻点；
其余任何死亡回合 MUST NOT 钉住任何东西，否则世界会冻在尸体上。

#### Scenario: Moving pictures freeze the clock | 画面移动冻结时钟

- **WHEN** any creature's picture is still moving | 任一生物的画面仍在移动时
- **THEN** the clock holds still that frame | 该帧时钟保持不动

#### Scenario: Sweep to the nearest turn | 扫到最近回合

- **WHEN** no picture is moving and the creatures' next-turn slots read 33, 50, and 100 | 没有任何画面在移动，且各生物回合槽分别为 33、50、100 时
- **THEN** the clock advances to 33 | 时钟推进到 33

#### Scenario: The clock never moves backward | 时钟不倒退

- **WHEN** the clock's current value is already past every next-turn slot | 时钟当前值已大于所有回合槽时
- **THEN** the clock keeps its current value | 时钟保持当前值

#### Scenario: The world opens at zero | 世界自零点开张

- **WHEN** the world opens with every creature's first slot at the world's zero | 一切生物的首个回合槽都落在世界零点时
- **THEN** a due creature plans as the world opens, without waiting for any other creature | 到期的生物随开张照常策划，不等任何生物

#### Scenario: The clock rests on the ready driver's unspent turn | 时钟停驻待命驾驶者的未消耗回合

- **WHEN** the driver stands ready with no order and no nearer turn exists | 驾驶者待命、无指令且无更近的回合时
- **THEN** the clock holds on that turn until input produces the next action | 时钟停在该回合直至输入产生下一行动

#### Scenario: A dead creature's turn leaves the schedule | 死亡生物的回合退出调度

- **WHEN** a dead non-driver creature's slot is due and a living creature's slot lies further ahead | 一只已死的非驾驶者生物的回合槽已到期、某存活生物的槽更靠后时
- **THEN** the clock sweeps past the dead slot onto the living one | 时钟越过死亡槽，扫到存活生物的槽

#### Scenario: The dead driver's turn holds the world | 死亡驾驶者的回合停住世界

- **WHEN** the driver is dead with an unspent due turn | 驾驶者已死、到期回合未消耗时
- **THEN** the clock holds at that turn — the game over | 时钟停在该回合上——游戏结束

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

### Requirement: The Reference's Energy Accumulation Is Replaced | 参照的能量累积机制已被替代

The reference implementation (tome2) schedules turns by accumulating
energy — original code, kept here so the mechanism stays searchable
(player: `.ref/tome2/src/dungeon.c:4752-5000`; monster:
`.ref/tome2/src/melee2.c:7718-7728`):

```c
/* Give the player some energy */
p_ptr->energy += extract_energy[speed_use];
if (p_ptr->energy < 100) return;      /* not enough: no action */
while (p_ptr->energy >= 100) { ... }  /* act while energy suffices */
p_ptr->energy -= energy_use;          /* pay the action's cost */

m_ptr->energy += e;                   /* monster side */
if (m_ptr->energy < 100) continue;
m_ptr->energy -= 100;                 /* every monster action costs 100 */
```

This mechanism is superseded by this capability's next-turn slot and
action pricing — an action's cost is priced once into "now +
duration" instead of being saved up turn by turn — and MUST NOT be
reintroduced. The two models coincide deliberately: the rate table
matches `extract_energy` entry for entry (see Action Pricing and the
Rate Table), and a standard action at standard speed costs 100 — the
reference's action threshold. One nuance resolves differently on
purpose: the reference keeps the fractional remainder in the
accumulator (a rate-15 creature alternates 66/67-tick intervals);
here the duration rounds once per action, and the interval between
one creature's consecutive actions is always the action's own
duration (see Interleaved Turn Cadence).

参照实现（tome2）以能量累积安排回合——原始代码保留在此，供检索
（玩家：`.ref/tome2/src/dungeon.c:4752-5000`；怪物：
`.ref/tome2/src/melee2.c:7718-7728`）：

```c
/* Give the player some energy */
p_ptr->energy += extract_energy[speed_use];
if (p_ptr->energy < 100) return;      /* 不够：不行动 */
while (p_ptr->energy >= 100) { ... }  /* 能量够就行动 */
p_ptr->energy -= energy_use;          /* 支付行动代价 */

m_ptr->energy += e;                   /* 怪物侧 */
if (m_ptr->energy < 100) continue;
m_ptr->energy -= 100;                 /* 怪物每个行动固定扣 100 */
```

该机制已被本能力的回合槽与时长定价取代——行动代价一次性定价为
"当前时刻 + 时长"，不再逐回合积攒——MUST NOT 重新引入。两个模
型的重合是有意的：速率表与 `extract_energy` 逐项一致（见"时长
定价与速率表"），且标准速度下标准行动的代价 100 正是参照的行动
阈值。一处细微差异有意不同：参照把小数余数留在累积器里（速率 15
的生物行动间隔按 66/67 tick 交替）；本条时长按行动一次取整，同
一只生物相邻两次行动的间隔恒为该行动的时长（见"回合节奏交
错"）。

#### Scenario: No accumulator exists | 不存在累积器

- **WHEN** any creature finishes an action | 任一生物完成一个行动时
- **THEN** its next-turn slot is set to "now + the action's duration" at once, and no per-creature quantity grows between turns | 其回合槽立即定为"当前时刻 + 行动时长"，回合之间不存在任何逐回合增长的生物侧数量

#### Scenario: No remainder carryover | 余数不结转

- **WHEN** a speed-115 creature (rate 15) acts repeatedly | 速度 115（速率 15）的生物连续行动时
- **THEN** every interval between its consecutive actions is exactly 67 ticks — the reference's 66/67 alternation never occurs | 相邻两次行动的间隔恒为 67 tick——参照的 66/67 交替从不出现

### Requirement: Freeze and Hold | 冻结与停住

While any creature's picture is moving, the clock SHALL freeze;
planning does not wait for pictures to stop — a due turn plans at
once, different creatures' pictures may move in parallel, and only the
tick ledger is strictly serial. While the player holds no standing
order, their turn SHALL NOT be spent; that unspent turn SHALL be the
nearest turn the clock sweeps to — the world holds at that moment,
waiting for input. Freezing applies to world logic only: idle
animations SHALL keep playing as usual.

任何生物的画面在移动时，时钟 SHALL 冻结；规划不等待画面停下——回
合到期即规划，不同生物的画面可以并行移动，严格串行的只是 tick
账。主角没有站立指令时其回合 SHALL NOT 消耗；这个没有消耗掉的回
合 SHALL 成为时钟扫描到的最近回合——世界停在该时刻等待输入。冻
结只作用于世界逻辑：idle 动画 SHALL 照常播放。

#### Scenario: The world holds when the path runs out | 路径走完世界停住

- **WHEN** the player's standing order clears and the player stops | 主角的站立指令清除、主角停下时
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
