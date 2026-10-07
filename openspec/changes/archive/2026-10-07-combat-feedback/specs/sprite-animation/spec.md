# sprite-animation Specification (delta) | sprite-animation 规格（增量）

## Purpose

The frame-animation mechanism: a creature's sprite plays the frame
tables of its figure — the standing derivation of idle and run, the
one-shot action animations of attacks, and the death animation that
overrides everything — and a one-shot's end is announced so other
presentation systems can follow it.

帧动画机制：生物的精灵播放其形象的帧表——待机与奔跑的常驻推导、
攻击的一次性动作动画、凌驾一切的死亡动画——一次性动画播毕发出公
告，供其它呈现系统衔接。

## ADDED Requirements

### Requirement: Attack Presentation | 攻击呈现

When an attack-resolved fact arrives, the attacker's sprite SHALL face
the target cell (horizontal mirror) and play its attack animation as a
one-shot — whether or not any blow landed, since the swing happens
either way. An attacker no longer in the world SHALL be skipped.

攻击结果事实到达时，攻击方的精灵 SHALL 朝目标格转向（水平翻转）并
以一次性方式播放其攻击动画——无论有无命中，挥击都已发生。攻击方
已不在世界中时 SHALL 跳过。

#### Scenario: A landed attack plays the attack animation | 命中攻击播放攻击动画

- **WHEN** an attack-resolved fact arrives naming a living attacker and a target cell to its left | 攻击结果事实到达，攻击方存活、目标格在其左侧时
- **THEN** the attacker faces left and plays its attack animation once | 攻击方面向左并完整播放一次攻击动画

#### Scenario: A full miss still swings | 全失照样挥击

- **WHEN** an attack-resolved fact arrives whose `damage_amounts` is empty | 攻击结果事实的 `damage_amounts` 为空时
- **THEN** the attacker still faces the target and plays the attack animation | 攻击方仍朝目标转向并播放攻击动画

#### Scenario: A gone attacker is skipped | 攻击方已离场则跳过

- **WHEN** an attack-resolved fact arrives naming an attacker that no longer exists | 攻击结果事实指名的攻击方已不存在时
- **THEN** no animation is started | 不启动任何动画

### Requirement: One-Shot Animation Lock | 一次性动画锁

A frame table marked non-looping SHALL clamp playback at its last
frame instead of wrapping. While a one-shot action animation holds on
a creature, idle and run derivation SHALL NOT switch its animation;
when the one-shot completes, derivation SHALL resume. The death
marker SHALL override everything, cutting short even an active lock.

标记为不循环的帧表 SHALL 把播放钳在末帧，不回卷。一次性动作动画
在生物身上存续期间，待机与奔跑推导 SHALL NOT 切换其动画；一次性
动画播完后 SHALL 恢复推导。死亡标记 SHALL 凌驾一切，存续中的锁
也立即打断。

#### Scenario: A one-shot clamps at its last frame | 一次性动画定格末帧

- **WHEN** a non-looping animation's playback reaches past its final frame | 不循环动画的播放越过末帧时
- **THEN** the last frame keeps showing instead of wrapping to the first | 画面定格末帧，不回卷到首帧

#### Scenario: The lock holds against derivation | 锁存续期推导让位

- **WHEN** a one-shot action animation is playing and the creature stands still | 一次性动作动画播放中且生物静止时
- **THEN** the one-shot plays to completion rather than switching to idle | 一次性动画播至完毕，不切回待机

#### Scenario: Death cuts the lock short | 死亡凌驾锁

- **WHEN** a creature gains the death marker mid-way through an attack animation | 生物在攻击动画播到一半时获得死亡标记时
- **THEN** the animation switches to the death animation immediately | 动画立即切换为死亡动画

### Requirement: Death Animation | 死亡动画

Every dead creature SHALL play the death animation on its own body
and hold its last frame — the marked monster lingering through its
fade just as the marked player lingers through the frozen world.

一切死亡的生物 SHALL 在其本体上播放死亡动画并定格末帧——被标记
的怪物在其淡出中滞留，正如被标记的玩家在冻结的世界中滞留。

#### Scenario: The dead play their death animation on their own bodies | 死者在各自本体上播放死亡动画

- **WHEN** a creature carries the death marker | 一只生物携带死亡标记时
- **THEN** its own sprite plays the death animation once and holds the last frame | 其本体精灵完整播放一次死亡动画并定格末帧

### Requirement: Animation Completion Announcement | 动画播毕公告

A one-shot animation that reaches its last frame SHALL announce its
completion once, naming the entity and the animation — the
notification that lets other presentation systems follow an
animation's end. A one-shot cut short by a higher-priority switch
SHALL announce no completion.

抵达末帧的一次性动画 SHALL 把播毕公告一次，指明实体与动画——其
它呈现系统凭此衔接动画的结束。被更高优先级切换打断的一次性动画
SHALL 不公告播毕。

#### Scenario: The death animation's end is announced | 死亡动画播毕有公告

- **WHEN** a creature's death animation reaches its last frame | 生物的死亡动画抵达末帧时
- **THEN** a completion announcement names the creature and the death animation | 播毕公告指明该生物与死亡动画

#### Scenario: An interrupted one-shot stays silent | 被打断的一次性不公告

- **WHEN** the death marker cuts an attack animation short | 死亡标记打断攻击动画时
- **THEN** the attack animation announces no completion | 该攻击动画不公告播毕
