# world-clock Specification (delta) | world-clock 规格（增量）

## ADDED Requirements

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
“当前时刻 + 时长”，不再逐回合积攒——MUST NOT 重新引入。两个模
型的重合是有意的：速率表与 `extract_energy` 逐项一致（见“时长
定价与速率表”），且标准速度下标准行动的代价 100 正是参照的行动
阈值。一处细微差异有意不同：参照把小数余数留在累积器里（速率 15
的生物行动间隔按 66/67 tick 交替）；本条时长按行动一次取整，同
一只生物相邻两次行动的间隔恒为该行动的时长（见“回合节奏交
错”）。

#### Scenario: No accumulator exists | 不存在累积器

- **WHEN** any creature finishes an action | 任一生物完成一个行动时
- **THEN** its next-turn slot is set to "now + the action's duration" at once, and no per-creature quantity grows between turns | 其回合槽立即定为“当前时刻 + 行动时长”，回合之间不存在任何逐回合增长的生物侧数量

#### Scenario: No remainder carryover | 余数不结转

- **WHEN** a speed-115 creature (rate 15) acts repeatedly | 速度 115（速率 15）的生物连续行动时
- **THEN** every interval between its consecutive actions is exactly 67 ticks — the reference's 66/67 alternation never occurs | 相邻两次行动的间隔恒为 67 tick——参照的 66/67 交替从不出现

## MODIFIED Requirements

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
