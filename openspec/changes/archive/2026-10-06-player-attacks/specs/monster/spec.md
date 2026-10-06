# monster Specification (delta) | monster 规格（增量）

## REMOVED Requirements

### Requirement: Monster Presentation and Non-Blocking | 怪物呈现与占位

**Reason**: The non-blocking clause inverts: monsters now block the
player — a move targeting a monster's cell becomes an attack order,
and move routes treat monster cells as obstacles (see the combat and
player-movement deltas). The presentation half survives unchanged in
the replacement requirement.

**原因**：不阻碍条款反转——怪物现在阻挡主角：以怪物格为目标的移
动成为攻击指令，移动路线把怪物格视为障碍（见 combat 与
player-movement 增量）。呈现半条原样保留在替代条款中。

**Migration**: The replacement requirement "Monster Presentation and
Blocking" carries the presentation behavior verbatim; the walk-through
scenario is superseded by the blocking scenarios.

**迁移**：替代条款“Monster Presentation and Blocking”逐字承接呈现
行为；穿行场景由阻挡场景取代。

## ADDED Requirements

### Requirement: Monster Presentation and Blocking | 怪物呈现与阻挡

A monster SHALL present the figure bound to its kind key and play the
idle animation. When several monsters spawn on the same frame, their
idle playback start frames SHALL derive from their own spawn cells
rather than synchronizing at zero. Monsters SHALL block the player:
the player's movement MUST NOT step onto a cell holding a living
monster — a click on such a cell issues an attack order instead (see
the combat capability), and a move order's route treats monster cells
as obstacles.

怪物 SHALL 按其 kind 键绑定的形象呈现并播放 idle 动画。多只怪物同
帧出生时，它们的 idle 播放起始帧 SHALL 由各自出生格坐标派生，而
非全零同步。怪物 SHALL 阻挡主角：主角的移动 MUST NOT 踏上持有活
怪物的格子——点击这样的格子改为发出攻击指令（见 combat 能力），
移动指令的路线把怪物格视为障碍。

#### Scenario: Presenting and idling from birth | 出生即呈现并播放 idle

- **WHEN** a monster entity enters the world | 怪物实体进入世界时
- **THEN** the entity presents the idle animation of the figure bound to its kind key and keeps looping it | 该实体呈现其 kind 键绑定形象的 idle 动画并持续循环播放

#### Scenario: Crowd start frames are staggered | 群体起始帧错开

- **WHEN** two monsters whose spawn-cell coordinates sum differently spawn on the same frame | 两只出生格坐标之和不同的怪物同帧出生时
- **THEN** their idle playback start frames differ | 它们的 idle 播放起始帧不同

#### Scenario: A monster's cell blocks the player | 怪物格阻挡主角

- **WHEN** the player targets a move at a monster's cell | 主角以怪物所在格为移动目标时
- **THEN** the player never steps onto that cell — the click becomes an attack order | 主角永不踏上该格——该点击成为攻击指令

#### Scenario: A move route goes around monsters | 移动路线绕开怪物

- **WHEN** a monster stands between the player and the move target | 怪物站在主角与移动目标之间时
- **THEN** the player's steps go around the monster's cell | 主角的步伐绕开怪物所在格

## MODIFIED Requirements

### Requirement: Monster Wandering | 怪物随机移动

On each of its due turns a monster SHALL plan one action: 75% of the
time it stays put; 25% of the time it redraws an independent random
direction among the eight up to four times and steps into the first
target cell that is passable and holds no living creature, staying
put if all four fail. The turn SHALL be spent as usual whether the monster moves
or not. The world starts running from the player's first action:
monsters MUST NOT plan while the player's next-turn slot is still
zero; a monster MUST NOT plan while its own picture is moving. When
the action takes effect, the monster's cell coordinate changes to the
target cell at once; the presentation position is caught up by the
display side.

怪物 SHALL 在自己的到期回合规划一次行动：75% 原地不动；25% 从 8 个
方向中独立随机重选至多四次，第一个“可通行且无活物”的目标格即走
入，四次皆失败则原地不动。无论是否移动，该回合 SHALL 照常消耗。
世界自主角的首个行动开始运转：主角的回合槽仍为零时怪物 MUST NOT
规划；怪物自己的画面仍在移动时 MUST NOT 规划。行动生效时，怪物的
格坐标立即修改为目标格；呈现位置由画面层追上。

#### Scenario: No planning before the world starts | 世界未启动不规划

- **WHEN** the player's next-turn slot is still zero (the first action has not happened) | 主角的回合槽仍为零（首个行动尚未发生）时
- **THEN** due monsters do not plan, and their next-turn slots stay put | 到期怪物不规划，回合槽保持不动

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
