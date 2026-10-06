# player-animation Specification | player-animation 规格

## Purpose

The player sprite's frame animation: the frame sequences and rates of
the idle and run animations, how playback switches with movement
state, and how the sprite turns with the horizontal direction of
travel.

定义主角精灵的帧动画行为：待机与奔跑动画的帧序列、帧率、随移动状态的
切换，以及随水平移动方向的朝向翻转。

## Requirements

### Requirement: Run Animation | 奔跑动画

While the player moves, the sprite SHALL play the run loop: sprite
sheet frames 2–7 at about 20 fps.

主角移动时 SHALL 播放奔跑循环动画，帧序列为精灵表帧 2–7，帧率约 20
fps。

#### Scenario: Run plays while moving | 移动时播放奔跑动画

- **WHEN** the player is in the moving state | 主角处于移动状态时
- **THEN** the sprite loops frames 2–7 at about 20 fps | 精灵按帧 2–7 循环播放，帧率约 20 fps

### Requirement: Idle Animation | 待机动画

While the player stands still, the sprite SHALL play the idle loop:
sprite sheet frames 0 and 1 in alternation (the reference's breathing
rhythm) at about 8 fps.

主角静止时 SHALL 播放待机循环动画，帧序列为精灵表帧 0 与 1 交替（还原
原版呼吸节奏），帧率约 8 fps。

#### Scenario: Switching to idle after stopping | 停止后切换到待机

- **WHEN** the player reaches the path's end and stops moving | 主角到达路径终点停止移动时
- **THEN** the sprite animation switches to the idle loop alternating frames 0 and 1 | 精灵动画切换为帧 0/1 交替的待机循环

### Requirement: Facing Turn | 朝向翻转

While the player moves horizontally, the sprite SHALL mirror
horizontally with the direction of travel so that it faces the way it
moves.

主角水平移动时 SHALL 根据移动方向水平翻转精灵，使其面向移动方向。

#### Scenario: Facing left on leftward travel | 向左移动即翻转

- **WHEN** the player moves left (including diagonally left) | 主角向左（含左斜向）移动时
- **THEN** the sprite mirrors horizontally to face left; it faces right again on rightward travel | 精灵水平翻转面向左；向右移动时恢复面向右
