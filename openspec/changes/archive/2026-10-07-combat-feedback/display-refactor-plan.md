# Display 侧 PD 对齐重构 · 方案与决策记录（落盘版）

> 用途：display 侧 PD 对齐重构的决策记录（**已实施**）。本文件是中间
> 阶段的存档：§1 后段（D9/D10/D11）、§4/§5 的终裁，以及 CR 后追加的
> 裁决（AnimStarted 删除、字体字段 tier→font、Walk 删除、能力重排等）
> 为准绳；前段与 §2 树/§6④ 有被后续裁决覆盖处，已在 §2 勘误与 §6 标注。
> 前置状态（本 change 内已实施）：combat-feedback 主体（B′ 死亡方案）、
> 术语统一、各轮语义复查——见 §8。

---

## 0. 背景与动机

用户对 frontend 的 CR 中提出：display 侧整体采用 PD（Pixel Dungeon v1.9.1，
源码在 `.ref/pixel-dungeon`（游戏层）与 `.ref/PD-classes`（noosa 引擎层，
反编译））的分组与命名，**除我们的机制（ECS、协议旗、数据驱动）外不对齐
者须给出强烈依据**。起因是 `BurstSpec`——PD 的对应物叫 `Splash`，属无理由
标新立异，用户点名纠正。

PD 分层事实（锚点见 §7）：
```
noosa（引擎机制）: tweeners(Tweener/Alpha/Pos/Scale/Delayer/CameraScroll)
                   particles(Emitter/PixelParticle/Factory)
                   BitmapText(Font/splitBy/measure) MovieClip(Animation)
                   Camera(shake) TextureFilm Visual/Image
pixeldungeon:      sprites(CharSprite/MobSprite: idle/run/attack/die/flash/
                   showStatus/bloodBurstA/move; 字段 isMoving/flashTime)
                   effects(FloatingText/Splash/Wound/Speck…)
                   effects.particles(BloodParticle…)
                   ui(未涉及)
```

---

## 1. 已裁决决策（含依据）

**D1 tween 自建，不引入 bevy_tweening**
- 事实：bevy 0.19 无内置 tween；生态 bevy_tweening 0.16 兼容 bevy ^0.19
  （2026-06 活跃）；bevy_hanabi（GPU 粒子）兼容但量级差三个数量级。
- bevy 自带且已在用：TextureAtlasLayout（= PD TextureFilm 的引擎等价）、
  Vec2::lerp/easing 数学、Sprite tint/flip。
- 决策：tween 自建（~百行）；particles 维持自建。依据：需求窄（alpha+pos
  两种）、避免依赖跟随 bevy 升级（config：引擎升级须独立 change）。

**D2 tween 完全 PD 化：elapsed + interval（累积器），放弃时间锚**
- PD：`Tweener{target, interval, elapsed, listener}`，逐帧 `elapsed +=
  Game.elapsed`，`progress = elapsed/interval`，完成自删 + onComplete 回调。
- 原提案 start/duration（锚）被推翻：AnimState 用锚是为了循环动画的相位
  同步与群像去同步；tween 是一次性的，无此需求，累积器零代价（update
  本就逐帧写值）。
- ECS 翻译：完成 = 组件自删；完成通知 = `RemovedComponents<Tween>` 观察者
  （PD Listener.onComplete 的对应物）。
- 不建 Delayer/ScaleTweener/CameraScrollTweener：零用例（不为假想需求写
  实现；fall/掉落落地时再加）。

**D3 AlphaTween 时序修正（讨论中逼出的真 bug 修复）**
- 淡出必须等 Die 动画播完才起算（PD：MobSprite.onComplete(die) 里 new
  AlphaTweener(this, 0, FADE_TIME)）。累积器 AlphaTween 一插即走表，故
  "认领"与"起淡"是两个时刻，加"放行"共三个：
  ```
  死亡落帧──认领──>Die 播毕──起淡──>淡出完成──放行──>despawn_dead 收走
  （升 IsDisappearing）  （插 AlphaTween）        （降旗）
  ```
- 放行判定用时间算术：`AnimState.finish_at + FADE_TIME`（finish_at 过期
  后滞留过去时刻，单调可比）。**Disappearance 计时簿维持删除**——不需要簿。
- 生命周期命名待裁（§4）。

**D4 particles 内部命名（noosa.particles 对齐）**
- `Particle → PixelParticle`；字段 `velocity → speed`（PD PointF speed）、
  `life_total → lifespan`（PD）、`life_remaining → left`（PD）。
