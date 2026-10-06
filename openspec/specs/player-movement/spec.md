# player-movement Specification | player-movement 规格

## Purpose

The player's cell-based movement: a mouse click picks the target cell,
A* pathfinding (8 directions, corner cutting allowed) finds the way,
and the player walks the path cell by cell with the presentation
position moving continuously; the target may be re-picked mid-move.

定义主角的格子化移动行为：鼠标点击选取目标格，A* 寻路（8 方向、允许
切角），沿路径逐格连续插值移动，移动途中可重新选取目标。

## Requirements

### Requirement: Click Picks the Target | 鼠标点击选目标

The game SHALL translate a left mouse button click's screen
coordinates into the cell under that world position and take it as the
movement target; clicking an impassable cell MUST NOT produce
movement.

游戏 SHALL 将鼠标左键点击的屏幕坐标转换为世界坐标对应的格子，作为移
动目标；点击不可通行格时 MUST 不产生移动。

#### Scenario: Clicking a walkable floor cell | 点击可通行地板格

- **WHEN** the player left-clicks a walkable floor cell | 玩家左键点击一个可通行的地板格时
- **THEN** the player starts moving toward that cell along a path | 主角开始沿路径向该格移动

#### Scenario: Clicking a wall or water cell | 点击墙格或水格

- **WHEN** the player left-clicks an impassable wall or water cell | 玩家左键点击不可通行的墙格或水格时
- **THEN** the player does not move and keeps the current state | 主角不移动，保持当前状态

### Requirement: A* Pathfinding | A* 寻路

The game SHALL pathfind over walkable cells with the A* algorithm,
supporting 8-direction (including diagonal) movement; a diagonal step
MUST pass only when the diagonal target cell itself is walkable. An
unreachable target MUST NOT produce movement.

游戏 SHALL 使用 A* 算法在可通行格上寻路，支持 8 方向（含对角）移动；
对角移动 MUST 只在目标对角格可通行时通过。目标不可达时 MUST 不产生移
动。

#### Scenario: Pathfinding around the pool | 绕过水池寻路

- **WHEN** the player clicks a target cell whose straight line is blocked by the pool | 玩家点击直线被水池阻断的目标格时
- **THEN** the player reaches the target along the shortest 8-direction path around the pool | 主角沿绕过水池的最短 8 方向路径到达目标格

### Requirement: Cell-by-Cell Movement | 逐格移动

The player SHALL walk the path cell by cell: the presentation position
moves toward each next cell at a constant pace (0.1 seconds per cell,
straight and diagonal steps alike) and stops at the path's end — the
reference's step-by-step feel.

主角 SHALL 沿路径逐格移动：呈现位置以恒定步速连续移向目标格（每格 0.1
秒，直走与斜走时长相同）；到达路径终点后停止。还原原版的步进手感。

#### Scenario: Walking the path cell by cell | 沿路径逐格移动

- **WHEN** the player has a valid path | 主角获得一条有效路径时
- **THEN** the player advances cell by cell, the presentation position moving continuously through every cell on the path, and stops at the end | 主角逐格前进，呈现位置连续移动、依次经过路径上每个格，到达终点后停止

#### Scenario: Clicking a new target mid-move | 移动中点击新目标

- **WHEN** the player clicks a new walkable target while the player is mid-move | 主角移动途中玩家点击新的可通行目标格时
- **THEN** the player abandons the old path, re-pathfinds from the cell it currently stands in, and heads for the new target | 主角放弃旧路径，从当前所在格重新寻路并前往新目标

### Requirement: Commands Settle Before Planning | 命令先于规划落盘

Movement commands SHALL execute in the command phase — after the world
clock advances and before step planning: target validation (ignored
when impassable or unreachable) and pathfinding complete and settle
there. Step planning MUST read the settled path — when a retarget and
a planning fall on the same frame, that step follows the new path.

移动命令 SHALL 在世界时钟推进之后、步进规划之前的命令阶段执行：目标
验证（不可通行或不可达则忽略）与寻路在该阶段完成并落盘。步进规划
MUST 读到已落盘的路径——改目标与规划同帧发生时，该步沿新路径走出。

#### Scenario: A retarget lands on the planning frame | 改目标落在规划帧

- **WHEN** the player walks a queued path and a retarget command arrives on the very frame the next step is planned | 主角带着排队路径行进，点击新目标的命令恰在下一步规划所在的帧到达时
- **THEN** that step follows the new path's first cell, no cell is lost from the path, and every hop is between adjacent cells | 该步沿新路径的第一格走出，路径不丢格，每一跳都是相邻格

### Requirement: Steps Are Turn-Driven | 步进由回合驱动

Each of the player's steps SHALL be a turn action: a due turn plans
the next step (taking the path's head cell) without waiting for any
picture to stop — pictures may run in parallel; only the tick ledger
is strictly serial. Planning spends the turn (the next-turn slot
advances by one action's duration); with no path, the turn is not
spent. When the action takes effect, the player's cell coordinate
SHALL change to the target cell at once — game logic reads only cell
coordinates, and the presentation position is caught up by the display
side at its constant pace.

主角的每一步 SHALL 是一个回合行动：回合到期即规划下一步（取路径首
格），不等待任何画面停下——画面可以并行，严格串行的只是 tick 账；规
划即消耗回合（回合槽推进一个行动时长）；无路径则回合不消耗。行动生效
时，主角的格坐标 SHALL 立即修改为目标格——游戏逻辑的判定只读格坐
标，呈现位置由画面层以恒定步速追上。

#### Scenario: A due turn plans one step | 到期规划一步

- **WHEN** the player's turn comes due with a non-empty path (whether or not any picture is moving) | 主角回合到期、路径非空（无论是否有画面在移动）时
- **THEN** an action stepping to the path's head cell is produced, and the next-turn slot advances by 100 ticks (standard speed) | 产出一个走向路径首格的行动，回合槽推进 100 tick（标准速度）

#### Scenario: No path, no turn spent | 无路径不消耗回合

- **WHEN** the player's turn comes due with no queued path | 主角回合到期但没有排队路径时
- **THEN** the next-turn slot stays put and the world holds | 回合槽保持不动，世界停住
