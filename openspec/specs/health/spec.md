# health Specification | health 规格

## Purpose

Hit points: the life component shared by every woundable creature
(current value and ceiling). A player's ceiling derives from
constitution and the hit die at birth; a monster's ceiling is rolled
from its vocabulary hit dice at spawn, independent of the six
statistics.

生命值：一切可受伤生物共享的生命组件（当前值与上限）。玩家生命值上限
由体质与生命骰在出生期派生；怪物生命值上限由其词表生命骰在出生期掷
出，不经六维。

## Requirements

### Requirement: Hit-Point Component | 生命值组件

A woundable creature SHALL carry a hit-point component recording the
current value and the ceiling; the current value MUST NOT exceed the
ceiling. The component is common to players and monsters.

可受伤生物 SHALL 携带生命值组件，记录当前值与上限；当前值 MUST NOT 超
过上限。该组件对玩家与怪物通用。

#### Scenario: The component carries current and ceiling | 组件承载当前值与上限

- **WHEN** a woundable creature's hit-point component is read | 读取一个可受伤生物的生命值组件时
- **THEN** the current value and the ceiling are available, and the current value does not exceed the ceiling | 可取得当前值与上限，且当前值不超过上限

### Requirement: Player Hit-Point Derivation | 玩家生命值派生

A player's hit-point ceiling SHALL derive at birth from their
constitution and their race's and class's hit dice: at level 1 the
ceiling equals the hit die plus half the constitution bonus (integer
division); the hit die is the race's and class's shares summed, and
the constitution bonus is looked up in the 38-bracket bonus table by
the constitution's current value. The derivation runs once at birth
and the current value starts equal to the ceiling; per-level hit
tables and recomputation on constitution change are deferred.

玩家的生命值上限 SHALL 在出生期由其体质与种族、职业生命骰派生：等级 1
时上限等于生命骰加上体质加成的一半（整数除法）；生命骰为种族与职业生
命骰之和，体质加成按体质的当前值查 38 项分段加成表得出。出生期一次算
好，当前值初始等于上限；逐级生命表与体质变动时的重算延后。

#### Scenario: The ceiling is a fixed function of constitution and hit die | 上限为体质与生命骰的确定函数

- **WHEN** the player's constitution current value and the race and class hit dice are given | 给定玩家的体质当前值与种族、职业生命骰时
- **THEN** the hit-point ceiling is determined, equal to the hit die plus half the constitution bonus (integer division) | 生命值上限是确定的，且等于生命骰加上体质加成的一半（整数除法）

#### Scenario: The current value starts at the ceiling | 当前值初始等于上限

- **WHEN** the player is born | 主角出生时
- **THEN** the hit-point current value equals the ceiling | 生命值当前值等于上限

### Requirement: Player Birth Hit Points | 玩家出生生命

At birth the player SHALL carry the hit-point component, its ceiling
derived as in "Player Hit-Point Derivation".

主角出生时 SHALL 携带生命值组件，其上限按"玩家生命值派生"得出。

#### Scenario: The player is born with hit points | 主角出生携带生命值

- **WHEN** the player is born | 主角出生时
- **THEN** the player entity carries the hit-point component, its ceiling the value derived from constitution and hit die | 主角实体携带生命值组件，上限为体质与生命骰派生值

### Requirement: Monster Hit-Point Derivation | 怪物生命值派生

A monster's hit-point ceiling SHALL be rolled at spawn from its
vocabulary hit dice: the ceiling is the sum of N draws of 1..=M (N and
M the entry's dice count and face count), independent of the six
statistics; the current value starts equal to the ceiling.

怪物的生命值上限 SHALL 在出生期由其词表生命骰掷出：上限等于 N 个
1..=M 之和（N、M 为该条目生命骰的骰数与面数），不经六维；当前值初始
等于上限。

#### Scenario: The ceiling is one roll of the hit dice | 上限为生命骰的一次掷出

- **WHEN** a vocabulary entry declares hit dice NdM and the monster spawns | 词表条目声明生命骰 NdM 且对应怪物出生时
- **THEN** its hit-point ceiling lands between N and N×M inclusive, independent of the six statistics | 其生命值上限落在 N 到 N×M 之间（含两端），且与六维无关

#### Scenario: The current value starts at the ceiling | 当前值初始等于上限

- **WHEN** a monster spawns | 怪物出生时
- **THEN** the hit-point current value equals the ceiling | 生命值当前值等于上限
