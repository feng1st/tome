# particles Specification (delta) | particles 规格（增量）

## Purpose

The point-particle mechanism: short-lived small squares sprayed in a
cone, integrated under gravity, shrinking out over their lifetimes.
Combat feedback's blood burst is the first user.

点粒子机制：存活短暂的小方块按锥形喷射而出，在重力下积分，随生命
收缩殆尽。战斗反馈的血粒子迸溅是它的第一个用户。

## ADDED Requirements

### Requirement: Particle Burst | 粒子迸溅

A burst SHALL spawn the requested count of particles at a position.
Each particle SHALL draw: a lifetime uniform within the burst's
lifetime range, a speed uniform within the burst's speed range, and a
direction uniform within the burst's cone; and SHALL carry the
burst's color, initial size, and gravity. A count of zero SHALL spawn
nothing.

一次迸溅 SHALL 在指定位置生成请求数量的粒子。每个粒子 SHALL 各自
抽取：在迸溅生命区间内均匀的生命、在迸溅速度区间内均匀的速
度、在迸溅锥角内均匀的方向；并携带迸溅的颜色、初始尺寸与重
力。数量为零 SHALL 什么都不生成。

#### Scenario: A burst spawns its count within range | 迸溅按数生成且各值在区间内

- **WHEN** a burst requests five particles with lifetimes 0.5..1.0 seconds and speeds 40..80 | 一次迸溅请求五粒、生命区间 0.5..1.0 秒、速度区间 40..80 时
- **THEN** five particles exist, each lifetime within 0.5..1.0 and each speed within 40..80 | 存在五颗粒子，各自生命在 0.5..1.0 内、速度在 40..80 内

#### Scenario: A zero count spawns nothing | 零数量不生成

- **WHEN** a burst requests zero particles | 一次迸溅请求零粒时
- **THEN** no particle entity exists | 不存在任何粒子实体

### Requirement: Particle Motion | 粒子运动

Each frame a particle SHALL accelerate its speed by its gravity,
advance its position by its speed, and shrink its size linearly
toward zero across its lifetime; a particle whose life ends SHALL
leave the world.

每帧粒子 SHALL 按重力加速其速度、按速度推进其位置、随生命进程
把尺寸线性缩向零；生命结束的粒子 SHALL 离开世界。

#### Scenario: Gravity bends the flight | 重力改变轨迹

- **WHEN** a particle with an upward speed and positive gravity advances frames | 一颗向上飞行且重力为正的粒子经过若干帧后
- **THEN** its upward speed has decreased frame over frame | 其向上速度逐帧减小

#### Scenario: The end of life removes the particle | 生命结束粒子消失

- **WHEN** a particle's remaining life runs out | 粒子剩余生命耗尽时
- **THEN** the particle's entity no longer exists | 该粒子的实体不复存在
