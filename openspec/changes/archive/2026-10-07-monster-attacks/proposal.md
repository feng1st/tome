# Proposal: monster-attacks | 变更提案：monster-attacks

## Why

The fifth entry of the minimal combat loop (OPEN_ISSUES entries 1-6):
the monster strikes back, the player can be bitten dead, and the loop
closes. Every ingredient is landed — the monster vocabulary's combat
profile (blows, level, armor), the shared hit skeleton, the damage
channel with death handling, and the player's standing orders. What
does not exist is the monster-side half: nothing turns an adjacent
player into a strike, and nothing executes it.

最小战斗闭环（OPEN_ISSUES 条目 1-6）的第五条：怪物还手，主角能被
咬死，闭环在此合拢。素材已全部落地——怪物词表的战斗档案
（blows、等级、护甲）、共享命中骨架、伤害通道与死亡处理、玩家的
站立指令。缺的是怪物侧半环：没有任何东西把"相邻的主角"变成一
次出手，也没有任何东西执行它。

## What Changes

- Monster planning grows a highest-priority attack branch: when the
  living player stands in one of the eight neighboring cells, the
  monster's due turn plans a strike and is spent; otherwise the wander
  runs unchanged. `plan_wander` renames to `plan_action` — wandering
  becomes the fallback branch, not the system's whole job.

  怪物策划长出最高优先的攻击分支：存活主角站在八邻格之一时，到期
  回合策划出手并消耗；否则游荡照旧。`plan_wander` 改名
  `plan_action`——游荡降为回落分支，不再是系统的全部职责。

- Strike resolution flattens to a uniform component pipeline
  (**BREAKING** for the strike path): the attacker carries `Blows`
  (each blow a hit chance and a damage — fixed or a dice roll), the
  target carries
  `ArmorClass`; one `act_attack` system — registered in both
  `PlayerAct` and `WorldAct`, the `act_move` template — resolves
  every strike for every pairing (player→monster, monster→player, and
  monster→monster alike) without reading any registry or asking what
  the entity is.

  出手解析压平为统一组件管线（对出手路径为 **BREAKING**）：攻击方
  携带 `Blows`（每击各带命中品质与伤害——定值或掷骰），目标携带
  `ArmorClass`；单一 `act_attack` 系统——注册进 `PlayerAct` 与
  `WorldAct` 两个阶段，照 `act_move` 模板——为一切配对（玩家→
  怪、怪→玩家、怪→怪亦然）解析出手，不读任何词表、不问实
  体是谁。

- The player-monster difference moves to the producers: the player's
  `Blows` and `ArmorClass` derive from the six statistics in the
  Derive phase (existing formulas and stand-ins unmoved); the
  monster's translate from the vocabulary row at spawn. An absent
  component switches its mechanic off — no armor class means armor
  takes no part in the hit check; no blows means the creature cannot
  strike (the reference's NEVER_BLOW expressed without a flag).

  玩家与怪物的差异移到生产侧：玩家的 `Blows` 与 `ArmorClass` 在
  Derive 阶段由六维派生（既有公式与顶替值原地不动）；怪物的在出
  生时从词表行翻译。组件缺席即机制关闭——无护甲组件则护甲不参
  与命中判定；无打击组件则该生物出不了手（参照的 NEVER_BLOW 不
  用旗标即得表达）。

- The monster's hit quality carries a fixed power stand-in 60 —
  mapping to the reference's HURT effect power (the effect family
  does not exist; the rat's literal POISON power 5 is reference-only,
  since nothing attaches an effect here). The stand-in deletes when
  the effect family lands (OPEN_ISSUES entry 10). Each blow judges
  the hit and rolls its damage on its own, the reference shape; the
  rat's single blow behaves identically either way.

  怪物的命中品质携带固定威力顶替值 60——映射参照的 HURT effect
  威力（effect 家族不存在；老鼠字面的 POISON 威力 5 仅作参考，
  此处并无效果附加）。顶替值随 effect 家族落地（条目 10）删除。
  每击独立判定命中、独立掷伤害，即参照形状；老鼠单击，两种形状
  行为相同。

- Player-side normalization riding along: the existing `act_attack`'s
  target-armor read leaves `MonsterRegistry` for the target's
  `ArmorClass` component, and `MonsterKind`'s `level`/`blows` shed
  their dead-code allowances.

  顺手归一玩家侧：既有 `act_attack` 的目标护甲读取离开
  `MonsterRegistry`、改读目标的 `ArmorClass` 组件；
  `MonsterKind` 的 `level`/`blows` 摘除 dead_code 豁免。

## Capabilities

### New Capabilities

(none —— 无新增能力)

### Modified Capabilities

- `combat`: hit determination reads each blow's chance and the
  target's optional armor class at strike time, instead of the
  player-side formulas directly; the player's blows and armor class
  are derived production. The attack-order and formula requirements
  keep their numbers.

  命中判定在出手时改读每击的命中品质与目标的可选护甲，不再直接
  消费玩家侧公式；玩家的打击与护甲改为派生产物。攻击指令与公式
  条款的数值不变。

- `monster`: a new Monster Strike requirement (the adjacent-player
  attack branch); Monster Spawning carries the blows and the armor; the
  wander gains the no-adjacent-player precondition.

  新增"怪物出手"条款（相邻主角攻击分支）；怪物出生携带打击与护
  甲；游荡增加"无相邻活主角"前提。

## Impact

- Code: the combat domain gains the `Blows`/`ArmorClass` components
  and their value types, the player's strike derivation,
  and the uniform `act_attack` rewrite; the monster domain gains the
  spawn translation and the `plan_action` rename with its attack
  branch. No data-schema or save impact.

  代码：combat 域新增 `Blows`/`ArmorClass` 组件与值类型、玩
  家打击派生、`act_attack` 统一重写；monster 域新增出生翻译与
  `plan_action` 改名及其攻击分支。无数据模式或存档影响。

- OPEN_ISSUES: entry 5 deletes on landing; entry 10's recovery note
  gains the power stand-in's home.

  登记表：条目 5 随落地删除；条目 10 的回收注记补威力顶替值的落
  点。

- Tests: the `act_attack` tests rewrite around the components; the
  monster planner gains attack-branch tests; the whole-app
  regression gains the rat-bites-player half.

  测试：`act_attack` 测试按组件形状重写；怪物策划补攻击分支测
  试；完整应用回归补"老鼠咬主角"半环。
