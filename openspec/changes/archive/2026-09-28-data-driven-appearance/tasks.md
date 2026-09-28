# Tasks: data-driven-appearance

## 1. 数据文件

- [x] 1.1 创建 `data/core/appearances.ron`：词表头部注释（编号纪律：句柄不稳定、存档走 id 映射）+ `[ "warrior" ]`；verify: 随 2.2 的加载测试覆盖
- [x] 1.2 创建 `data/graphic/looks.ron`：warrior 条目逐值复刻现常量表（texture warrior.png、帧 12×15、21 列 8 行、idle [0,0,0,1,0,0,1,1]@8fps、run [2,3,4,5,6,7]@20fps）；verify: 随 6.1 的规格对齐测试覆盖

## 2. core/appearance 词表注册表

- [x] 2.1 新增 `components/appearance_index.rs`：`AppearanceIndex(u16)` 直接 derive Component（句柄即组件），`from_index`/`index` 为 pub(crate)；doc 写明编号纪律（不稳定、非身份、存档走映射）；verify: `cargo check` 通过，类型带 doc comment
- [x] 2.2 新增 `resources/appearance_registry.rs`：`AppearanceRegistry`（`ids` + `by_id`，`get_index`/`get`/`iter`，`APPEARANCE_TABLE_PATH`）+ `parse_appearance_registry`（空 id、重复 id、非法 RON 校验，panic 指明文件）+ `FromWorld`；单测覆盖正常分配（同 id 同句柄、不同 id 不同句柄、文件顺序）与每类非法输入；verify: `cargo test` 绿
- [x] 2.3 `core/appearance/mod.rs` 新增 register（`init_resource` 一行 + resources 模块声明），`core/mod.rs` 增 `appearance::register(app)`；verify: `cargo check` 通过

## 3. sprite_animation 随迁

- [x] 3.1 `constants/anim_kind.rs`：`AnimKind` derive `Serialize`/`Deserialize` + `rename_all = "snake_case"`；verify: 内联 RON 片段可反序列化出 `idle`/`run`（单测或随 4.3 测试覆盖）
- [x] 3.2 `types/anim_clip.rs`：`frames` 改 `Vec<usize>`、去 `Copy` 留 `Clone`，doc 更新（去掉 `Rc` 预言，写明 Vec 的理由：serde 直出、无 Clone 消费方、Resource 需 Send+Sync）；`anim_state.rs` 测试从 const 构造改辅助函数；verify: `cargo test` 绿

## 4. display/appearance 画法注册表

- [x] 4.1 `types/appearance.rs` 更名 `types/look.rs`：`Appearance` → `Look`（`clip()` 的 Idle 回退语义与测试随迁）；verify: `cargo test` 绿
- [x] 4.2 新增 `types/look_entry.rs`（`LookEntry`：appearance/texture/frame_width/frame_height/columns/rows/clips）与 `types/anim_clip_entry.rs`（`AnimClipEntry`：anim/frames/fps）；verify: `cargo check` 通过，公开类型均带 doc comment
- [x] 4.3 `resources/appearance_registry.rs` 重写为 `resources/look_registry.rs`：`LookRegistry`（`HashMap<AppearanceIndex, Look>`，`look()` 必然成功访问器，`LOOKS_PATH`）+ `parse_look_entries(path, text, appearance_registry)`（D3 校验清单：未知 id、重复条目、缺失画法、空网格、空帧序列、非正 fps、帧号越界、重复动画名、缺 idle）+ `FromWorld`（`get_resource_or_init` 拉 core 注册表，装配贴图句柄与图集布局）；单测覆盖正常加载与每类非法输入；verify: `cargo test` 绿
- [x] 4.4 `attach_appearance`/`sync_animation`/`animate` 的查询与参数改 `AppearanceIndex` + `LookRegistry`（参数完整蛇形 `appearance_index`/`look_registry`），`sync_animation` 测试随迁（注册表键从枚举换句柄）；删除 `constants/` 目录；verify: `cargo test` 绿且 `grep -rn "AppearanceKind\|WARRIOR" src/` 无残留

## 5. hero 出生点

- [x] 5.1 `entities/hero.rs`：`HERO_APPEARANCE: &str = "warrior"` 常量；`spawn_hero` 带 `Res<AppearanceRegistry>`，`get_index` 换句柄（expect 注明 warrior 是词表声明的外观）；verify: `cargo test` 绿

## 6. 清理与验收

- [x] 6.1 规格对齐测试（落 `look_registry.rs` 测试模块）：读仓库真实 `data/core/appearances.ron` 与 `data/graphic/looks.ron`，断言 warrior 在词表中、画法条目帧尺寸 12×15、21×8 网格、idle/run 帧序列与帧率逐值一致；verify: `cargo test` 绿
- [x] 6.2 全绿验收：`cargo +nightly fmt --check`、`cargo clippy --all-targets`、`cargo check --all-targets`、`cargo test` 全部零警告通过
- [x] 6.3 运行验证：`cargo run` 后画面与操作手感同基线（主角 warrior 外观、待机呼吸、奔跑循环、朝向翻转、点击寻路）；verify: 人工确认无可见差异
