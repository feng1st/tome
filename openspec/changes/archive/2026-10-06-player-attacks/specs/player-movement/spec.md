# player-movement Specification (delta) | player-movement 规格（增量）

## ADDED Requirements

### Requirement: Standing Move Order | 站立移动指令

A click on a walkable cell SHALL issue the player a standing move
order naming the target cell, replacing any standing order; the player
carries at most one standing order. The order SHALL persist across
turns until it clears: arriving at the target clears it, having no
route to the target clears it, and reaching a cell adjacent to the
target while a monster stands on the target clears it.

玩家点击可走格 SHALL 向玩家发出以该格为目标的站立移动指令，替换任
何既有站立指令；玩家至多持有一道站立指令。指令 SHALL 跨回合存续
直至清除：到达目标格清除、无路线可达时清除、目标格上有怪物而主
角走到其邻格时清除。

#### Scenario: Arrival clears the order | 到格清指令

- **WHEN** the player reaches the target cell | 主角到达目标格时
- **THEN** the order is gone and no further step is planned | 指令消失，不再策划任何一步

#### Scenario: The target blocked by a monster | 目标格被怪物占据

- **WHEN** a monster stands on the target cell and the player comes adjacent to it | 怪物站在目标格上、主角走到其邻格时
- **THEN** the order clears without the player stepping onto the target | 指令清除，主角不踏上目标格

#### Scenario: A new click replaces the order | 新点击替换指令

- **WHEN** the player clicks a new walkable target while an order stands | 指令存续期间玩家点击新的可走目标时
- **THEN** the old order is replaced and the next step heads for the new target | 旧指令被替换，下一步朝新目标走出

## RENAMED Requirements

- FROM: `### Requirement: Commands Settle Before Planning | 命令先于规划落盘`
  TO: `### Requirement: Commands Land Before Planning | 命令先于规划落盘`

## MODIFIED Requirements

### Requirement: Click Picks the Target | 鼠标点击选目标

The game SHALL translate a left mouse button click's screen
coordinates into the cell under that world position and take it as the
command target; clicking an impassable cell MUST NOT produce any
order; clicking a cell holding a living monster MUST NOT produce a
move order (it issues an attack order — see the combat capability).

游戏 SHALL 将鼠标左键点击的屏幕坐标转换为世界坐标对应的格子，作
为命令目标；点击不可通行格 MUST NOT 产生任何指令；点击持有活怪
物的格子 MUST NOT 产生移动指令（它发出攻击指令——见 combat 能
力）。

#### Scenario: Clicking a walkable floor cell | 点击可通行地板格

- **WHEN** the player left-clicks a walkable floor cell holding no monster | 玩家左键点击一个没有怪物的可通行地板格时
- **THEN** the player starts moving toward that cell | 主角开始向该格移动

#### Scenario: Clicking a wall or water cell | 点击墙格或水格

- **WHEN** the player left-clicks an impassable wall or water cell | 玩家左键点击不可通行的墙格或水格时
- **THEN** the player does not move and keeps the current state | 主角不移动，保持当前状态

#### Scenario: Clicking a monster's cell | 点击怪物所在格

- **WHEN** the player left-clicks a cell holding a living monster | 玩家左键点击持有活怪物的格子时
- **THEN** no move order is issued | 不产生移动指令

### Requirement: A* Pathfinding | A* 寻路

The game SHALL pathfind over walkable cells with the A* algorithm,
supporting 8-direction (including diagonal) movement; a diagonal step
MUST pass only when the diagonal target cell itself is walkable. When
pathing for a standing order, cells holding living monsters SHALL be
treated as obstacles, the route's destination cell exempt. An
unreachable target MUST NOT produce movement.

游戏 SHALL 使用 A* 算法在可通行格上寻路，支持 8 方向（含对角）移
动；对角移动 MUST 只在目标对角格可通行时通过。为站立指令寻路
时，持有活怪物的格子 SHALL 视为障碍，路线目的格本身除外。目标不
可达时 MUST NOT 产生移动。

#### Scenario: Pathfinding around the pool | 绕过水池寻路

- **WHEN** the player clicks a target cell whose straight line is blocked by the pool | 玩家点击直线被水池阻断的目标格时
- **THEN** the player reaches the target along the shortest 8-direction path around the pool | 主角沿绕过水池的最短 8 方向路径到达目标格

