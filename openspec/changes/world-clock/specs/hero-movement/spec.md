# hero-movement Specification Delta

## ADDED Requirements

### Requirement: 主角步进驱动世界时钟

主角步进 SHALL 由行动点累积驱动：帧 delta 连续累积，满 100 点完成一步；帧率抖动 MUST NOT 改变步进的节奏与平滑度。主角每帧累积的点数 SHALL 发布为世界时钟的帧进度，主角路径的剩余点数 SHALL 发布为世界的剩余量（供怪物决策预判）。主角 SHALL 携带整格逻辑位，步完成时翻转为新格；呈现位保持浮点连续。

#### Scenario: 主角进度即世界时间

- **WHEN** 主角在移动中的一帧累积了若干行动点
- **THEN** 世界时钟该帧的进度等于该累积量，每只怪物按自己的速率比值同步累积

#### Scenario: 帧率抖动不损平滑

- **WHEN** 相邻两帧的 delta 不等
- **THEN** 主角每格的真实时长不变，呈现位置按累积量插值

#### Scenario: 步完成翻转逻辑位

- **WHEN** 主角完成一步
- **THEN** 其逻辑位变为新格坐标；步进途中逻辑位保持为出发格
