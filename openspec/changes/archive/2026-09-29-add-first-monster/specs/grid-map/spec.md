# grid-map Specification Delta

## ADDED Requirements

### Requirement: 地图怪物出生条目

固定地图的数据文件 SHALL 可声明怪物出生条目：每条由怪物 id 与格子坐标组成；文件未声明时视为无出生条目。出生条目的坐标 MUST 落在地图范围内，越界 SHALL 导致启动失败，错误信息指明出错文件与出错位置。

#### Scenario: 从文件解析出生条目

- **WHEN** 地图文件声明若干怪物出生条目
- **THEN** 每个条目解析为怪物 id 与格子坐标，随地图数据一同加载

#### Scenario: 越界出生条目拒绝启动

- **WHEN** 出生条目的坐标越出地图范围
- **THEN** 启动失败，错误信息指明出错文件与出错位置