- `acceleration` 保留全拼（PD 是 `acc`；房规：标识符不省略缩写）。
- `BurstSpec → Factory`（回归！）：PD 三层 = Splash.at（effects 层配方+编排）
  → Emitter.burst(factory, quantity)（机制层）→ Factory.emit（逐粒子配方，
  SplashFactory 是 Splash 的内部类，持 color/dir/cone，emit 里掷 life/speed）。
  我此前的 BurstSpec 形状本就 PD 对，只是名字烂；曾误判删除，现回归命名
  Factory，burst 签名对齐 `burst(factory, count, …)`。
- `spawn_burst → burst`；`advance_particle → update_particles`。
- 血粒子配方（血色 0xFFBB0000、重力、锥角）住 effects/splash.rs——对齐
  SplashFactory 住在 Splash 内部 + CharSprite.blood()。

**D5 bitmap_font → bitmap_text；FontTier → Font；去 tier 概念**
- 域名对齐 PD 类名 BitmapText；Font 对齐 BitmapText.Font。
- bevy::prelude::Font 撞名不成立：glob 导入被本模块显式定义遮蔽（Rust 规
  则：显式项优先于 glob），早前"强烈依据"是误判，撤回。
- GlyphTable 并入 Font（PD 的 Font 本身就是字形表，TextureFilm 继承）：
  glyphs/space_advance 字段与 glyph() 方法归 Font，文件删除。
- `split_glyphs → split_by`（PD Font.splitBy）。
- `TIERS → FONT_IDS`（constants/font_ids.rs）。
- 保留（不对齐，依据）：FontRegistry/FONT_TABLE_PATH（数据文件机制）、
  spawn_text（实体生成 vs 场景对象）、advance()/layout()（ECS 逐字形
  sprite 需显式排版）、glyph(ch)（房规"以被取物命名"）。

**D6 AnimState 锁家族正名**
- `locked_until → finish_at: Option<f32>`（一次性动画播毕时刻；PD 对应
  finished: bool——命名事件本身而非后果"锁"）。
- 辅助 `one_shot_at(anim) → started_at(anim)`（= finish_at − 时长，animate
  算相对播放用）。
- 判定 `locked(now) → playing(now)`（now < finish_at；PD 对应
  `curAnim.looped || !finished` 的谓词化）。使用点：
  `if state.playing(elapsed) { continue; }`。
- 语义表（已向用户澄清）：looped=true 永远循环播、推导随时可切、永不加锁；
  looped=false 钳末帧、switch_one_shot 设 finish_at=now+时长、锁存续期推导
  让位（attack 到期切回 idle/run；die 因 Dead 标记推导仍要 Die、定格末帧
  forever）；Dead 凌驾锁。PD 的"锁"= finished 标志 + 游戏逻辑不在播完前
  调 idle() 的调用纪律；我们是自动推导无调用方，故纪律显式化为时间戳。

**D7 sprite_animation 吸收 CharSprite 精灵行为（域边界重划）**
- combat_feedback 更名 **effects**（对齐 PD effects 包），只留"生成独立
  演出实体"者：飘字（show_status + update_floating_text）、splash、shake
  触发。
- 精灵侧行为迁 **sprite_animation**（≈ PD CharSprite/MobSprite 行为总汇）：
  present_attack → attack（PD CharSprite.attack）、present_flash×2 →
  flash + update_flash（PD flash）、死亡认领/淡出 → dead_fade（§4）。
- 系统命名律：**PD 裸动词**（attack/flash/splash/shake/show_status +
  update_* 家族），弃 present_ 前缀（PD 无此前缀；CharSprite 全是裸动词）。
- 常量对齐改名：FLASH_SECS→**FLASH_INTERVAL**（PD 0.05f 同名）、
  FADE_SECS→**FADE_TIME**（PD MobSprite.FADE_TIME=3f）、
  COLOR_ABOVE_HALF→**WARNING**、COLOR_AT_OR_BELOW_HALF→**NEGATIVE**
  （PD CharSprite 同名常量 0xFF8800/0xFF0000）、
  TEXT_LIFESPAN_SECS→**LIFESPAN**（PD FloatingText.LIFESPAN=1f）。
- FloatingText 字段对齐：life_total→lifespan、life_remaining→left。
- AnimKind 删 **Walk**（PD 词汇无 walk，零构造点）。

