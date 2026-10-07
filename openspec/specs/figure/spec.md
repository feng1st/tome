# figure Specification | figure 规格

## Purpose

Figure declaration and presentation: the display side's figure table
declares each figure as an entry (id, texture, atlas grid, animation
frame tables) and assigns runtime handles at load; the display side
resolves the entry behind an entity's figure handle and attaches the
texture, the atlas layout, and the animation state to that entity.

形象的声明与呈现：display 的形象表以条目声明每个形象（id、贴图、图集
网格、动画帧表），加载时分配运行时句柄；显示侧按实体携带的形象句柄解
析条目，为实体挂载贴图、图集布局与动画状态。

## Requirements

### Requirement: Figure Table Registry | 形象表注册表

Figure kinds and how they draw SHALL be defined by one display data
file: each entry declares a figure id, a texture path, a frame size in
pixels, the atlas grid's columns and rows, and frame tables keyed by
animation names from the code-owned vocabulary — a frame sequence, a
rate, and a looping flag; a looping table wraps its playback, a
non-looping table clamps at its last frame. Loading assigns each id a
runtime handle in entry order. A
handle is a transient identifier within one load of the process and
MUST NOT be assumed stable across loads; cross-load scenarios such as
saves MUST translate through the "handle ↔ id" mapping. Code MUST NOT
contain a figure enum, nor test a handle or an id for figure identity.
Adding a figure MUST take only a new entry in the figure table — no
code change.

形象的种类与画法 SHALL 由 display 的同一数据文件定义：每个条目声明形象
id、贴图路径、帧像素尺寸、图集行列数，以及以代码持有的动画词汇名给出
的帧表——帧序列、帧率与循环标志；循环表回卷播放，不循环表钳在末
帧。加载时按条目出现顺序为每个 id 分配运行时句
柄。句柄只是进程内一次加载中的临时标识，MUST NOT 假定其跨加载稳定；
存档等跨加载场景 MUST 经"句柄 ↔ id"映射翻译。代码 MUST NOT 包含形象
枚举，也不以句柄或 id 判断形象身份。新增形象 MUST 只需在形象表新增一
个条目，不改动任何代码。

#### Scenario: Ids resolve to handles | 按名换取句柄

- **WHEN** the figure table declares several figure entries and loads | 形象表声明若干形象条目并加载时
- **THEN** each figure id resolves to a handle; the same id resolves to equal handles within one load, different ids to different ones | 每个形象 id 可换取一个句柄；同一 id 在同一次加载内换取的句柄相等，不同 id 的句柄不同

#### Scenario: Entries parse from the file | 从文件解析形象条目

- **WHEN** the figure table is loaded | 加载形象表时
- **THEN** each entry resolves into: a texture handle, an atlas layout built from the frame size and the grid dimensions, and frame tables indexed by animation name | 每个条目解析为：贴图句柄、按帧尺寸与行列数构建的图集布局、按动画名索引的帧表

#### Scenario: The looping flag parses per animation | 循环标志随动画解析

- **WHEN** a figure entry declares a looping run table and a non-looping die table | 形象条目声明循环的奔跑帧表与不循环的死亡帧表时
- **THEN** the run table wraps its playback and the die table clamps at its last frame | 奔跑表回卷播放，死亡表钳在末帧

#### Scenario: Adding a figure changes no code | 新增形象不改代码

- **WHEN** a new entry is added to the figure table for a new figure | 在形象表为一个新形象新增一个条目时
- **THEN** the figure loads and can present creatures without any code change | 不改动任何代码即可加载该形象并供生物呈现

### Requirement: Animation Fallback to Idle | 动画回退 Idle

Every figure MUST define an Idle frame table; when a creature plays an
animation its figure does not define, playback SHALL fall back to the
Idle frame table.

每个形象 MUST 定义 Idle 帧表；生物播放其形象未定义的动画时 SHALL 回退
到 Idle 帧表。

#### Scenario: An undefined animation falls back to Idle | 未定义动画回退 Idle

- **WHEN** a creature plays an animation its figure does not define | 生物播放其形象未定义的动画时
- **THEN** what actually plays is the figure's Idle frame table | 实际播放该形象的 Idle 帧表

### Requirement: Figure Data Validation | 形象数据校验

The figure table MUST be validated at load. A duplicated id, an empty
id, an empty frame sequence, a non-positive rate, a frame index
outside the atlas grid, or an entry missing its Idle frame table SHALL
fail startup, the error naming the file and the offending position.

形象表 MUST 在加载时校验。同一 id 重复出现、id 为空、帧序列为空、帧率
非正、帧号越出图集网格、条目缺少 Idle 帧表，SHALL 导致启动失败，错误
信息指明出错文件与出错位置。

