历史归档，不符合先英文后中文规范，请勿参考

# health Specification (delta)

## ADDED Requirements

### Requirement: 怪物生命值派生

怪物的生命值上限 SHALL 在出生期由其词表生命骰掷出：上限等于 N 个
1..=M 之和（N、M 为该条目生命骰的骰数与面数），不经六维；当前值初始
等于上限。

#### Scenario: 上限为生命骰的一次掷出

- **WHEN** 词表条目声明生命骰 NdM 且对应怪物出生
- **THEN** 其生命值上限落在 N 到 N×M 之间（含两端），且与六维无关

#### Scenario: 当前值初始等于上限

- **WHEN** 怪物出生
- **THEN** 生命值当前值等于上限