**D9 域边界终裁（覆盖 D7 的部分划分）**
- sprite_animation = **仅帧动画**：sync_animation()（一切切换：idle/run 推导
  +朝向、AttackResolved→攻击一次性+转向、Dead→Die 一次性；一次性开始发
  finish_at 到点发 AnimFinished{entity,anim}——
  PD MovieClip.onComplete 的消息化；AnimState 增 finished:bool 对齐 PD 字段）
  + animate()（纯播放）。present_attack 独立系统**并入** sync。
- effects = **一切特效**（flash、fade 都归此）：调用关系 effects → tween
  （AlphaTween/TweenFinished）、→ particles（burst）、→ camera（ShakeCamera）。
- 新消息协议：AnimFinished（sprite_animation 发；AnimStarted 已裁决删除）、
  TweenFinished{entity}（tween 的 update_tweens 完成自删时发——PD
  Tweener.Listener.onComplete 消息化）。
- 死亡认领（Added<Dead>→升 IsDisappearing，非驾驶者）**暂时放 sync_animation
  内**（Dead 分支切 Die 时一并升旗），后面再拆——用户明示。
- flash 属 effects（色彩特效，非帧动画）。

**D10 dead_fade 命名终裁**
- `fade_dead → start_dead_fade()`；`release_dead → release_dead_fade()`。
- 生命周期经消息解耦：start 读 AnimFinished{Die}∧持旗→插 AlphaTween；
  release 读 TweenFinished∧持旗→降旗。effects 不读 AnimState（彻底解耦）。

**D11 杂项终裁**
- Font 与 Appearance 的 `image` 字段 → `texture`。
- display/creature/mod.rs 注释写明：待合并入 display/figure/ 域。
- spec capability combat-feedback → effects 更名（连带 delta 目录/产物措辞）。

**D8 motion 重建为 PosTween 应用**
- tween 域补 `PosTween{from: Vec2, to: Vec2, elapsed, interval}`（PD
  PosTweener）；update_tweens 写 CurrPosition（snap 管线不动）。
- motion/systems/move.rs `move()`（PD CharSprite.move(from,to)）：
  Changed<CellCoord> → 插 PosTween{from:当前画面位, to:新格世界位,
  interval: MOVE_INTERVAL}。数学等价已验证：单格步进下等轴速 approach
  与 lerp 逐帧同值（行为不变）。
- motion/systems/is_moving.rs `update_is_moving()`：PD isMoving 字段升
  降的系统化；保留"剩余 ≤ 一帧步长提前落旗"约定。
- `CELL_SECS → MOVE_INTERVAL`（PD CharSprite.MOVE_INTERVAL=0.1f 同值同
  名）。
- utils/position.rs：approach/has_reached/is_moving 删（tween 取代），
  flip_x 保留（PD turnTo 判定）。
- camera/systems/advance_shake → **update_shake**。

---

## 2. 目标架构树（对齐版·成员级）

> 勘误（终版为准）：sprite_animation 只剩 `sync_animation` + `animate`；
> `flash` / `update_flash` 与 `dead_fade`（现 `start_dead_fade` /
> `release_dead_fade`）住 effects（D9/D10），`FLASH_INTERVAL` /
> `FADE_TIME` 亦随之住 effects/constants；motion 触发系统实际名
> `move_cells`（文件 move_cells.rs）；`Factory` 含 count/position/z，
> `burst(commands, factory, rng)`；`font_entry` 字段名 `font`（非 tier）。