#### Scenario: Routing around a wandering monster | 绕开游荡怪物

- **WHEN** a monster stands on the straight line to the target and a way around exists | 怪物站在通往目标的直线上且存在绕行路线时
- **THEN** the planned step follows a route around the monster's cell | 策划的一步沿绕开怪物格的路线走出

### Requirement: Cell-by-Cell Movement | 逐格移动

The player SHALL approach the target cell by cell: each due turn plans
one step along a freshly computed route, the presentation position
moves toward each next cell at a constant pace (0.1 seconds per cell,
straight and diagonal steps alike), and the movement ends when the
standing order clears.

主角 SHALL 逐格逼近目标：每个到期回合沿即时计算的路线策划一步，
呈现位置以恒定步速连续移向下一格（每格 0.1 秒，直走与斜走时长
相同）；移动随站立指令清除而结束。

#### Scenario: Walking the path cell by cell | 沿路径逐格移动

- **WHEN** the player holds a move order | 主角持有移动指令时
- **THEN** the player advances one cell per due turn, the presentation position moving continuously through every passed cell, and stops when the order clears | 主角每个到期回合前进一格，呈现位置连续移动、依次经过每个途经格，指令清除时停下

#### Scenario: Clicking a new target mid-move | 移动中点击新目标

- **WHEN** the player clicks a new walkable target while the player is mid-move | 主角移动途中玩家点击新的可通行目标格时
- **THEN** the standing order is replaced, the next step is computed from the cell the player currently stands in, and the player heads for the new target | 站立指令被替换，下一步从当前所在格即时计算，主角前往新目标

### Requirement: Commands Land Before Planning | 命令先于规划落盘

Movement commands SHALL execute in the command phase — after the world
clock advances and before step planning: target validation (ignored
when impassable) and the order dispatch (a monster on the target cell
produces an attack order, otherwise a move order) complete and land
there. Step planning MUST read the landed order — when a retarget and
a planning fall on the same frame, that step follows the new order.

移动命令 SHALL 在世界时钟推进之后、步进规划之前的命令阶段执行：
目标校验（不可通行则忽略）与指令分派（目标格有怪物产生攻击指
令，否则产生移动指令）在该阶段完成并落盘。步进规划 MUST 读到已
落盘的指令——改目标与规划同帧发生时，该步跟随新指令。

#### Scenario: A retarget lands on the planning frame | 改目标落在规划帧

- **WHEN** the player holds a standing order and a retarget command arrives on the very frame the next step is planned | 主角持有站立指令，改目标的命令恰在下一步规划所在的帧到达时
- **THEN** that step follows the new order, and every hop is between adjacent cells | 该步跟随新指令走出，且每一跳都是相邻格

### Requirement: Steps Are Turn-Driven | 步进由回合驱动

Each of the player's steps SHALL be a turn action: a due turn plans
one step from the standing order (one cell along the freshly computed
route) without waiting for any picture to stop — pictures may run in
parallel; only the tick ledger is strictly serial. Planning an action
spends the turn (the next-turn slot advances by one action's
duration); with no standing order the turn is not spent. When the
action takes effect, the player's cell coordinate SHALL change to the
target cell at once — game logic reads only cell coordinates, and the
presentation position is caught up by the display side at its constant
pace.

主角的每一步 SHALL 是一个回合行动：回合到期即按站立指令规划一步
（即时路线的一格），不等待任何画面停下——画面可以并行，严格串
行的只是 tick 账；规划出行动即消耗回合（回合槽推进一个行动时
长）；无站立指令则回合不消耗。行动生效时，主角的格坐标 SHALL 立
即修改为目标格——游戏逻辑的判定只读格坐标，呈现位置由画面层以
恒定步速追上。

#### Scenario: A due turn plans one step | 到期规划一步

- **WHEN** the player's turn comes due with a standing order (whether or not any picture is moving) | 主角回合到期且持有站立指令（无论是否有画面在移动）时
- **THEN** an action stepping one cell along the route is produced, and the next-turn slot advances by 100 ticks (standard speed) | 产出一个沿路线走一格的行动，回合槽推进 100 tick（标准速度）

#### Scenario: No path, no turn spent | 无路径不消耗回合

- **WHEN** the player's turn comes due with no standing order | 主角回合到期但没有站立指令时
- **THEN** the next-turn slot stays put and the world holds | 回合槽保持不动，世界停住
