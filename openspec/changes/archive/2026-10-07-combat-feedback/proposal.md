# Proposal: combat-feedback | 变更提案：combat-feedback

## Why

The sixth and last entry of the minimal combat loop (OPEN_ISSUES
entries 1-6). Combat already resolves end to end — attacks, damage,
death — but invisibly: an attack consumes its action component without
a trace, a miss writes nothing at all, the damage queue is drained
destructively, and a dead monster despawns in the same frame. The
display side observes none of it, so the player sees creatures
vanish and hit points change without cause. This entry lets the
display side see combat and gives the loop its minimal visible
surface, aligned with the reference implementation's presentation.

最小战斗闭环（OPEN_ISSUES 条目 1-6）的第六条，即收尾条。战斗已经
端到端走完——攻击、伤害、死亡——但全程不可见：攻击消耗行动组
件后无迹可寻，未命中什么都不产生，伤害队列被破坏式排干，死亡
的怪物在同一帧消失。展示侧观察不到其中任何一环，玩家只能看到
生物凭空消失、生命值无故变动。本条让展示侧看得见战斗，为闭环
补上最小的可见外观，表现对齐参考实现。

## What Changes

- Combat facts in core. Attack resolution emits an attack-resolved
  fact — the attacker, both cells, and the landed blows' damage
  amounts, emitted even on a full miss; damage application emits a
  damage-applied fact and marks the dead with a component (`Dead`),
  not an event. Facts are past-tense broadcasts consumed through ordinary
  message readers — unlike the damage request queue, which keeps its
  destructive drain.

  核心侧的战斗事实。攻击解析发出"攻击结果"事实——攻击者、双
  方格、各命中击的伤害数值，全失也照发；伤害施加发出"伤害落
  地"事实，并以组件（`Dead`）标记死亡，不立事件。事实是过去时广播，
  经普通消息读取器消费——与保持破坏式排干的伤害请求队列语义
  不同。

- Attack and die animations. The figure table gains attack and die
  anim entries using frames already present in the existing textures
  (warrior: attack frames 13-15, die frames 8-12; giant white rat,
  on its white-variant row: attack frames 18-21, die frames 27-30 —
  no new art). Animation
  selection gains one-shot action animations with a playback lock: a
  one-shot runs to completion before the idle/run derivation takes
  over again. Combat animation never holds the world clock — the
  movement-only freeze convention is untouched, matching the
  reference implementation, whose actor loop never waits for sprite
  animation.

  攻击与死亡动画。形象表新增攻击与死亡动画条目，帧全部取自现有
  贴图（战士：攻击帧 13-15、死亡帧 8-12；大白鼠取白色变体行：攻击
  帧 18-21、死亡帧 27-30——零新增素材）。动画选择新增一次性动作动画与播
  放锁：一次性动画播完后，idle/run 推导才重新接管。战斗动画不
  阻挡世界时钟——只冻结移动的既有约定不动，与参考实现一致：
  其行动循环从不等待动画。

- Hit feedback. A wounded target flashes white; a landed wound bursts
  blood particles (blood color 0xFFBB0000, count
  min(9*sqrt(damage/maximum), 9), a fan spray at 40-80 px/s under
  gravity 100 px/s², shrinking out over 0.5-1s). When the player
  takes a hit exceeding a quarter of their maximum, the camera
  shakes (magnitude gated to 1-5 by damage proportion, 0.3s).

  受击反馈。受击的目标白闪；落地的伤害迸溅血粒子（血色
  0xFFBB0000，数量 min(9*sqrt(伤害/上限), 9)，40-80 px/s 扇形
  喷射、重力 100 px/s²，0.5-1 秒内缩没）。玩家受到超过上限四
  分之一的重击时，相机震动（幅度按伤害比例钳制在 1-5，时长
  0.3 秒）。

- Death presentation. Death is the marker: any creature whose hit
  points cross below zero gains `Dead`, identity-free, and its body
  lingers as inert remains. The body plays its own death animation,
  then fades over 3 seconds under the display's claim — the
  `IsDisappearing` protocol flag, held through the presentation,
  gates the core's departure sweep (`despawn_dead`); with no display
  running, the dead leave on the next sweep. The driver is never
  claimed: its death presentation is the frozen world, and the death
  wrap-up (entry 13) owns its exit.

  死亡表现。死亡即标记：任何越零的生物获得 `Dead`，无身份分
  支，其躯体作为惰性遗存滞留。躯体自演死亡动画，随后在展示侧
  的认领下 3 秒淡出——`IsDisappearing` 协议旗贯穿呈现，拦住核
  心的离场清扫（`despawn_dead`）；无展示运行时，死者下一扫即
  离场。驾驶者永不被认领：其死亡呈现是冻结的世界，离场归死亡
  收尾（条目 13）。

- Damage numbers as floating text. A small Latin bitmap-text
  mechanism lands in the display side, ported from the reference
  implementation: the font texture is copied whole (no slicing), and
  glyphs are split at load by its transparent marker columns, paired
  with a hardcoded character table plus per-font baseline and
  tracking values. All five fonts are copied and picked by the zoom,
  exactly the reference's zoom-matched selection. A landed wound
  spawns the damage amount as world-space text that floats up one
  tile and fades out over one second. The message system (entry 14)
  will reuse this mechanism.

  伤害数字飘字。展示侧落地一套小型拉丁位图字体机制，移植自参考
  实现：字体贴图整份复制（不切不裁），加载时按透明标记列切分
  字形，配合硬编码字符表与每字体的基线、字距值。五份字体全
  部复制、按缩放选字体，正是参考实现的选字方式。落地的伤口生成
  伤害数值的世界空间文字，上浮一格并在一秒内淡出。消息
  系统（条目 14）届时复用这套机制。