```
frontend/display/
├─ tween/                                  // PD noosa.tweeners（新建；自建依据 D1）
│  ├─ components/
│  │  ├─ alpha_tween.rs  AlphaTween{from:f32, to:f32, elapsed:f32, interval:f32}
│  │  │                    // PD AlphaTweener；from/to=子类插值端点
│  │  └─ pos_tween.rs    PosTween{from:Vec2, to:Vec2, elapsed, interval}
│  │                       // PD PosTweener
│  └─ systems/update_tweens.rs update_tweens()
│                           // PD Tweener.update+updateValues(progress)
│                           //   AlphaTween→写 Sprite alpha；PosTween→写 CurrPosition
│                           //   完成：组件自删（PD kill）；观察者=RemovedComponents
│
├─ particles/                              // PD noosa.particles
│  ├─ components/pixel_particle.rs PixelParticle{speed:Vec2, acceleration:Vec2,
│  │                                   lifespan:f32, left:f32}
│  ├─ types/factory.rs Factory{color, size, dir, cone, speed:Range, life:Range,
│  │                       gravity}    // PD Emitter.Factory+SplashFactory 合形
│  ├─ entities/burst.rs burst(commands, factory, count, position, z, rng)
│  │                                  // PD Emitter.burst(factory, quantity)
│  └─ systems/update_particles.rs      // PD PixelParticle.update（含 Shrinking 缩放）
│
├─ bitmap_text/                            // ← bitmap_font 更名；PD noosa.BitmapText
│  ├─ types/font.rs Font{image:Handle<Image>, glyphs:HashMap<char,Rect>,
│  │     space_advance:f32, line_height:f32, baseline:f32, tracking:f32}
│  │     + glyph(ch)/advance()/measure()/layout()
│  ├─ types/font_entry.rs（serde）        // 字段 tier/texture/row_height/baseline/
│  │                                      // tracking/chars ✓ 语义对齐 splitBy 参数
│  ├─ utils/split.rs     split_by()       // PD Font.splitBy
│  ├─ utils/choose_font.rs choose_font()  // PD PixelScene.chooseFont ✓
│  ├─ resources/font_registry.rs          // 数据文件机制（不对齐，依据 §3）
│  ├─ entities/spawn_text.rs              // 实体生成机制（不对齐，依据 §3）
│  └─ constants/font_ids.rs FONT_IDS      // 数据 id 校验表
│
├─ sprite_animation/                       // PD MovieClip + CharSprite/MobSprite 总汇
│  ├─ constants/anim_kind.rs AnimKind{Idle,Run,Attack,Die}（删 Walk）
│  ├─ constants/ FLASH_INTERVAL / FADE_TIME
│  ├─ types/anim.rs Anim{frames, fps, looped}  // looped✓PD；fps=构造参单位
│  ├─ components/anim_state.rs AnimState{anim, frame_offset, finish_at}
│  │                       + started_at(anim) + playing(now)
│  ├─ components/flash.rs Flash{until}
│  └─ systems/ sync_animation / animate /
│             attack / flash+update_flash /
│             dead_fade.rs（§4 命名待裁）
│
├─ effects/                                // ← combat_feedback 更名瘦身；PD effects
│  ├─ components/floating_text.rs FloatingText{cell, lifespan, left}
│  ├─ resources/text_stacks.rs TextStacks
│  ├─ constants/ WARNING / NEGATIVE / LIFESPAN / 目标字号具名常量
│  └─ systems/
│     ├─ show_status.rs show_status()+update_floating_text()
│     ├─ splash.rs splash()               // 组 Factory → particles::burst
│     └─ shake.rs shake()
│
├─ motion/                                 // PD CharSprite.move 语义层（D8 重建）
│  ├─ components/curr_position.rs CurrPosition（PosTween 逐帧写入）
│  ├─ constants/move_interval.rs MOVE_INTERVAL
│  ├─ systems/move.rs move()              // Changed<CellCoord>→插 PosTween
│  ├─ systems/is_moving.rs update_is_moving()
│  └─ utils/position.rs flip_x()
│
├─ camera/                                 // PD noosa.Camera
│  ├─ resources/camera_shake.rs CameraShake{magnitude,duration,time_remaining,offset}
│  ├─ messages/shake_camera.rs ShakeCamera{magnitude,duration}
│  └─ systems/ update_shake / snap_camera / snap_sprites
│
├─ figure/ creature/ map/ tileset/ terrain_animation/   // 数据驱动（§3）
├─ constants/layout.rs                     // TILE_SIZE≈PD DungeonTilemap.SIZE
└─ display_phase.rs
```

---

## 3. 不对齐项清单（仅列不齐，均带依据）

