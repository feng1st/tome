# Tasks: combat-feedback | 任务：combat-feedback

## 1. Core Facts (combat / health domains) | 核心事实（combat / health 域）

- [x] 1.1 `src/core/health/messages/damage.rs`: `Damage` gains
  `source: Option<Entity>`; `act_attack` fills it with the attacker.
  verify: existing health and combat tests updated for the new field
  pass with `cargo test`.

  `src/core/health/messages/damage.rs`：`Damage` 新增
  `source: Option<Entity>`；`act_attack` 以攻击方填入。验证：既有
  health 与 combat 测试适配新字段后 `cargo test` 通过。

- [x] 1.2 `src/core/combat/messages/attack_resolved.rs` (new message
  type, doc comment stating the request-noun / fact-past-tense
  convention): `act_attack` emits `AttackResolved { attacker,
  attacker_cell, target_cell, damage_amounts }` per attack — hit
  or miss, the amounts in blow order, empty on a full miss. verify: unit tests — one
  landed blow of 4 plus one miss yields `damage_amounts` of `[4]`; a full miss
  yields an empty list and the consumed action; both pass with
  `cargo test`.

  `src/core/combat/messages/attack_resolved.rs`（新消息类型，doc
  comment 写明"请求用名词、事实用过去时"约定）：`act_attack` 每次
  攻击发出 `AttackResolved { attacker, attacker_cell, target_cell,
  damage_amounts }`——无论命中，命中伤害值按击序、全失为空。验
  证：单测——一击命中 4 一击未中得 `damage_amounts` 为 `[4]`；
  全失得空表且动作照耗；`cargo test` 通过。

- [x] 1.3 `src/core/health/messages/`: `DamageApplied { target,
  cell, source_cell, amount, max }` (past-tense fact); `apply_damage`
  emits one per applied request (source cell resolved fallibly, a
  gone source reads `None`), skips requests onto the dead, and marks
  every unmarked creature that crossed below zero with the `Dead`
  component — identity-free, no species branch. verify: unit tests —
  applied fact's fields; vanished source reads `None`; a request onto
  a dead body drops; below zero marks the body; the dual registration
  applies each request exactly once (fact included); all pass with
  `cargo test`.

  `src/core/health/messages/`：`DamageApplied { target, cell,
  source_cell, amount, max }`（过去时事实）；`apply_damage` 每条被
  施加的请求发一份（来源格可失败解析，来源已失读 `None`），跳过
  指名死者的请求，并为每只越零且未标记的生物挂上 `Dead` 组件——
  无身份、零分支。验证：单测——事实字段；来源消失读 `None`；指
  名死者的请求落空；越零标记躯体；双注册下每条请求仍恰好施加一
  次（事实同此）；`cargo test` 通过。

- [x] 1.4 `src/core/health/systems/despawn_dead.rs` (Derive phase):
  takes every `(With<Dead>, Without<IsDisappearing>,
  Without<WorldDriver>)` body — a same-frame death is never swept
  early (Derive runs ahead of the resolve phases), an unflagged
  corpse is always one whose presentation is over or never claimed,
  and the driver stays for the death wrap-up. verify: unit tests — an
  unclaimed body leaves; a disappearing body stays; the dead driver
  stays; the living are untouched; a same-frame death survives the
  sweep and leaves once its flag drops; `cargo test` passes.

  `src/core/health/systems/despawn_dead.rs`（Derive 阶段）：带走每
  个 `(With<Dead>, Without<IsDisappearing>, Without<WorldDriver>)`
  的躯体——同帧新死永不被抢扫（Derive 先于各 Resolve），无旗尸
  体必然呈现已毕或从未被认领，驾驶者留待死亡收尾。验证：单测——
  无人认领的躯体离开；消失中的躯体留下；死亡驾驶者留下；活体不
  受触碰；同帧新死在本扫存活、旗降后离开；`cargo test` 通过。

