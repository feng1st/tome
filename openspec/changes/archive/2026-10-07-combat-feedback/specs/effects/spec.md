# effects Specification (delta) | effects 规格（增量）

## Purpose

The display-side effects of combat: the effects domain reads the
core's combat facts (damage applied, creatures died) and renders
them as game-content effects — the wounded body's white flash, the
floating damage number, the blood splash, the dead body's fade, and
the heavy hit's camera-shake trigger. Effects compose mechanisms and
own no timing machinery of their own: fades are carried by tweens,
splashes by burst particles, shakes by camera requests.

战斗的展示侧效果：effects 域读取核心的战斗事实（伤害落地、生物死
亡），把它们呈现为游戏内容对应的效果——受击躯体的白闪、伤害数字
飘字、血粒子迸溅、死亡躯体的淡出、重击的相机震动触发。effects 编
排机制而不自持计时设施：淡出由 tween 承载、迸溅由粒子承载、震动
由相机请求承载。

## ADDED Requirements

### Requirement: Hit Flash | 受击白闪

When damage is applied to a creature still in the world, its sprite
SHALL be overexposed — its tint multiply-boosted toward white — for
0.05 seconds, then restored.

伤害施加于仍在世界中的生物时，其精灵 SHALL 过曝——着色乘性拉向
白色——持续 0.05 秒后复原。

#### Scenario: A wounded creature flashes briefly | 受伤生物短暂白闪

- **WHEN** damage is applied to a creature that stays in the world | 伤害施加于仍在世界中的生物时
- **THEN** its sprite shows overexposed for 0.05 seconds and then returns to normal | 其精灵过曝显示 0.05 秒后恢复正常

#### Scenario: The killing blow flashes before the death takes over | 致命一击白闪后被死亡接管

- **WHEN** applied damage kills the creature | 施加的伤害致死时
- **THEN** the flash blinks, and the death presentation takes the body over from there | 白闪一瞬，死亡呈现随即接管躯体

### Requirement: Damage Number Text | 伤害数字飘字

Each damage-applied fact SHALL spawn world-space text showing the
damage amount, positioned at the wounded sprite's top-center. The text
SHALL rise one tile over one second of life and fade during the
latter half of that life. Its color SHALL report the target's
remaining condition: orange 0xFF8800 while the target's current hit
points exceed half the maximum, red 0xFF0000 at or below half; a
target already removed reads as red. An attack that lands no damage
SHALL spawn no text. Texts alive on the same cell SHALL stack: a
newcomer pushes the cell's living texts up by one line.

每条伤害落地事实 SHALL 生成显示伤害数值的世界空间文字，位置在受
击精灵的顶部中央。文字 SHALL 在一秒生命内上浮一格，并在生命后
半程淡出。颜色 SHALL 报告目标伤后状态：目标当前生命值高于上限
一半为橙 0xFF8800，一半及以下为红 0xFF0000；目标已离开世界按
红处理。未造成任何伤害的攻击 SHALL 不生成文字。同格存活的飘字
SHALL 堆叠：新到的把该格存活飘字上挤一行。

#### Scenario: A hit floats its amount | 命中飘出数值

- **WHEN** damage of 4 is applied to a creature at full hit points | 对满血生物施加 4 点伤害时
- **THEN** world-space text "4" appears at its sprite's top-center, orange, rising one tile over one second | 其精灵顶部中央出现橙色的世界空间文字"4"，一秒内上浮一格

#### Scenario: Below half the maximum turns red | 低于半血转红

- **WHEN** damage lands on a creature whose current hit points fall to half the maximum or below | 伤害落地且目标当前生命值降至上限一半及以下时
- **THEN** the spawned text is red 0xFF0000 | 生成的文字为红 0xFF0000

#### Scenario: A miss spawns no text | 未命中无飘字

- **WHEN** an attack resolves without any damage applied | 攻击解析未产生任何伤害落地时
- **THEN** no floating text spawns | 不生成任何飘字

#### Scenario: Same-cell texts stack upward | 同格飘字向上堆叠

- **WHEN** a second damage text spawns on a cell while a first is still alive there | 某格第二条伤害飘字生成时第一条仍存活时
- **THEN** the first text shifts up by one line and the second takes its place | 第一条上移一行，第二条占据其原位置

### Requirement: Blood Splash | 血粒子迸溅