- Out of this entry: any UI (the hero's HUD health bar and monster
  overhead bars — the reference has no overhead bars outside its
  mob-inspection window, so neither is built); miss wording text
  (belongs to the message system, entry 14); holding the world clock
  for combat animation.

  不入本条：任何 UI（主角 HUD 血条与怪物头顶血条——参考实现在
  怪物详情窗口之外没有头顶血条，故二者都不做）；未命中的措辞
  文字（属消息系统，条目 14）；战斗动画阻挡世界时钟。

## Capabilities

### New Capabilities

- `effects`: the display-side effects of combat facts — white flash
  and blood splash on a wounded target, floating damage numbers, the
  dead body's fade, and the camera-shake trigger on heavy hits to
  the player.

  `effects`：战斗事实的展示侧效果——受击目标的白闪与血粒子迸溅、
  伤害数字飘字、死亡躯体的淡出，以及玩家受重击时的相机震动触
  发。

- `sprite-animation`: the frame-animation mechanism — one-shot action
  animations with a playback lock, the attack and death switches
  driven by combat facts, and the completion announcement.

  `sprite-animation`：帧动画机制——带播放锁的一次性动作动画、由
  战斗事实驱动的攻击与死亡切换，以及播毕公告。

- `bitmap-text`: the Latin bitmap-text mechanism — font textures plus
  marker-column glyph splitting, character table, baseline and
  tracking, and text rendering as glyph sprites.

  `bitmap-text`：拉丁位图字体机制——字体贴图与标记列字形切分、
  字符表、基线与字距，以及逐字形 sprite 的文字渲染。

- `particles`: the pixel-particle mechanism — point particles with a
  polar-random spray speed, gravity, and a shrinking lifetime.

  `particles`：像素粒子机制——点粒子的极坐标随机喷射速度、重
  力与缩小中的生命周期。

### Modified Capabilities

- `combat`: attack resolution gains the emission of attack-resolved
  facts (landed or missed alike) alongside the damage requests.

  `combat`：攻击解析新增发出攻击结果事实（命中与未命中皆有），
  与伤害请求并行。

- `health`: damage application gains the emission of damage-applied
  and death facts; the destructive drain and the death semantics
  (monster removal, player marker) are unchanged.

  `health`：伤害施加新增发出伤害落地与死亡事实；破坏式排干与死
  亡语义（怪物移除、玩家标记）不变。

- `figure`: the warrior and giant-white-rat entries gain attack and
  die frame tables from their existing textures.

  `figure`：战士与大白鼠条目新增取自现有贴图的攻击与死亡帧表。

- `camera`: the camera gains a shake behavior — a decaying offset
  impulse on request.

  `camera`：相机新增震动行为——按请求施加随时间衰减的偏移。

- `player-animation`: the idle, run, and facing derivations yield
  while a one-shot action animation holds.

  `player-animation`：待机、奔跑与朝向推导在一次性动作动画存续期
  间让位。

## Impact

- Core: `src/core/combat/` (attack resolution emits facts) and
  `src/core/health/` (damage application emits facts and marks the
  dead, identity-free; `despawn_dead` sweeps the departed in the
  Derive phase). `src/core/world_clock/`: dead turns leave the
  schedule (the driver's excepted), and the world opens at zero with
  every creature's first turn slot there. No changes to resolution
  math or the damage channel's arithmetic.

  核心：`src/core/combat/`（攻击解析发事实）与
  `src/core/health/`（伤害施加发事实并标记死亡，无身份；`despawn_dead`
  在 Derive 阶段清扫离场者）。`src/core/world_clock/`：死亡回合退
  出调度（驾驶者例外），世界自零点开张、一切生物的首个回合槽都落
  在那里。解析数值与伤害通道的算术不变。

- Frontend, new domains under `src/frontend/display/`: the bitmap
  text, the particles, the tween mechanism, and the effects
  presentation (flash, floating text, blood splash, death fade,
  shake trigger).

  前端，`src/frontend/display/` 下新增域：位图字体、粒子、tween
  机制，以及 effects 呈现（白闪、飘字、血粒子、死亡淡出、震动
  触发）。

- Frontend, existing domains: `sprite_animation` (every animation
  switch, one-shot animations with their finish state, the attack
  branch, the death animation and claim), `motion` (cell changes
  drive position tweens), `camera` (shake).

  前端既有域：`sprite_animation`（一切动画切换、一次性动画与其
  播毕状态、攻击分支、死亡动画与认领）、`motion`（格变化驱动
  位置 tween）、`camera`（震动）。

- Data and assets: `data/graphic/figures.ron` gains attack and die
  anim entries for the warrior and the giant white rat; the five PD
  font textures are copied whole into `assets/`; a font table data
  file records per font the character table, split row height,
  baseline, and tracking.

  数据与素材：`data/graphic/figures.ron` 为战士与大白鼠新增攻
  击、死亡动画条目；五份 PD 字体贴图整份复制进 `assets/`；一
  个字体表数据文件逐份字体记录字符表、切分行高、基线与字距。

- OPEN_ISSUES: entry 6 is removed on completion; its pending
  decision (animation frame sources) resolves to using the frames
  already present in the existing textures, white flash for wounds,
  and the PD font texture for damage numbers.

  OPEN_ISSUES：条目 6 在完成后删除；其待裁决项（动画帧素材来
  源）裁决为——帧用现有贴图已有帧，受击用白闪，伤害数字用 PD
  字体贴图。

- No dependency, UI, or input changes.

  依赖、UI、输入均不变。