- [x] 1.5 `src/core/world_clock/`: the sweep counts turns under
  `Or<(Without<Dead>, With<WorldDriver>)>` — a dead turn pins
  nothing, the driver's unspent turn holds the game over;
  the world opens at zero with every creature's first slot there, so
  a due creature plans as it opens. The monster planner drops its
  driver query and start gate; the player planner keys on the
  `Player` marker. verify: unit tests — a dead monster's turn is
  swept past; a dead driver's turn holds; a due monster plans as the
  world opens; the ready driver's unspent turn holds the clock;
  `cargo test` passes.

  `src/core/world_clock/`：扫描以 `Or<(Without<Dead>,
  With<WorldDriver>)>` 计数回合——死亡回合不钉住任何东西，驾驶者
  未消耗的回合停住游戏结束；世界自零点开张、一切生物的首槽都在那
  里，到期者随即照常策划。怪物策划删去驾驶者查询与开局门；玩家策
  划改键于 `Player` 标记。验证：单测——死亡怪物的回合被扫过；死
  亡驾驶者的回合停住；到期怪物随世界开张照常策划；待命驾驶者的未
  消耗回合停住时钟；`cargo test` 通过。

## 2. Bitmap Text Domain | 位图字体域

- [x] 2.1 Copy the five font textures whole from
  `.ref/pixel-dungeon/assets/font{1x,15x,2x,25x,3x}.png` into
  `assets/`; create `data/graphic/fonts.ron` declaring the five fonts
  (texture, split row height — full height for 1x, 12/14/17/22 for
  the rest — baseline 6/9/11/13/17, tracking -1/-1/-1/-1/-2, and the
  reference's full Latin character table). verify: a data test loads
  the table and asserts the five fonts' values item for item;
  `cargo test` passes.

  将五份字体贴图从
  `.ref/pixel-dungeon/assets/font{1x,15x,2x,25x,3x}.png` 整份复制进
  `assets/`；新建 `data/graphic/fonts.ron` 声明五份字体（贴图、
  切分行高——1x 取全高，其余 12/14/17/22——基线 6/9/11/13/17、
  字距 -1/-1/-1/-1/-2、参考实现的完整拉丁字符表）。验证：数据测
  试加载字体表并逐项断言五份字体的数值；`cargo test` 通过。

- [x] 2.2 `src/frontend/display/bitmap_text/`: the font registry —
  load each font, split its texture within the row height at
  fully-transparent separator columns, pair stretches with the
  character table; startup fails when a texture cannot be read or a
  glyph count mismatches the table, the error naming file and font.
  verify: unit tests — the ten digit glyphs resolve to ten distinct
  rectangles in character-table order; a mismatch fails load;
  `cargo test` passes.

  `src/frontend/display/bitmap_text/`：字体注册表——逐份字体加
  载，在行高内按全透明分隔列切分贴图，区段按序配对字符表；贴图
  不可读或字形数与字符表不符时启动失败，错误指明文件与字体。验证：单
  测——十个数字字形按字符表顺序解析为十个互异矩形；不符即加载
  失败；`cargo test` 通过。

- [x] 2.3 Zoom-matched font choice, ported from the reference's
  `chooseFont` (thresholds on pt = target × zoom: 19/14/12/10 with
  the in-band exceptions; render scale = the band's integer ratio
  divided by the zoom). verify: unit test — target 9 at zoom 2 picks
  font 25x at scale one half; `cargo test` passes.

  按缩放选字体，移植参考实现的 `chooseFont`（阈值取 pt = 目标 ×
  缩放：19/14/12/10 及带内例外；渲染缩放 = 区间整数倍率除以缩
  放）。验证：单测——目标 9、缩放 2 时选中 25x 字体、缩放二分之
  一；`cargo test` 通过。

- [x] 2.4 Glyph-sprite text rendering: a spawned text lays one sprite
  per glyph side by side (advance = glyph width + tracking) on the
  font's baseline, under a root entity owning position; setting the
  text's transparency writes every glyph sprite's alpha. verify: unit
  tests — "10" lays two sprites advanced by width plus tracking; a
  half transparency reads one half on every glyph; `cargo test`
  passes.

  逐字形文字渲染：生成的文字每字形一个 sprite 并排摆放（步进 =
  字宽加字距），共用该字体基线，共属一个持有位置的根实体；设置文
  字透明度写入每个字形 sprite 的 alpha。验证：单测——"10" 摆两
  个 sprite、按字宽加字距步进；透明度设半后每个字形 alpha 均为
  二分之一；`cargo test` 通过。

## 3. Particles Domain | 粒子域

- [x] 3.1 `src/frontend/display/particles/`: the `PixelParticle`
  component (speed, acceleration, remaining life, and the lifetime
  born with) and the `Factory` value plus `burst` spawner — count,
  position, cone, color, size, speed range, lifetime range, gravity;
  a zero count spawns nothing.
  verify: unit tests — a five-particle burst with lifetimes 0.5..1.0
  and speeds 40..80 spawns five entities each in range; zero spawns
  none; `cargo test` passes.

  `src/frontend/display/particles/`：`PixelParticle` 组件（速度、
  加速度、剩余生命与出生时的寿命）与 `Factory` 值、`burst` 生成
  函数——数量、位置、锥角、颜色、尺寸、速度区间、生命区间、重
  力；零数量不生成。验证：单测——五
  粒迸溅（生命 0.5..1.0、速度 40..80）生成五个实体且各值在区间
  内；零粒不生成；`cargo test` 通过。

- [x] 3.2 Particle motion (`update_particles`): per frame accelerate
  speed by gravity, advance position by speed, shrink size linearly
  toward zero over the lifetime, despawn at life's end. verify: unit tests — an upward
  particle under positive gravity slows frame over frame; an expired
  particle's entity no longer exists; `cargo test` passes.

  粒子运动（`update_particles`）：每帧按重力加速速度、按速度推
  进位置、随生命线性缩向
  零、生命结束即 despawn。验证：单测——向上粒子在正重力下逐帧
  减速；到期粒子实体不复存在；`cargo test` 通过。

## 4. Animation Mechanism | 动画机制

- [x] 4.1 `data/graphic/figures.ron`: anim entries gain the looping
  flag; the warrior gains Attack (frames 13,14,15,0 at 15 fps,
  non-looping) and Die (8,9,10,11,12,11 at 20 fps, non-looping); the
  giant white rat gains Attack (18,19,20,21,16 at 15 fps,
  non-looping) and Die (27,28,29,30 at 10 fps, non-looping) on the
  white-variant row. The figure registry parses and validates the
  flag. verify: data tests assert both entries' sequences, rates, and
  looping flags value for value; `cargo test` passes.

  `data/graphic/figures.ron`：动画条目新增循环标志；战士新增
  Attack（帧 13,14,15,0、15 fps、不循环）与 Die（帧
  8,9,10,11,12,11、20 fps、不循环）；大白鼠在白色变体行新增
  Attack（帧 18,19,20,21,16、15 fps、不循环）与 Die（帧
  27,28,29,30、10 fps、不循环）。形象注册表解析并校验该标志。验
  证：数据测试逐项断言两条目的序列、帧率与循环标志；`cargo
  test` 通过。

- [x] 4.2 `src/frontend/display/sprite_animation/`: a non-looping
  frame table clamps playback at its last frame instead of wrapping;
  `AnimState` gains the one-shot's finish state (`finish_at` +
  `finished`) — while a one-shot plays, the idle/run derivation
  switches nothing; the death marker overrides everything, cutting
  short even an active one-shot. verify: unit tests — clamp at the
  last frame; the one-shot holds against derivation and resumes
  after; the death marker interrupts a mid-swing attack; `cargo test`
  passes.

  `src/frontend/display/sprite_animation/`：不循环帧表把播放钳在末
  帧不回卷；`AnimState` 新增一次性播毕状态（`finish_at` +
  `finished`）——一次性动画存续期间待机/奔跑推导不做任何切换；
  死亡标记凌驾一切，存续中的一次性也立即打断。验证：单测——定
  格末帧；一次性存续期推导让位、播毕恢复；死亡标记打断挥击中
  的攻击；`cargo test` 通过。

