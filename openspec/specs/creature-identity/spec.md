# creature-identity Specification | creature-identity 规格

## Purpose

Humanoid identity and figure binding: a humanoid creature declares its
identity by a race (required) and a class (optional), and identity
handles never change once attached; a monster's identity is its kind
in the monster vocabulary. The display side resolves the figure
through its binding table, taking the most specific match in the order
monster key, race+class, race.

人形生物的身份与形象绑定：人形 creature 以 race（必需）与 class（可
选）声明身份，身份句柄挂载后不变；怪物以怪物词表的 kind 为身份。形象
由 display 的绑定表按 monster 键、race+class、race 的顺序取最具体匹配
解析。

## Requirements

### Requirement: Creature Identity Components | 生物的身份组件

A humanoid creature SHALL carry a race handle to declare its race, and
a class handle as an optional component; a monster carries neither a
race nor a class handle — its identity is its kind in the monster
vocabulary. Once attached, an identity handle component SHALL NOT
change at runtime; the figure binding resolves once when the identity
component is attached, with no runtime re-resolution. Mechanics that
change the body — possession, polymorph, vampirization — are not part
of this capability and will be proposed separately.

人形 creature SHALL 携带 race 句柄以声明其种族，class 句柄为可选组件；
怪物不携带 race 或 class 句柄，其身份由怪物词表的 kind 承担。身份句柄
组件挂载后 SHALL NOT 在运行期变更；形象绑定在身份组件挂载时一次解
析，不做运行期重解析。附身、变形、吸血鬼化等改变身体的机制不属于本
能力，将来单独立项。

#### Scenario: The player's birth identity | 玩家出生身份

- **WHEN** the game enters a run and the player is born | 游戏进入对局、主角出生时
- **THEN** the player entity carries a race handle and a class handle | 主角实体携带 race 句柄与 class 句柄

#### Scenario: A monster's birth identity | 怪物出生身份

- **WHEN** a monster entity spawns from the map's spawn table | 怪物实体按地图出生表出生时
- **THEN** the entity carries the monster handle and neither a race nor a class handle | 该实体携带怪物句柄，不携带 race 或 class 句柄

### Requirement: Figure Binding | 形象绑定

The creature-to-figure mapping SHALL be defined by a display data
file: each binding entry declares an identity in one of three key
shapes — monster, race+class, or race — and points to a figure id.
Monsters resolve through the monster key; humanoids take the most
specific match in the order race+class then race, precedence being
monster > race+class > race. A race whose members always bear a class
may exist only through race+class entries, with no "bare" figure.
Every declared race MUST be covered by at least one entry (a race key
or a race+class key), and every declared monster MUST be covered by at
least one monster-key entry; an identity matching no entry SHALL fail
at attach time with an error naming that identity. Code MUST NOT
contain the identity-to-figure mapping.

creature 到形象的映射 SHALL 由 display 的数据文件定义：每个绑定条目以
monster、race+class、race 三种键形态之一声明身份，并指向一个形象 id。
怪物按 monster 键解析；人形按 race+class → race 的顺序取最具体匹配，
优先级为 monster > race+class > race。成员必带职业的种族可以只有组合键
条目，无"裸"形象。每个声明的 race MUST 至少被一条条目（race 键或组合
键）覆盖，每个声明的 monster MUST 至少被一条 monster 键条目覆盖；一个
身份未命中任何条目时 SHALL 在挂载时以指明该身份的错误失败。代码
MUST NOT 包含身份到形象的映射。

#### Scenario: A monster key resolves a monster | monster 键解析怪物

- **WHEN** a monster entity carries a monster handle and the binding table has an entry keyed by that monster id | 怪物实体携带 monster 句柄，且绑定表存在以该 monster id 为键的条目时
- **THEN** the entity presents the figure that monster entry points to | 该实体呈现该 monster 条目指向的形象

#### Scenario: A race+class key beats the race default | 组合键优先于 race 默认

- **WHEN** a humanoid creature carries race and class handles and the binding table has an entry keyed by that (race, class) | 人形 creature 携带 race 与 class 句柄，绑定表存在以该 (race, class) 为键的条目时
- **THEN** the entity presents the figure that race+class entry points to | 该实体呈现该组合条目指向的形象

#### Scenario: Falling back to the race default | 回退到 race 默认

- **WHEN** a humanoid creature matches no (race, class) entry | 人形 creature 无 (race, class) 条目命中时
- **THEN** the entity presents the figure its race's race-key entry points to | 该实体呈现其 race 的 race 键条目指向的形象

#### Scenario: A race with no binding fails startup | race 无任何绑定拒绝启动

- **WHEN** a race declared in the race vocabulary has neither a race-key entry nor any race+class entry in the binding table | race 词表声明的某个 race 在绑定表中既无 race 键条目也无任何组合键条目时
- **THEN** startup fails, the error naming the file and that race id | 启动失败，错误信息指明出错文件与该 race id

#### Scenario: A monster with no binding fails startup | monster 无任何绑定拒绝启动

- **WHEN** a monster declared in the monster vocabulary has no monster-key entry in the binding table | 怪物词表声明的某个 monster 在绑定表中无 monster 键条目时
- **THEN** startup fails, the error naming the file and that monster id | 启动失败，错误信息指明出错文件与该 monster id

#### Scenario: An identity matching no binding fails at attach | 身份未命中绑定挂载即失败

- **WHEN** a creature's identity matches nothing in the binding table (such as a classless member of a class-keyed race) | creature 的身份在绑定表中无匹配（如无 class 的人形成员）时
- **THEN** attaching fails with an error naming that identity | 挂载时以指明该身份的错误失败

### Requirement: Current Identity and Binding Content | 当前身份与绑定内容

The race vocabulary SHALL declare human, the class vocabulary SHALL
declare warrior, and the monster vocabulary SHALL declare
giant_white_rat; the current content SHALL NOT declare any unique
individual. The player SHALL carry race human and class warrior. The
binding table SHALL contain: a race+class entry for (human, warrior)
pointing to figure warrior, and a monster-key entry for monster
giant_white_rat pointing to figure giant_white_rat. A classless human
has no figure to bind (no matching art), so the binding table SHALL
NOT contain a race-key entry for human.

race 词表 SHALL 声明 human，class 词表 SHALL 声明 warrior，怪物词表
SHALL 声明 giant_white_rat；当前内容 SHALL NOT 声明任何 unique 个体。
主角 SHALL 携带 race human 与 class warrior。绑定表 SHALL 包含：
(human, warrior) 的组合键条目指向形象 warrior；monster giant_white_rat
的 monster 键条目指向形象 giant_white_rat。无 class 的 human 无形象可
绑（无对应素材），绑定表 SHALL NOT 包含 human 的 race 键条目。

#### Scenario: Figures in the test room | 测试房间的形象呈现

- **WHEN** the game starts into the test room | 启动游戏进入测试房间时
- **THEN** the player presents the warrior figure and the two rats present the giant_white_rat figure | 主角呈现 warrior 形象，两只老鼠呈现 giant_white_rat 形象