#### Scenario: A frame index outside the grid | 帧号越出网格

- **WHEN** a figure entry's frame sequence references a frame number outside the atlas grid | 形象条目的帧序列引用图集网格之外的帧号时
- **THEN** startup fails, the error naming the file and that figure id | 启动失败，错误信息指明出错文件与该形象 id

#### Scenario: A missing Idle frame table fails startup | 缺少 Idle 帧表拒绝启动

- **WHEN** a figure entry does not define an Idle frame table | 形象条目未定义 Idle 帧表时
- **THEN** startup fails, the error naming the file and that figure id | 启动失败，错误信息指明出错文件与该形象 id

### Requirement: Creatures Present Figures | 生物呈现形象

After an entity carrying a figure handle enters the world, the display
side SHALL resolve the figure entry through the handle and attach the
matching texture, atlas layout, and initial animation state to that
entity.

携带形象句柄的实体进入世界后，显示侧 SHALL 按句柄解析形象条目，为该
实体挂载对应贴图、图集布局与初始动画状态。

#### Scenario: Resolving through the handle and attaching | 按句柄解析并挂载

- **WHEN** an entity carrying a figure handle enters the world | 一个携带形象句柄的实体进入世界时
- **THEN** the display side finds the figure entry through the handle and attaches the matching texture, atlas layout, and initial animation state to the entity | 显示侧按句柄查到形象条目，为该实体挂载对应贴图、图集布局与初始动画状态

### Requirement: Warrior Figure Data | warrior 形象数据

The warrior figure's entry SHALL reproduce the established
presentation: texture warrior.png, frame size 12×15 pixels, atlas 21
columns by 8 rows; Idle frame sequence 0,0,0,1,0,0,1,1 at 8 fps,
looping; Run
frame sequence 2,3,4,5,6,7 at 20 fps, looping; Attack frame sequence
13,14,15,0 at 15 fps, non-looping; Die frame sequence 8,9,10,11,12,11
at 20 fps, non-looping.

warrior 形象的条目 SHALL 复现既定表现：贴图 warrior.png，帧尺寸 12×15
像素，图集 21 列 8 行；Idle 帧序列为 0,0,0,1,0,0,1,1、帧率 8 fps、循
环；Run
帧序列为 2,3,4,5,6,7、帧率 20 fps、循环；Attack 帧序列为
13,14,15,0、帧率 15 fps、不循环；Die 帧序列为 8,9,10,11,12,11、帧
率 20 fps、不循环。

#### Scenario: The entry matches the established presentation value for value | 数据条目与既定表现逐值一致

- **WHEN** the repository's figure table is read | 读取仓库中的形象表时
- **THEN** the warrior entry's texture, frame size, grid dimensions, and the Idle, Run, Attack, and Die frame sequences, rates, and looping flags match the values above item for item | warrior 条目的贴图、帧尺寸、行列数、Idle/Run/Attack/Die 帧序列、帧率与循环标志与上述数值逐项一致

### Requirement: Giant White Rat Figure Data | 巨白鼠形象数据

The giant_white_rat figure's entry SHALL reproduce the established
presentation: texture rat.png, frame size 16×15 pixels, atlas 16
columns by 2 rows, all frames on the sheet's second (white-variant)
row; Idle frame sequence 16,16,16,17 at 2 fps, looping; Run frame
sequence 22,23,24,25,26 at 10 fps, looping; Attack frame sequence
18,19,20,21,16 at 15 fps, non-looping; Die frame sequence 27,28,29,30
at 10 fps, non-looping.

giant_white_rat 形象的条目 SHALL 复现既定表现：贴图 rat.png，帧尺寸
16×15 像素，图集 16 列 2 行，全部帧取自贴图第二行（白色变体）；
Idle 帧序列为 16,16,16,17、帧率 2 fps、循环；Run 帧序列为
22,23,24,25,26、帧率 10 fps、循环；Attack 帧序列为
18,19,20,21,16、帧率 15 fps、不循环；Die 帧序列为 27,28,29,30、帧
率 10 fps、不循环。

#### Scenario: The entry matches the established presentation value for value | 数据条目与既定表现逐值一致

- **WHEN** the repository's figure table is read | 读取仓库中的形象表时
- **THEN** the giant_white_rat entry's texture, frame size, grid dimensions, and the Idle, Run, Attack, and Die frame sequences, rates, and looping flags match the values above item for item | giant_white_rat 条目的贴图、帧尺寸、行列数与 Idle/Run/Attack/Die 帧序列、帧率及循环标志与上述数值逐项一致
