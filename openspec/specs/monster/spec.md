# monster Specification | monster 规格

## Purpose

Monster kinds and spawning: a core data file declares the monster
vocabulary and assigns runtime handles (a kind is the monster's
identity — an entry carries its speed and combat profile: hit dice,
armor class, level, and blows); map files declare what spawns where.
Monsters spawn as pure game data and block the player: their cells are
obstacles to the player's orders. Their visual presentation is the
creature-identity capability's business; what targeting one means is
the combat capability's.

怪物种类的定义与出生：core 的数据文件声明怪物词表并分配运行时句柄
（kind 即身份，条目携带速度与战斗档案——生命骰、护甲等级、等级、攻击），
地图文件声明出生内容。怪物以纯游戏数据出生并阻挡主角：怪物格是主
角指令路上的障碍。形象呈现由 creature-identity 能力承接；以怪物为
目标意味着什么由 combat 能力承接。

## Requirements

### Requirement: Monster Vocabulary | 怪物词表

Monster kinds SHALL be defined by a core data file: the vocabulary
file declares every monster id as a list of entries, and each entry
SHALL declare the monster's speed (a raw rate-table index), hit_points
(a dice-notation string), armor_class (an integer), level (an
integer), and blows (a list of damage dice, each element holding one
dice notation). The kind is the monster's identity — no race, class,
or unique id is declared. Loading assigns each id a runtime handle in
entry order and parses every entry's dice notations. A handle is a
transient identifier within one load of the process and MUST NOT be
assumed stable across loads; cross-load scenarios such as saves MUST
translate through the "handle ↔ id" mapping. Code MUST NOT contain a
monster enum, nor test a handle or an id for monster identity. Adding
a monster MUST take only a new entry in each of the vocabulary file,
the figure binding file, and the figure table — no code change.

怪物的种类 SHALL 由 core 的数据文件定义：词表文件以词条列表声明全部怪
物 id，每个条目 SHALL 声明该怪物的 speed（速率表索引原值）、hit_points
（骰子格式字符串）、armor_class（整数）、level（整数）与 blows（伤害骰列
表，每个元素含一个伤害骰子格式）。kind 即怪物的身份——不声明 race、class
或 unique id。加载时按条目出现顺序为每个 id 分配运行时句柄，并在加载
期解析每条的骰子格式。句柄只是进程内一次加载中的临时标识，MUST NOT 假定
其跨加载稳定；存档等跨加载场景 MUST 经"句柄 ↔ id"映射翻译。代码
MUST NOT 包含怪物枚举，也不以句柄或 id 判断怪物身份。新增怪物 MUST 只
需在词表文件、形象绑定文件与形象表中各新增一个条目，不改动任何代码。

#### Scenario: Ids resolve to handles | 按名换取句柄

- **WHEN** the vocabulary file declares a list of monster id entries and loads | 词表文件声明怪物 id 词条列表并加载时
- **THEN** each id resolves to a handle; the same id resolves to equal handles within one load, different ids to different ones | 每个 id 可换取一个句柄；同一 id 在同一次加载内换取的句柄相等，不同 id 的句柄不同

#### Scenario: Adding a monster changes no code | 新增怪物不改代码

- **WHEN** a new entry is added for a new monster in each of the vocabulary file, the figure binding file, and the figure table, and the map file declares its spawn | 在词表文件、形象绑定文件与形象表中为一个新怪物各新增一个条目，并在地图文件声明其出生时
- **THEN** the monster loads and appears in the game without any code change | 不改动任何代码即可加载该怪物并在游戏中呈现

#### Scenario: Missing or out-of-range speed fails startup | speed 缺失或越界拒绝启动

- **WHEN** a monster entry lacks the speed field or its speed falls outside the rate table | 怪物条目缺失 speed 字段或 speed 超出速率表范围时
- **THEN** startup fails; a missing field is reported with the file and the field name, an out-of-range value further names the monster id | 启动失败；缺失时错误信息指明出错文件与缺失字段，越界时进一步指明该怪物 id

#### Scenario: Missing combat fields or illegal dice fail startup | 战斗字段缺失或格式非法拒绝启动

- **WHEN** a vocabulary entry lacks any combat field (hit_points, armor_class, level, blows), or a damage dice in hit_points or blows is not legal dice notation | 词表条目缺失任一战斗字段（hit_points、armor_class、level、blows），或其 hit_points、blows 的伤害骰不是合法格式时
- **THEN** startup fails; a missing field is reported with the file and the field name, an illegal dice further names the monster id and the legal form | 启动失败；缺失时错误信息指明出错文件与缺失字段，格式非法时进一步指明该怪物 id 与合法形态

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
maximum SHALL be rolled from the entry's hit_points at spawn, and the
current value starts equal to the maximum. The entity carries no
race, class, or unique handle. Presentation data is attached by the
frontend in the spawn reaction, keyed by the spawn cell. A spawn
entry whose monster id is not declared in the vocabulary SHALL fail
startup, the error naming that id.