```
【无对应】PD 无此物（我们的机制）  【机制】同物异形（ECS/数据驱动）
【房规】本工程纪律压过 PD

bitmap_text/
  Font.image: Handle<Image>            【待定】PD Font.texture；可改 texture，待裁
  Font.glyph(ch)                       【房规】PD Font.get；"以被取物命名"
  Font.advance()/layout()              【无对应】PD 内联顶点构建；ECS 需显式排版
  FontRegistry + FONT_TABLE_PATH       【无对应】PD 静态字段；数据文件机制
  spawn_text                           【机制】场景对象→实体生成
  FONT_IDS                             【无对应】PD font1x..font3x 变量名
particles/
  PixelParticle.acceleration           【房规】PD acc；不省略缩写
sprite_animation/
  Anim.fps                             【房规】PD 存 delay=1/fps；按作者单位
  AnimState.frame_offset              【机制】PD frameTimer 余数；时间锚（循环相位同步需要）
  AnimState.finish_at/started_at/playing 【机制】PD finished:bool+调用纪律；自动推导需时间戳
  Flash{until}                         【机制】PD flashTime 倒计时；时间锚
  sync_animation                       【机制】PD 无系统（调用纪律）；自动推导
effects/
  TextStacks                           【机制】PD stacks 静态字段；资源化
  目标字号具名常量                     【房规】PD 字面量 chooseFont(9, zoom)
motion/
  CurrPosition                         【无对应】PD Visual.x/y 泛用；画面位 vs 逻辑格
  move() 触发=Changed<CellCoord>       【机制】PD 由逻辑调用；反应式
  update_is_moving()                   【机制】PD 字段手动升降；系统化+提前落旗
camera/
  ShakeCamera 消息                     【机制】PD Camera.shake 方法；跨域请求消息化
  snap_camera/snap_sprites/screen_grid 【无对应】子像素晶格防渗色（PD 无此问题）
figure/ creature/ map/ tileset/ terrain_animation/ 【无对应】逐类硬编码→数据文件化
constants/layout.rs ZOOM/LAYER_*       【无对应】缩放/分层模型
display_phase.rs                       【无对应】系统调度分区
core/display/ IsDisappearing           【无对应】PD actor/sprite 分离；单实体协议旗
（IsMoving 已对齐 PD isMoving，不列）
```

---

## 4. dead_fade 生命周期命名（已裁：A）

> 终裁（D10）：`start_dead_fade` / `release_dead_fade`。以下三案为决议
> 过程存档。

```
A  start_dead_fade / release_dead_fade   （用户初案）
   评：对称 ✓；release_dead_fade 宾语歧义（释放的是尸体非淡出）；
      start_ 前缀与裸动词族不合拍
B  claim_dead / fade_dead / release_dead （三动词全对称）
   评：认领→起淡→放行，宾语始终是尸体；与 IsDisappearing=认领令牌、
      despawn_dead 收"无人认领或已放行"的协议语义严丝合缝
C  claim_dead + update_dead_fade         （B 的合并版，推荐）
   评：起淡/放行并为一次遍历两分支（无 AlphaTween 且过 Die 末→插；
      过 finish_at+FADE_TIME→降旗）；少一系统、无相序顾虑；
      update_ 对齐 PD 动词；文件 dead_fade.rs 两函数一主题
```

---

## 5. 全部待裁项（已清零）

1. §4 —— 已裁 A（D10）：`start_dead_fade` / `release_dead_fade`。
2. Font 字段名 —— 已裁 `texture`（D11）。
3. spec capability `combat-feedback → effects` 更名 —— 已更名；连带
   `bitmap-font → bitmap-text`；动画类需求另立 `sprite-animation`
   capability delta。

已裁决关闭的旧项：burst 参数形态（Factory 回归，D4）；系统命名律（裸动词，
D7）；粒子字段 left/lifespan/speed 照齐、acc 拒绝（D4）；GlyphTable 并入
（D5）；删 Walk（D7，已落地）；tween 自建（D1）；elapsed/interval（D2）；
字体数据字段 tier → font、残留 tiers.rs 删除（CR 后追加）。

---

## 6. 实施顺序与影响面（①–⑧ 已全部落地）

```
① tween 域新建（AlphaTween/PosTween/update_tweens）✓
② motion 重建（move_cells 插 tween；update_is_moving；MOVE_INTERVAL；
   删 approach 族）✓
③ dead_fade 生命周期按 D10 落地（start_dead_fade/release_dead_fade，
   消息驱动；Disappearance 删除）✓
④ combat_feedback → effects 更名瘦身 ✓——归属以 D9/D10 为准（本行原写
   "attack/flash/dead_fade 迁 sprite_animation" 已被覆盖）：effects 承接
   白闪、飘字、迸溅、震动、死亡淡出；sprite_animation 只留
   sync_animation + animate；attack 切换并入 sync
⑤ bitmap_font → bitmap_text + Font 合并 + split_by + FONT_IDS +
   字段 tier→font（CR 后追加）✓
⑥ particles 改名（PixelParticle/Factory/burst/update_particles）✓
⑦ camera update_shake；figure Appearance.texture ✓
⑧ 产物扫描（design、tasks、proposal、spec delta 措辞；capability 重排）
   + 门禁 ✓
行为不变量：单格步进数学等价（已验证）；淡出时序 Die 播毕→tween 淡出→
降旗→清扫；无头一帧离场；测试基线 325（CR 后清理旧测试所得）。
```

