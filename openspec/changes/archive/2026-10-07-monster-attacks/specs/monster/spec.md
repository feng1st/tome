# monster delta for monster-attacks | monster 增量：monster-attacks

## ADDED Requirements

### Requirement: Monster Strike | 怪物出手

When a monster's turn comes due and the living player stands in one
of the eight neighboring cells, the monster SHALL plan a strike at
the player and spend the turn as usual — one action's duration —
planning no step that turn. The strike resolves through the shared
pipeline (see the combat capability). A monster MUST NOT plan a
strike while the player is dead.

怪物的回合到期且存活主角站在八邻格之一时，怪物 SHALL 策划一次对
主角的出手，并照常消耗回合——一个行动时长——本回合不策划任何
步伐。出手经共享管线解析（见 combat 能力）。主角已死亡时怪物
MUST NOT 策划出手。

#### Scenario: An adjacent player is struck | 相邻主角挨打

- **WHEN** a monster's turn comes due and the living player stands in one of the eight neighboring cells | 怪物回合到期且存活主角站在八邻格之一时
- **THEN** a strike at the player is planned, the next-turn slot advances by one action's duration, and no step is planned | 策划一次对主角的出手，回合槽推进一个行动时长，不策划步伐

#### Scenario: A distant player is not struck | 不相邻不出手

- **WHEN** a monster's turn comes due and the player stands beyond the eight neighboring cells | 怪物回合到期且主角在八邻格之外时
- **THEN** no strike is planned, and the turn plans as the wandering requirement describes | 不策划出手，本回合按游荡条款策划

#### Scenario: A dead player is not struck | 死亡主角不挨打

- **WHEN** a monster's turn comes due while the player is dead | 主角已死亡、怪物回合到期时
- **THEN** no strike is planned | 不策划出手

## MODIFIED Requirements

### Requirement: Monster Spawning | 怪物出生

On entering the game state, one monster entity SHALL spawn per entry
of the current map's spawn table: the entity carries the monster
handle, its spawn cell, the speed component resolved from the entry's
speed, the next-turn component, the hit-point component, the
armor-class component holding the entry's armor_class, and the
blows — one blow per blow the entry declares, its chance the fixed
power stand-in 60 plus three times the entry's level (the stand-in
maps to the reference's HURT effect power and deletes when the effect
family lands), its damage the entry blow's dice; the hit-point
ceiling SHALL be rolled from the entry's hit_points at spawn, and the
current value starts equal to the ceiling. The entity carries no
race, class, or unique handle. Presentation data is attached by the
frontend in the spawn reaction, keyed by the spawn cell. A spawn
entry whose monster id is not declared in the vocabulary SHALL fail
startup, the error naming that id.

进入游戏状态时 SHALL 按当前地图的出生表为每个出生条目生成一个怪物实
体：实体携带怪物句柄、出生格坐标、由条目 speed 解析的速度组件、回合
槽组件、生命值组件、持条目 armor_class 的护甲组件，以及打击列表——
条目每声明一条 blow 携带一击，其命中品质为固定威力顶替值 60 加三倍条
目等级（顶替值映射参照的 HURT effect 威力，随 effect 家族落地删
除），其伤害为该条目 blow 的骰子；生命值上限 SHALL 在出生期掷该条目
的 hit_points 得出，当前值初始等于上限。不携带 race、class 或 unique
句柄。呈现数据由前端按出生格坐标在出生反应中挂载。出生条目的怪物 id
未在词表声明时 SHALL 导致启动失败，错误信息指明该 id。

#### Scenario: Spawning follows the map's declaration | 按地图声明出生

- **WHEN** the current map declares several monster spawn entries and the game state is entered | 当前地图声明了若干怪物出生条目并进入游戏状态时
- **THEN** each spawn entry produces one entity at its cell, carrying the monster handle, the speed component, and the next-turn component | 每个出生条目生成一个位于对应格子、携带怪物句柄、速度组件与回合槽的实体

#### Scenario: An unknown monster fails startup | 未知怪物拒绝启动

- **WHEN** a spawn entry of the current map references a monster id outside the vocabulary | 当前地图的出生条目引用词表之外的怪物 id 时
- **THEN** startup fails, the error naming that monster id | 启动失败，错误信息指明该怪物 id

#### Scenario: Spawning carries hit points | 出生携带生命值

- **WHEN** a spawn entry produces a monster entity | 一个出生条目生成怪物实体时
- **THEN** the entity carries the hit-point component, the current value equal to the ceiling, and the ceiling within the entry's hit-dice range (dice count to dice count × face count, both ends included) | 实体携带生命值组件，当前值等于上限，且上限落在该条目生命骰的值域内（骰数到骰数×面数，含两端）

#### Scenario: Spawning carries the blows and the armor | 出生携带打击与护甲

- **WHEN** a spawn entry produces a monster entity | 一个出生条目生成怪物实体时
- **THEN** the entity carries the armor class of its entry and one blow per entry blow — each blow's chance equal to the power stand-in 60 plus three times the entry's level, its damage equal to the entry blow's dice | 实体携带其条目的护甲，并按条目每条 blow 携带一击——每击命中品质等于威力顶替值 60 加三倍条目等级，伤害等于该条目 blow 的骰子

### Requirement: Monster Wandering | 怪物随机移动

The strike branch precedes the wander (see Monster Strike): while no
living player stands in one of the eight neighboring cells, on each
of its due turns a monster SHALL plan one wander action: 75% of the
time it stays put; 25% of the time it redraws an independent random
direction among the eight up to four times and steps into the first
target cell that is passable and holds no living creature, staying
put if all four fail. The turn SHALL be spent as usual whether the
monster moves or not. The world starts running from the player's
first action: monsters MUST NOT plan while the player's next-turn
slot is still zero; a monster MUST NOT plan while its own picture is
moving. When the action takes effect, the monster's cell coordinate
changes to the target cell at once; the presentation position is
caught up by the display side.

攻击分支先于游荡（见"怪物出手"）：八邻格内没有存活主角时，怪物
SHALL 在自己的到期回合规划一次游荡：75% 原地不动；25% 从 8 个
方向中独立随机重选至多四次，第一个"可通行且无活物"的目标格即走
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