- [x] 4.3 `sync_animation` gains the attack branch: on an
  attack-resolved fact the attacker faces the target cell and
  switches to its attack animation, hit or miss; a gone attacker is
  skipped. A one-shot's end announces `AnimFinished{entity, anim}`;
  the former standalone presentation system is gone. verify: unit
  tests — a landed attack faces and plays; a full miss still swings;
  a gone attacker starts nothing; the end announces once; `cargo
  test` passes.

  `sync_animation` 新增攻击分支：攻击结果事实到达时攻击方朝目标
  格转向并切到攻击动画，无论命中；攻击方已离场则跳过。一次性
  播毕发 `AnimFinished{entity, anim}` 公告；原独立呈现系统不复
  存在。验证：单测——命中攻击转向并播放；全失照样挥击；离场攻
  击方不启动任何动画；播毕恰好公告一次；`cargo test` 通过。

## 5. Camera Shake | 相机震动

- [x] 5.1 `src/frontend/display/camera/`: the camera-owned
  `ShakeCamera { magnitude, duration }` message and shake state; each
  frame the offset draws uniform-random within ±magnitude per axis,
  linearly damped to zero over the duration, applied before the
  pixel-grid snap; after the duration the offset is exactly zero.
  verify: unit tests — a shake of magnitude 4 over 0.3s keeps offsets
  within 4 world pixels, decays, and ends exactly at zero; the
  snapped position stays on the grid throughout; `cargo test` passes.

  `src/frontend/display/camera/`：相机域持有的 `ShakeCamera
  { magnitude, duration }` 消息与震动状态；每帧各轴在 ±幅度内均匀
  随机取偏移、随时长线性衰减至零、在像素网格吸附前施加；时长结
  束后偏移恰为零。验证：单测——幅度 4、0.3 秒的震动偏移不超 4
  世界像素、逐渐收窄、结束恰为零；全程吸附位置落在网格上；
  `cargo test` 通过。