---

## 7. PD 锚点速查（file:line，.ref 相对路径）

```
PixelScene.chooseFont            pixel-dungeon/.../scenes/PixelScene.java:146-200
PixelScene 字体构建           同文件 :103-127（baseline 6/9/11/13/17 等）
Font.splitBy/colorMarked         PD-classes/.../noosa/BitmapText.java:266-336
Font.LATIN_FULL                  同上 :214-215
MovieClip{curAnim,curFrame,frameTimer,finished}  PD-classes/.../noosa/MovieClip.java:23-28
MovieClip.play 守卫(looped||!finished)           同上 :86-90
Animation{delay,looped,frames}   同上 :103-119
Tweener{target,interval,elapsed,listener}        PD-classes/.../noosa/tweeners/Tweener.java:23-61
AlphaTweener/PosTweener/ScaleTweener/Delayer     同目录
Emitter.burst(factory,quantity)  PD-classes/.../noosa/particles/Emitter.java:74
Emitter.Factory(emit 抽象)       同上 :143
PixelParticle{speed,acc,reset(x,y,color,size,lifespan)}  同目录
SplashFactory{color,dir,cone}    pixel-dungeon/.../effects/Splash.java:50-80
Splash.at(p,dir,cone,color,n)    同上 :48
FloatingText{LIFESPAN=1,DISTANCE=SIZE,stacks,timeLeft,key}
                                 pixel-dungeon/.../effects/FloatingText.java:30-45
CharSprite: FLASH_INTERVAL=.05   :54；MOVE_INTERVAL=.1 :53；
  POSITIVE/NEGATIVE/WARNING/NEUTRAL :47-51；isMoving :88；flashTime :82；
  idle/run/attack/operate/zap/jump/die/flash/burst/bloodBurstA/blood()/showStatus
  :132-247；attack=turnTo+play :160-163
MobSprite: FADE_TIME=3f :29；onComplete(die)→AlphaTweener→killAndErase :46-56
HeroSprite: attack 15fps 13,14,15,0；die 20fps 8..12,11  :66-72
RatSprite:  attack 15fps 2,3,4,5,0；die 10fps 11-14      :35-48
Camera.shake(mag,dur)/shakeMag/shakeTime/decay           PD-classes/.../noosa/Camera.java:159-166,228-231
Char.attack 里 shake 触发 gate(1,dmg/(HT/4),5)           Char.java:161 附近
```

---

## 8. 前置已实施状态（本 change 内，勿重做）

- 核心事实通道：AttackResolved/DamageApplied；apply_damage 身份无关
  （WoundableQuery 无 WorldDriver/Player/MonsterIndex；死者跳过；统一
  insert Dead）；CreatureDied 已删。
- despawn_dead（Derive，(Dead,¬IsDisappearing,¬WorldDriver)）；
  IsDisappearing 协议旗（core/display）；IsMoving 对齐 PD isMoving。
- advance NextTurnFilter=Or<(Without<Dead>,With<WorldDriver>)>；怪物策划
  无驾驶者依赖；玩家策划键 Player；act_move ActorFilter。CR 后裁：先启/
  开局门方案撤（PRE_START_TURN 删除）——世界自零点开张、一切首槽为 0；
  原 -1 与怪物 0 的永久 1 tick 相位差会让每一步都夹一个怪物回合（「一格
  一顿」），共享零点后同速回合同帧解析。
- 展示：死亡线已按 D10 落地（start_dead_fade/release_dead_fade，消息
  驱动）；旧 start/advance_death_fade 与 Disappearance 计时簿已删；
  Remains/快照登记簿/record_snapshot 已删；FADE_TIME 已成名。
- spec：world-clock/monster/health 三 delta，加 CR 后重排的
  effects/sprite-animation/bitmap-text/particles 四 delta + 主 specs 术语
  直改（strike→attack、ceiling→maximum、命中率、护甲等级等，术语表在
  combat delta）。
- 术语/黑名单/snake_case/跨行/注释零参考对象等五轮语义复查全净。
- 过滤器别名命名律：查询对象+Filter（ActorFilter/MonsterFilter/
  NextTurnFilter/DeadFilter/FreshDeadFilter）。
- 尸体格可走（target_cell execute MonsterFilter）。
- 门禁基线：325 测试 / clippy 0 / fmt / validate ✓（重构落地后）。
- 挂起：7.4 人工观感验收（用户跑游戏）；问题 3 已闭环（B′ 落地）。
