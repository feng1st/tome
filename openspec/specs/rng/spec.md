# rng Specification | rng 规格

## Purpose

The random source: the game's single seedable random state. A normal
run seeds it from system entropy, so one run is independent of
another; tests and debugging may construct it with a given seed for a
fully reproducible sequence.

随机源：全游戏共享的唯一可固定种子值的随机状态。正式游戏每局以系统熵
为种子值，各局相互独立；测试与调试可用固定种子构造，得到完全可复现的
序列。

## Requirements

### Requirement: Central Random Source | 集中随机源

The game SHALL hold a single seedable random source, seeded once from
system entropy when the game state is entered; every random draw from
then on SHALL come from it, and creating another random source MUST
NOT occur.

游戏 SHALL 持有唯一可固定种子值的随机源，进入游戏状态时以系统熵为种子
值；此后一切随机消耗 SHALL 取数自它，另建随机源 MUST NOT 出现。

#### Scenario: Seeded from entropy when unspecified | 种子未固定时以系统熵为种子值

- **WHEN** the game state is entered without a given seed | 进入游戏状态且种子未固定时
- **THEN** the source is seeded from system entropy, and two launches produce independent sequences | 随机源以系统熵为种子值，两次启动得到相互独立的序列

#### Scenario: A given seed reproduces | 固定种子可复现

- **WHEN** two sources built with the same seed each draw a sequence of random numbers | 以同一固定种子构造两个随机源并各自消耗一串随机数时
- **THEN** the two sequences are identical, and sequences from different seeds differ | 两串序列完全一致；不同种子的序列不同

### Requirement: Consumers Draw from the One Source | 消费者统一取数

Every existing random consumer SHALL draw from the central source:
the six-statistic birth roll and the modifier merge, monster
wandering, and the monster hit-dice roll. A consumer's result SHALL
depend only on its inputs and the source's state.

现有随机消耗 SHALL 全部取数自集中随机源：六维出生掷骰与出身合并、怪物
随机移动、怪物生命骰掷出。任一消费者的结果 SHALL 只由其输入与随机源
状态决定。

#### Scenario: Consumers are seedable in tests | 消费者以固定种子复现

- **WHEN** a birth roll or a monster's wander runs on a fixed-seed source | 以固定种子的随机源运行出生掷骰或怪物随机移动时
- **THEN** the result is exactly predictable and reproduces under the same seed | 结果可精确预测，且同种子下复现
