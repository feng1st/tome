# world-clock delta for combat-feedback | world-clock 增量：combat-feedback

## MODIFIED Requirements

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