Each damage-applied fact SHALL burst blood particles at the wounded
sprite's center: the count SHALL be min(9·√(amount ÷ maximum), 9),
so a zero amount bursts nothing; the color SHALL be 0xFFBB0000.
Particles SHALL fly within a 90° cone around the direction from the
source cell to the target cell; damage without a source SHALL spray
in an upward 180° fan.

每条伤害落地事实 SHALL 在受击精灵中心迸溅血粒子：数量 SHALL 为
min(9·√(伤害 ÷ 上限), 9)，伤害为零则不迸溅；颜色 SHALL 为
0xFFBB0000。粒子 SHALL 在以来源格指向目标格为轴的 90° 锥内飞
出；无来源的伤害 SHALL 以朝上的 180° 扇形喷射。

#### Scenario: Count follows the wound's weight | 数量随伤害占比

- **WHEN** damage equal to the maximum lands | 数额等于上限的伤害落地时
- **THEN** nine blood particles burst | 迸溅九粒血粒子

#### Scenario: A zero amount bursts nothing | 零伤害不迸溅

- **WHEN** damage of zero is applied | 施加零数额伤害时
- **THEN** no blood particles spawn | 不生成血粒子

#### Scenario: The spray follows the blow | 喷射沿打击方向

- **WHEN** damage lands whose source cell sits directly left of the target cell | 来源格位于目标格正左方的伤害落地时
- **THEN** the particles fly rightward within a 90° cone | 粒子在 90° 锥内向右飞出

### Requirement: Death Fade | 死亡淡出

Every fresh death but the driver's SHALL be claimed for its exit
presentation: the body gains the disappearance flag — the core-side
protocol that holds the departure sweep off — when its death
presentation takes it. The fade SHALL open when the death animation
ends, run over three seconds, and carry the body's sprite to
transparency; at its end the flag SHALL drop, releasing the body to
the departure sweep. The driver is never claimed: its death
presentation is the frozen world, and the death wrap-up owns its
exit.

除驾驶者外的一切新死 SHALL 被认领进入离场呈现：死亡呈现接管躯体
时，躯体获得消失存续标志（核心侧协议，拦住离场清扫）。淡出 SHALL
在死亡动画播毕时开启、历时三秒，把躯体的精灵带到透明；到点标志
SHALL 降下，把躯体交还给离场清扫。驾驶者永不被认领：其死亡呈现
是冻结的世界，离场归死亡收尾。

#### Scenario: A fresh death is claimed | 新死被认领

- **WHEN** a creature gains the death marker | 一只生物获得死亡标记时
- **THEN** its body carries the disappearance flag through its exit presentation, and the fade opens when its death animation ends and closes three seconds later | 其躯体在离场呈现期间携带消失存续标志，淡出于死亡动画播毕开启、三秒后结束

#### Scenario: The body fades through the fade and releases the world | 躯体淡出并释放世界

- **WHEN** the fade runs and then ends | 淡出运行并结束时
- **THEN** the sprite is transparent, the flag drops, and the body leaves on the next departure sweep | 精灵透明，标志降下，躯体在下一次离场清扫中离开

#### Scenario: The dead driver is never claimed | 死亡驾驶者永不被认领

- **WHEN** the driver gains the death marker | 驾驶者获得死亡标记时
- **THEN** it carries neither the flag nor a fade, and stays until the death wrap-up | 它既无标志也无淡出，留待死亡收尾

### Requirement: Heavy-Hit Shake Trigger | 重击震动触发

When applied damage to the player exceeds a quarter of the player's
hit-point maximum, a camera-shake request SHALL issue with the
magnitude gated to the 1..5 band by the damage proportion — amount
divided by a quarter of the maximum — and a duration of 0.3 seconds;
damage at or below the quarter SHALL request nothing.

施加于玩家的伤害超过其生命上限四分之一时 SHALL 发出相机震动请
求：幅度按伤害占比——数额除以上限的四分之一——钳制在 1..5
区间，时长 0.3 秒；不超过四分之一的伤害 SHALL 不发请求。

#### Scenario: A heavy hit shakes | 重击震动

- **WHEN** the player at maximum 20 takes damage of 10 | 上限 20 的玩家受到 10 点伤害时
- **THEN** a shake request issues with magnitude 2 and duration 0.3 seconds | 发出幅度 2、时长 0.3 秒的震动请求

#### Scenario: A light hit stays still | 轻击不震动

- **WHEN** the player at maximum 20 takes damage of 5 | 上限 20 的玩家受到 5 点伤害时
- **THEN** no shake request issues | 不发出震动请求