进入游戏状态时 SHALL 按当前地图的出生表为每个出生条目生成一个怪物实
体：实体携带怪物句柄、出生格坐标、由条目 speed 解析的速度组件、回合
槽组件、生命值组件、持条目 armor_class 的护甲等级组件，以及打击列表——
条目每声明一条 blow 携带一击，其命中率为固定威力顶替值 60 加三倍条
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
- **THEN** the entity carries the hit-point component, the current value equal to the maximum, and the maximum within the entry's hit-dice range (dice count to dice count × face count, both ends included) | 实体携带生命值组件，当前值等于上限，且上限落在该条目生命骰的值域内（骰数到骰数×面数，含两端）

#### Scenario: Spawning carries the blows and the armor | 出生携带打击与护甲等级

- **WHEN** a spawn entry produces a monster entity | 一个出生条目生成怪物实体时
- **THEN** the entity carries the armor class of its entry and one blow per entry blow — each blow's chance equal to the power stand-in 60 plus three times the entry's level, its damage equal to the entry blow's dice | 实体携带其条目的护甲等级，并按条目每条 blow 携带一击——每击命中率等于威力顶替值 60 加三倍条目等级，伤害等于该条目 blow 的骰子

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

### Requirement: Giant White Rat Data | 巨白鼠数据

The monster vocabulary SHALL declare the entry
`( monster: "giant_white_rat", speed: 110, hit_points: "2d2",
armor_class: 7, level: 4, blows: [ ( damage: "1d3" ) ] )`; its figure
binding and figure data are the creature-identity and figure
capabilities' business. The test room SHALL declare two giant white
rat spawns, at cells (28,10) and (24,13).

怪物词表 SHALL 声明条目 `( monster: "giant_white_rat", speed: 110,
hit_points: "2d2", armor_class: 7, level: 4, blows: [ ( damage: "1d3" ) ]
)`；其形象绑定与形象数据由 creature-identity 与 figure 能力承接。测试
房间 SHALL 声明两只巨白鼠出生，格子分别为 (28,10) 与 (24,13)。

#### Scenario: Two rats in the test room | 测试房间呈现两只老鼠

- **WHEN** the game starts into the test room | 启动游戏进入测试房间时
- **THEN** cells (28,10) and (24,13) each present an idling rat, with staggered start frames | 格子 (28,10) 与 (24,13) 各呈现一只播放 idle 动画的老鼠，且二者起始帧错开

#### Scenario: The rat entry value for value | 老鼠条目逐值

- **WHEN** the monster vocabulary is loaded | 加载怪物词表时
- **THEN** the giant white rat entry has hit dice 2d2, armor class 7, level 4, and a blow list holding exactly one 1d3 damage dice | 巨白鼠条目的生命骰为 2d2、护甲等级 7、等级 4，且攻击列表只含一个 1d3 伤害骰

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

### Requirement: Monster Attack | 怪物攻击

When a monster's turn comes due and the living player stands in one
of the eight neighboring cells, the monster SHALL plan an attack at
the player and spend the turn as usual — one action's duration —
planning no step that turn. The attack resolves through the shared
pipeline (see the combat capability). A monster MUST NOT plan an
attack while the player is dead.

怪物的回合到期且存活主角站在八邻格之一时，怪物 SHALL 策划一次对
主角的攻击，并照常消耗回合——一个行动时长——本回合不策划任何
步伐。攻击经共享管线解析（见 combat 能力）。主角已死亡时怪物
MUST NOT 策划攻击。

#### Scenario: An adjacent player is attacked | 相邻主角挨打

- **WHEN** a monster's turn comes due and the living player stands in one of the eight neighboring cells | 怪物回合到期且存活主角站在八邻格之一时
- **THEN** an attack at the player is planned, the next-turn slot advances by one action's duration, and no step is planned | 策划一次对主角的攻击，回合槽推进一个行动时长，不策划步伐

#### Scenario: A distant player is not attacked | 不相邻不攻击

- **WHEN** a monster's turn comes due and the player stands beyond the eight neighboring cells | 怪物回合到期且主角在八邻格之外时
- **THEN** no attack is planned, and the turn plans as the wandering requirement describes | 不策划攻击，本回合按游荡条款策划

#### Scenario: A dead player is not attacked | 死亡主角不挨打

- **WHEN** a monster's turn comes due while the player is dead | 主角已死亡、怪物回合到期时
- **THEN** no attack is planned | 不策划攻击
