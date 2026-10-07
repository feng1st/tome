# monster delta for combat-feedback | monster 增量：combat-feedback

## MODIFIED Requirements

### Requirement: Monster Wandering | 怪物随机移动

The attack branch precedes the wander (see Monster Attack): while no
living player stands in one of the eight neighboring cells, on each
of its due turns a monster SHALL plan one wander action: 75% of the
time it stays put; 25% of the time it redraws an independent random
direction among the eight up to four times and steps into the first
target cell that is passable and holds no living creature, staying
put if all four fail. The turn SHALL be spent as usual whether the
monster moves or not. The world opens at zero: every monster's first
turn sits at the world's zero alongside the driver's, so resident
monsters take their first action as the world opens, and afterwards
an unspent turn waits its tick like everyone else's — no planner
checks for it. A monster MUST NOT plan while its own picture is
moving. When the action takes effect, the monster's cell coordinate
changes to the target cell at once; the presentation position is
caught up by the display side.

攻击分支先于游荡（见"怪物攻击"）：八邻格内没有存活主角时，怪物
SHALL 在自己的到期回合规划一次游荡：75% 原地不动；25% 从 8 个
方向中独立随机重选至多四次，第一个"可通行且无活物"的目标格即走
入，四次皆失败则原地不动。无论是否移动，该回合 SHALL 照常消耗。
世界自零点开张：每只怪物的首个回合槽与驾驶者的一样落在世界零点，
在场的怪物随开张照常行动；其后未消耗的回合与他人一样按 tick 等
待——任何策划系统都不为此做检查。怪物自己的画面仍在移动时 MUST
NOT 规划。行动生效时，怪物的格坐标立即修改为目标格；呈现位置由
画面层追上。

#### Scenario: The world opens at zero | 世界自零点开张

- **WHEN** the world opens at zero with the driver's and the monsters' first slots all there | 世界零点开张、驾驶者与怪物的首槽都在零点时
- **THEN** a due monster plans its first action at once, spending its turn like any other | 到期怪物立即策划首次行动，回合照常消耗

#### Scenario: A due turn plans and is spent | 到期规划并消耗回合

- **WHEN** a monster's turn comes due while its own picture is still | 怪物回合到期且自身画面静止时
- **THEN** its next-turn slot advances by one action's duration; if it steps into a neighbor cell, the target is adjacent to the current cell | 其回合槽推进一个行动时长；若走入邻格，目标格与当前格相邻

#### Scenario: No planning while its own picture moves | 自身画面在动不规划

- **WHEN** a monster's turn comes due while its own picture is still moving | 怪物回合到期，但它自己的画面仍在移动时
- **THEN** it does not plan that frame, and its next-turn slot stays put | 该帧不规划，回合槽保持不动

#### Scenario: Four blocked rolls mean staying put | 四次皆堵原地不动

- **WHEN** a monster whose four neighbors are all impassable rolls a move | 一只四邻皆不可通行的怪物掷出移动时
- **THEN** it stays put that turn, and the turn is spent as usual | 该回合原地不动，回合照常消耗

#### Scenario: Occupied cells count as blocked | 占位格视同不可走

- **WHEN** a wandering roll picks a cell holding the living player or another monster | 随机移动选中持有存活主角或其他怪物的格子时
- **THEN** that pick fails and the next independent direction is drawn | 该次选取失败，重抽下一个独立方向
