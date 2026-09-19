# Spec: hero-animation

## Purpose

定义主角精灵的帧动画行为：待机与奔跑动画的帧序列、帧率及随移动状态的切换。

## ADDED Requirements

### Requirement: 奔跑动画

主角移动时 SHALL 播放奔跑循环动画，帧序列为精灵表帧 2–7，帧率约 20 fps。

#### Scenario: 移动时播放奔跑动画

- **WHEN** 主角处于移动状态
- **THEN** 精灵按帧 2–7 循环播放，帧率约 20 fps

### Requirement: 待机动画

主角静止时 SHALL 播放待机循环动画，帧序列为精灵表帧 0 与 1 交替（还原原版呼吸节奏）。

#### Scenario: 停止后切换到待机

- **WHEN** 主角到达路径终点停止移动
- **THEN** 精灵动画切换为帧 0/1 交替的待机循环

### Requirement: 朝向翻转

主角水平移动时 SHALL 根据移动方向水平翻转精灵，使其面向移动方向。

#### Scenario: 向左移动时翻转

- **WHEN** 主角向左（含左斜向）移动
- **THEN** 精灵水平翻转面向左；向右移动时恢复面向右
