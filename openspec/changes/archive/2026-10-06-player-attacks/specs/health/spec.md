# health Specification (delta) | health 规格（增量）

## ADDED Requirements

### Requirement: Damage Interrupts Standing Orders | 伤害打断站立指令

Applying damage to a creature SHALL clear the target's standing
order — whether it is a move order or an attack order; a creature that
takes no damage SHALL keep its order. The rule lives at the damage channel's
application point, so no damage source can bypass it.

对生物施加伤害 SHALL 清除该目标的站立指令——无论它是移动指令还
是攻击指令；未受伤害的生物 SHALL 保留其指令。该规则落在伤害通道的施加点，任
何伤害来源都无法绕过。

#### Scenario: A hit clears the target's orders | 命中清除目标指令

- **WHEN** damage is applied to a creature holding a standing order | 对持有站立指令的生物施加伤害时
- **THEN** the target's standing order is gone after the application | 施加之后目标的站立指令消失

#### Scenario: No damage, no interruption | 无伤害无打断

- **WHEN** a strike misses and no damage is applied | 出手未命中、没有伤害被施加时
- **THEN** the target's standing order is unchanged | 目标的站立指令不变