## 6. Combat-Feedback Domain | 战斗反馈域

- [x] 6.1 The death claim: every fresh death but the driver's is
  claimed — the body rises `IsDisappearing` (the core-side protocol
  flag) when its death presentation takes it, so the departure sweep
  holds off until the presentation releases it. The claim is raised
  inside `sync_animation`'s death branch for now; a later change may
  split it out. verify: unit tests — a fresh death is claimed; the
  dead driver is never claimed; unclaimed bodies leave on the sweep;
  `cargo test` passes.

  死亡认领：除驾驶者外的每个新死被认领——死亡呈现接管躯体时升
  `IsDisappearing`（核心侧协议旗），离场清扫因此在呈现释放前不
  收走躯体。认领暂在 `sync_animation` 的死亡分支内升起，后续变
  更可拆出。验证：单测——新死被认领；死亡驾驶者永不被认领；无
  人认领的躯体照常被清扫；`cargo test` 通过。

- [x] 6.2 Hit flash (`flash` / `update_flash`): a flash component
  with apply/expire systems —
  on a damage-applied fact whose target is still in the world, the
  target's sprite tint is multiply-boosted for 0.05 seconds and then
  restored; a removed target flashes nothing. verify: unit tests — a
  living target flashes and restores after 0.05s; a removed target
  flashes nothing; `cargo test` passes.

  受击白闪（`flash` / `update_flash`）：白闪组件与施加/到期系统——伤害落地事实的目标仍在世
  界中时，其精灵着色乘性拉白 0.05 秒后复原；已移除的目标不白
  闪。验证：单测——存活目标白闪并在 0.05 秒后复原；已移除目标
  不白闪；`cargo test` 通过。

