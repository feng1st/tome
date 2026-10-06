# stats Specification (delta) | stats 规格（增量）

## ADDED Requirements

### Requirement: Compressed Statistic Index | 压缩属性索引

The compressed statistic index SHALL map a statistic value onto the
38 brackets shared by every stat bonus table: a value at or below 3
lands on the first bracket; 4 through 18 map one bracket per point;
a value above 18 carries its x directly, one bracket per ten points
of x; 18/220 and beyond share the last (thirty-eighth) bracket.

压缩属性索引 SHALL 把属性值映射到一切属性加成表共享的 38 个分段：
不大于 3 的值落在第一段；4 到 18 逐点一段；18 以上的值直接携带 x，
每十点 x 一段；18/220 及以上共用末段（第三十八段）。

#### Scenario: The scale floor shares the first bracket | 刻度下限共用第一段

- **WHEN** a value at or below 3 is indexed | 对不大于 3 的值取索引时
- **THEN** the first bracket is selected | 选中第一段

#### Scenario: One bracket per point up to 18 | 18 及以下逐点一段

- **WHEN** a value between 4 and 18 is indexed | 对 4 到 18 的值取索引时
- **THEN** each point selects its own bracket, 18 landing on the sixteenth | 每点各选一段，18 落在第十六段

#### Scenario: One bracket per ten points of x above 18 | 18 以上每十点 x 一段

- **WHEN** a value above 18 and below 18/220 is indexed | 对高于 18 且低于 18/220 的值取索引时
- **THEN** every ten points of x select one bracket | 每十点 x 选中一段

#### Scenario: The last bracket is shared | 末段共享

- **WHEN** 18/220 or beyond is indexed | 对 18/220 及以上的值取索引时
- **THEN** the thirty-eighth bracket is selected | 选中第三十八段
