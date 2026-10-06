# dice Specification | dice 规格

## Purpose

Dice notation: the unified form and rolling semantics for random
quantities in vocabulary data. Monster hit dice and attack damage dice
share this form; parsing happens at vocabulary load, and rolls draw from
the injected random source.

骰子格式：词表数据表达随机量的统一形态与掷骰语义。怪物生命骰与攻击伤害骰
共用这一形态；解析在词表加载期完成，掷骰取数自注入的随机源。

## Requirements

### Requirement: Dice Notation | 骰子格式

A dice notation SHALL be the exact form "positive integer d positive
integer": a lowercase letter d separates the dice count from the face
count, both are integers no less than 1, and no other character may
appear. Any deviation — uppercase D, zero, a missing side, trailing
characters — MUST be rejected with an error describing the legal form.

骰子格式 SHALL 是"正整数 d 正整数"的精确形态：小写字母 d 分隔骰数与面数，
骰数与面数均为不小于 1 的整数，形态之外不得有任何字符。任何偏离——
大写 D、零、缺失一侧、多余字符——MUST 被解析拒绝，错误说明合法形态。

#### Scenario: Legal forms parse | 合法格式解析成功

- **WHEN** parsing "2d2" and "1d3" | 解析 "2d2" 与 "1d3" 时
- **THEN** the results are 2 dice of 2 faces and 1 die of 3 faces | 分别得到骰数 2、面数 2 与骰数 1、面数 3

#### Scenario: Illegal forms are rejected | 非法格式被拒绝

- **WHEN** parsing any of "0d2", "2d0", "2D2", "2d", "d6", "2d2x" | 解析 "0d2"、"2d0"、"2D2"、"2d"、"d6"、"2d2x" 之一时
- **THEN** parsing is rejected with an error describing the legal form | 解析被拒绝，错误说明格式的合法形态

### Requirement: Roll Semantics | 掷骰语义

A roll SHALL draw N uniform integers in 1..=M from the injected random
source and return their sum.

掷骰 SHALL 向注入的随机源取 N 个 1..=M 的均匀整数并求和。

#### Scenario: The sum lands in range | 求和落在值域内

- **WHEN** rolling any notation NdM repeatedly | 对任意格式 NdM 连续掷骰时
- **THEN** every result lies between N and N×M inclusive | 每次结果都在 N 到 N×M 之间（含两端）

#### Scenario: The source decides the result | 结果由随机源状态决定

- **WHEN** two random sources with the same seed each roll the same notation | 以两个相同种子的随机源分别掷同一格式时
- **THEN** the two results are equal | 两次结果相等