- [x] 6.3 Floating damage text: on a damage-applied fact, spawn the
  amount as world-space text (the bitmap-text domain renders it) at
  the wounded sprite's top-center; it rises one tile over one second
  and fades in the latter half; the color is orange 0xFF8800 while
  the target's current hit points exceed half the maximum, red
  0xFF0000 at or below half or when the target is gone; same-cell
  texts stack, a newcomer pushing the cell's living texts up one
  line. verify: unit tests — spawn position, content, rise and fade;
  the orange/red threshold; the stacking push; `cargo test` passes.

  伤害数字飘字：伤害落地事实到达时，在受击精灵顶部中央生成数值
  的世界空间文字（由位图字体域渲染）；一秒内上浮一格，后半程淡
  出；目标当前生命值高于上限一半为橙 0xFF8800，一半及以下或目
  标已失为红 0xFF0000；同格飘字堆叠，新到者把该格存活飘字上挤
  一行。验证：单测——生成位置、内容、上浮与淡出；橙红分档；堆
  叠上挤；`cargo test` 通过。

- [x] 6.4 Blood splash: on a damage-applied fact, burst blood (the
  particles domain's `Factory` + `burst`) at the wounded sprite's
  center — count
  min(9·√(amount ÷ maximum), 9), color 0xFFBB0000, 4-world-pixel
  squares living 0.5..1.0s, speeds 40..80 within a 90° cone from the
  source cell toward the target cell, gravity 100; a source-less
  wound sprays an upward 180° fan. verify: unit tests — maximum-equal
  damage bursts nine; zero damage bursts none; a left-side source
  sprays rightward; `cargo test` passes.

  血粒子迸溅：伤害落地事实到达时，在受击精灵中心迸溅血粒子（粒
  子域的 `Factory` + `burst`）——数量 min(9·√(数额 ÷ 上限), 9)，颜色 0xFFBB0000，4
  世界像素方块存活 0.5..1.0 秒，速度 40..80，沿来源格指向目标格
  的 90° 锥，重力 100；无来源伤害以朝上 180° 扇形喷射。验证：
  单测——数额等于上限迸九粒；零伤害不迸；正左来源向右喷射；
  `cargo test` 通过。

- [x] 6.5 Death fade (`start_dead_fade` / `release_dead_fade`):
  the fade opens when the death animation's end is announced on a
  claimed body — an `AlphaTween` to transparency over `FADE_TIME`;
  when that tween completes it announces its end, and the fade
  releases the claim, handing the body back to `despawn_dead`. The
  former timing book is gone. verify: unit tests — the die end opens
  the fade on a claimed body; an unclaimed body never fades; a
  non-die end never fades; the fade end releases the claim;
  `cargo test` passes.

  死亡淡出（`start_dead_fade` / `release_dead_fade`）：死亡动画播
  毕的公告到达已认领的躯体时开启淡出——一支 `AlphaTween` 在
  `FADE_TIME` 内淡至透明；该 tween 播毕发公告，淡出据此释放认
  领，把躯体交还 `despawn_dead`。原计时簿不复存在。验证：单测—
  —Die 播毕在已认领躯体上开启淡出；无人认领的躯体永不淡出；非
  Die 播毕不淡出；淡出播毕释放认领；`cargo test` 通过。

- [x] 6.6 Shake trigger (`shake`): on a damage-applied fact whose target
  carries the world-driver marker and whose amount exceeds a quarter
  of the maximum, write a `ShakeCamera` request — magnitude the
  amount divided by a quarter of the maximum, gated to 1..5, duration
  0.3s; lighter wounds request nothing. verify: unit tests — 10 damage
  at maximum 20 requests magnitude 2 for 0.3s; 5 at 20 requests
  nothing; `cargo test` passes.

  震动触发（`shake`）：伤害落地事实的目标带驾驶者标记且数额超过上限四分之
  一时，写出 `ShakeCamera` 请求——幅度为数额除以上限四分之
  一、钳在 1..5，时长 0.3 秒；更轻的伤害不发请求。验证：单测—
  —上限 20 受 10 点请求幅度 2、0.3 秒；受 5 点不发请求；`cargo
  test` 通过。

## 7. Wiring and Wrap-Up | 接线与收尾

- [x] 7.1 Register the four new domains (bitmap text, particles,
  tween, effects) and rebuild the orchestration: presentations into
  `DisplayPhase::Sync` (flash, floating text, splash, shake, the
  dead-fade start and release), time advancers into
  `DisplayPhase::Animate` (flash expiry, text float, tween updates,
  particle updates), and movement (`move_cells`, `update_is_moving`)
  into `DisplayPhase::Motion`. verify: `cargo check --all-targets`
  passes and the game boots to the map without panic.

  注册四个新域（位图字体、粒子、tween、effects）并重建编排：呈现
  入 `DisplayPhase::Sync`（白闪、飘字、迸溅、震动、死亡淡出开启
  与释放），时间推进者入 `DisplayPhase::Animate`（白闪到期、飘
  字上浮、tween 推进、粒子推进），移动（`move_cells`、
  `update_is_moving`）入 `DisplayPhase::Motion`。验证：`cargo
  check --all-targets` 通过，游戏启动进图无 panic。

- [x] 7.2 Update `openspec/OPEN_ISSUES.txt`: remove entry 6; register
  the flash stand-in (multiply overexposure standing in for the
  additive silhouette; recover when a custom material pipeline
  lands). verify: the file shows no entry 6 and carries the flash
  recovery note.

  更新 `openspec/OPEN_ISSUES.txt`：删除条目 6；登记白闪顶替（乘
  色过曝顶替加色剪影；待材质管线落地时回收）。验证：文件中条目
  6 不复存在，且载有白闪回收条目。

- [x] 7.3 Full green gate: `cargo +nightly fmt --check`, `cargo
  clippy --all-targets`, `cargo check --all-targets`, `cargo test` —
  all pass with zero warnings. verify: the four commands exit clean.

  全绿门禁：`cargo +nightly fmt --check`、`cargo clippy
  --all-targets`、`cargo check --all-targets`、`cargo test`——全
  部零警告通过。验证：四条命令干净退出。

- [ ] 7.4 Manual visual pass: run the game, attack the rat — attack
  swing, white flash, floating number, blood splash, death animation
  and fade all show; take a heavy bite — the camera shakes. verify:
  each behavior observed on screen.

  人工观感验收：运行游戏攻击老鼠——挥击、白闪、飘字、血粒子、
  死亡动画与淡出全部可见；被咬一口重的——相机震动。验证：各表
  现在屏幕上实际看到。

## 8. Display Alignment Refactor (post-CR) | display 对齐重构（CR 后）

Decision record: `display-refactor-plan.md`.

决策记录：`display-refactor-plan.md`。

- [x] 8.1 `src/frontend/display/tween/`: the tween domain —
  `AlphaTween` and `PosTween` with `elapsed + interval`
  accumulators; `update_tweens` writes the sprite alpha or the
  presentation position, self-removes at the end, and announces
  `TweenFinished`. verify: unit tests; `cargo test` passes.

  `src/frontend/display/tween/`：tween 域——`AlphaTween` 与
  `PosTween` 以 `elapsed + interval` 累积推进；`update_tweens`
  写入精灵透明度或呈现位置、到点自删并发 `TweenFinished` 公告。
  验证：单测；`cargo test` 通过。

- [x] 8.2 `src/frontend/display/motion/`: rebuilt on `PosTween` —
  `move_cells` inserts one on each cell change, `update_is_moving`
  owns the flag, `MOVE_INTERVAL` replaces the old cell time; the
  approach helpers are gone. verify: unit tests; `cargo test`
  passes.

  `src/frontend/display/motion/`：以 `PosTween` 重建——`move_cells`
  于每次格变化插入一支，`update_is_moving` 管旗，`MOVE_INTERVAL`
  取代旧格时间；approach 辅助族删除。验证：单测；`cargo test` 通
  过。

- [x] 8.3 Dead fade through messages: `start_dead_fade` reads
  `AnimFinished{Die}` on a claimed body and inserts `AlphaTween`;
  `release_dead_fade` reads `TweenFinished` and drops the flag; the
  `Disappearance` timing book is deleted. verify: unit tests;
  `cargo test` passes.

  死亡淡出走消息：`start_dead_fade` 在已认领躯体上读
  `AnimFinished{Die}` 插 `AlphaTween`；`release_dead_fade` 读
  `TweenFinished` 降旗；`Disappearance` 计时簿删除。验证：单测；
  `cargo test` 通过。

- [x] 8.4 The effects domain (renamed from combat feedback): the
  flash, floating-text, splash, shake, and dead-fade systems
  reorganized under the effects/tween/particles/camera boundary;
  constants `FLASH_INTERVAL`, `FLASH_TINT`, `FADE_TIME`, `LIFESPAN`,
  `WARNING`, `NEGATIVE`. verify: unit tests; `cargo test` passes.

  effects 域（由 combat feedback 更名）：白闪、飘字、迸溅、震动、
  死亡淡出各系统按 effects/tween/particles/camera 边界重组；常量
  `FLASH_INTERVAL`、`FLASH_TINT`、`FADE_TIME`、`LIFESPAN`、
  `WARNING`、`NEGATIVE`。验证：单测；`cargo test` 通过。

- [x] 8.5 `bitmap_text` rename and cleanup: `Font` merges the glyph
  table, `split_by`, `FONT_IDS`, the data field `font` (the stray
  `tiers.rs` deleted), `Font.texture` / `Appearance.texture`.
  verify: unit tests; `cargo test` passes.

  `bitmap_text` 更名清理：`Font` 合并字形表、`split_by`、
  `FONT_IDS`、数据字段 `font`（残留的 `tiers.rs` 删除）、
  `Font.texture` / `Appearance.texture`。验证：单测；`cargo test`
  通过。

- [x] 8.6 `particles` renames: `PixelParticle`
  (`speed`/`acceleration`/`lifespan`/`left`), `Factory`, `burst`,
  `update_particles`. verify: unit tests; `cargo test` passes.

  `particles` 更名：`PixelParticle`
  （`speed`/`acceleration`/`lifespan`/`left`）、`Factory`、`burst`、
  `update_particles`。验证：单测；`cargo test` 通过。

- [x] 8.7 Camera and animation cleanup: `update_shake`;
  `sync_animation` owns every switch; `AnimStarted` deleted;
  `AnimKind::Walk` deleted. verify: unit tests; `cargo test` passes.

  相机与动画清理：`update_shake`；`sync_animation` 总揽一切切换；
  `AnimStarted` 删除；`AnimKind::Walk` 删除。验证：单测；
  `cargo test` 通过。

- [x] 8.8 Artifact sweep: the spec deltas (effects /
  sprite-animation / bitmap-text / particles / player-animation) and
  design / proposal / tasks / plan / OPEN_ISSUES aligned with the
  final code; gates green. verify: `openspec validate combat-feedback`
  and the four cargo gates pass.

  产物扫描：spec 增量（effects / sprite-animation / bitmap-text /
  particles / player-animation）与 design / proposal / tasks / plan
  / OPEN_ISSUES 对齐最终代码；门禁全绿。验证：`openspec validate
  combat-feedback` 与四条 cargo 门禁通过。
