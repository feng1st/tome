# camera Specification

## Purpose
定义 2D 相机行为：跟随主角滚动视野，主角始终保持在画面中心。

## Requirements

### Requirement: 相机跟随

相机 SHALL 使主角始终位于视野中心。视野可以超出地图范围，地图外的区域显示为背景色（与原版 PD 一致）。

#### Scenario: 主角移动时相机跟随

- **WHEN** 主角在房间内任意位置移动
- **THEN** 相机同步移动，主角位于视野中心

#### Scenario: 主角在房间角落

- **WHEN** 主角移动到房间角落
- **THEN** 主角仍位于视野中心，视野中房间外的部分显示为背景色
