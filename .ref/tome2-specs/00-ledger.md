# tome2 分析账本

已完成分析的内容登记于此，由 tools/tome2spec.py report 生成。
再次进入分析前先查本表（或用 show 按路径查询），命中即跳过，不重读。

| 完成日期 | 源文件 | 产物 spec | 备注 |
|---|---|---|---|
| 2026-10-05 | lib/core/auto.lua | specs/automatizer/spec.md | 自动拾取器全量：动作件/object_status/规则树编译/规则集管线/auto_aux 界面/类型目录/easy_add_rule；Lua 4 方言现状记录 |
| 2026-10-05 | lib/core/building.lua | specs/lua-core/spec.md | 建筑动作注册表与 HOOK_BUILDING_ACTION 挂钩 |
| 2026-10-05 | lib/core/crpt_aux.lua | specs/corruption/spec.md | 腐化核心全量：访问器/依赖互斥判定/得失级联/注册与钩包装/剧透生成 |
| 2026-10-05 | lib/core/dungeon.lua | specs/lua-core/spec.md | 地城与方向辅助/#!map 检测/load_map 包装/place_trap 层暂存/level_generator 注册 |
| 2026-10-05 | lib/core/gen_idx.lua | specs/lua-core/spec.md | 帮助索引生成器全量；126 文档名清单为数据不逐条分析（帮助文本排除范围） |
| 2026-10-05 | lib/core/gods.lua | specs/lua-core/spec.md | add_god 注册：钩直挂全局（无判定包装）与 data 入存档 |
| 2026-10-05 | lib/core/help.lua | specs/lua-core/spec.md | 情境帮助全量：自维护钩表/hook 与 callback 双道/激活位入存档/出生重置/doc 件 |
| 2026-10-05 | lib/core/init.lua | specs/lua-core/spec.md | Lua 启动链装载序与 patch 系统（set_safe_globals 包夹与 patch_init 校验） |
| 2026-10-05 | lib/core/load.lua | specs/lua-core/spec.md | add_loadsave 注册表（表默认递归点路径） |
| 2026-10-05 | lib/core/load2.lua | specs/lua-core/spec.md | 存取双钩与点路径重建与默认补全块 |
| 2026-10-05 | lib/core/mimc_aux.lua | specs/lua-core/spec.md | 拟态形注册全量与 Abomination 兜底形 |
| 2026-10-05 | lib/core/monsters.lua | specs/lua-core/spec.md | summon_monster 分派封装 |
| 2026-10-05 | lib/core/objects.lua | specs/lua-core/spec.md | 造物/set_item_tester 三分/create_artifact/get_kind/get_item |
| 2026-10-05 | lib/core/player.lua | specs/lua-core/spec.md | 玩家访问器/mana/能力与 mkey 注册/亚种与躯干辅助 |
| 2026-10-05 | lib/core/powers.lua | specs/lua-core/spec.md | magic powers 注册与 execute_magic 流程与 get_level_power |
| 2026-10-05 | lib/core/quests.lua | specs/lua-core/spec.md | add_quest 注册：动态描述/hook 直挂/data 入存档 |
| 2026-10-05 | lib/core/s_aux.lua | specs/school-magic/spec.md | 学派法术核心全量：注册/等级三道折算/耗能与显示/施放管线/辅助/杖与激活/新 GF 注册 |
| 2026-10-05 | lib/core/stores.lua | specs/lua-core/spec.md | store_buy_list 收购清单钩 |
| 2026-10-05 | lib/core/util.lua | specs/lua-core/spec.md | 通用辅助全量：安全全局/补丁注册/add_hooks/msg 包装/罗盘距离/定时器/生成开关（set_object 与 set_artifact 误写 m_allow_special 现状缺陷）/栈/game 位 |
| 2026-10-05 | lib/core/xml.lua | specs/lua-core/spec.md | XML 模块全量：collect 解析/write 双道/english_xml 自然语渲染/print_xml |
| 2026-10-05 | lib/dngn/dun1.14 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun10.0 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun11.20 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun11.22 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun17.15 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun18.0 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun18.1 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun19.11 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun2.31 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun22.10 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun22.5 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun24.0 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun29.15 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun3.18 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun3.28 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun3.3 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun5.0 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun5.14 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/dngn/dun6.0 | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/a_info.txt | specs/artifact/spec.md, specs/edit-format/spec.md | 法宝词表格式与装载语义已提取；条目为内容数据不逐条分析；序号区间生成期语义随 object2.c 追加 |
| 2026-10-05 | lib/edit/ab_info.txt | specs/ability/spec.md, specs/edit-format/spec.md | 能力词表格式已提取；条目为内容数据不逐条分析；习得流程随 skills.c 追加 |
| 2026-10-05 | lib/edit/al_info.txt | specs/alchemy/spec.md, specs/edit-format/spec.md | 炼金配方与法宝可选旗标表两套数据的格式已提取；条目为内容数据不逐条分析；炼金界面与萃取随引擎分析追加 |
| 2026-10-05 | lib/edit/ba_info.txt | specs/building-action/spec.md, specs/edit-format/spec.md | 建筑动作词表格式已提取；条目为内容数据不逐条分析；动作执行与建筑编排随 bldg.c 追加 |
| 2026-10-05 | lib/edit/between.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/d_info.txt | specs/dungeon/spec.md, specs/edit-format/spec.md | 地城词表格式与装载语义已提取；条目为内容数据不逐条分析；生成期行为随 generate.c/dungeon.c 追加 |
| 2026-10-05 | lib/edit/dragons.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/e_info.txt | specs/ego-item/spec.md, specs/edit-format/spec.md | 称号词表格式已提取；条目为内容数据不逐条分析；生成挑选随 object2.c 追加；头注 T: 至多五行与 D: 可用两说与代码不符，以代码为准 |
| 2026-10-05 | lib/edit/evil.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/f_info.txt | specs/terrain/spec.md, specs/edit-format/spec.md | 地形词表格式与装载语义已提取；条目为内容数据不逐条分析；序号运行期语义随 cave.c/generate.c 追加 |
| 2026-10-05 | lib/edit/haunted.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/k_info.txt | specs/object/spec.md, specs/edit-format/spec.md | 物品词表格式已提取；6418 行条目为内容数据不逐条分析；生成/鉴定随 object1/object2.c 追加 |
| 2026-10-05 | lib/edit/maeglin.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/misc.txt | specs/capacities/spec.md | 容量声明文件；M: 指令路由与装载顺序已提取；通用指令解析器其余指令（地图类）随 init1.c 全量分析归入关卡布局格式 |
| 2026-10-05 | lib/edit/nirnaeth.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/numenor.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/ow_info.txt | specs/store-owner/spec.md, specs/edit-format/spec.md | 店主词表格式已提取；条目为内容数据不逐条分析；定价与声望机制随 store.c 追加 |
| 2026-10-05 | lib/edit/p_info.txt | specs/player/spec.md, specs/edit-format/spec.md | 种族/亚种/职业/元职业四类记录与通用技能、出身史已提取；1974 行条目为内容数据不逐条分析；出生消费随 birth.c 追加 |
| 2026-10-05 | lib/edit/qrand1.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/qrand10.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/qrand11.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/qrand12.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/qrand14.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/qrand5.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/qrand6.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/qrand7.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/r_info.txt | specs/monster/spec.md | 词表格式与装载语义已提取；各词条为内容数据，不逐条分析；旗标运行期效果随 monster1/2/3.c、melee1/2.c 分析追加 |
| 2026-10-05 | lib/edit/ra_info.txt | specs/randart-part/spec.md, specs/edit-format/spec.md | 随机神器部件词表格式已提取；条目为内容数据不逐条分析；生成组合随 randart.c/wizard1.c 追加 |
| 2026-10-05 | lib/edit/re_info.txt | specs/monster-ego/spec.md, specs/edit-format/spec.md | 怪物附属称号词表格式已提取；条目为内容数据不逐条分析；套用运算随 monster2.c/melee2.c 追加；头注 E: 行无解析分支已记录 |
| 2026-10-05 | lib/edit/s_crypt.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/s_death.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/s_doom.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/s_factory.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/s_gates.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/s_info.txt | specs/skill/spec.md, specs/edit-format/spec.md | 技能词表格式已提取；条目为内容数据不逐条分析；成长与消耗随 skills.c 追加；头注 A: 友好行实为 f: 已记录 |
| 2026-10-05 | lib/edit/s_name.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/s_orc.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/s_ship.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/set_info.txt | specs/item-set/spec.md, specs/edit-format/spec.md | 物品集合词表格式已提取；条目为内容数据不逐条分析；激活判定随 object2.c 追加 |
| 2026-10-05 | lib/edit/special.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/spiders.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/st_info.txt | specs/store/spec.md, specs/edit-format/spec.md | 商店词表格式已提取；条目为内容数据不逐条分析；补货与定价随 store.c 追加 |
| 2026-10-05 | lib/edit/t_basic.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/t_bree.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/t_d_bree.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/t_d_gond.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/t_d_khaz.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/t_d_lori.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/t_d_mina.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/t_gondol.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/t_info.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/t_khazad.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/t_lorien.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/t_minas.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/t_pref.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/thieves.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/thrain.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/tr_info.txt | specs/trap/spec.md, specs/edit-format/spec.md | 陷阱词表格式已提取；条目为内容数据不逐条分析；布设与触发随 traps.c 追加；头注短形态 I: 行与代码不符已记录 |
| 2026-10-05 | lib/edit/trolls.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/v_info.txt | specs/vault/spec.md, specs/edit-format/spec.md | 宝库词表格式已提取；布局行为内容数据不逐条分析；字形翻译随 generate.c 与地图格式追加 |
| 2026-10-05 | lib/edit/volcano.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/w_info.txt | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/wf_info.txt | specs/wilderness-terrain/spec.md, specs/edit-format/spec.md | 野外地形词表格式已提取；条目为内容数据不逐条分析；野外铺设随 wild.c 追加 |
| 2026-10-05 | lib/edit/wights.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/edit/wolves.map | specs/content-maps/spec.md | 内容地图/城镇/dngn 实例登记（要点见 spec，格网不誊抄；格式随 map-format 与 dungeon-level） |
| 2026-10-05 | lib/file/book-0.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-1.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-10.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-101.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-102.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-103.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-104.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-105.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-106.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-107.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-11.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-12.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-13.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-14.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-15.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-16.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-17.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-18.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-19.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-2.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-20.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-200.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-201.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-202.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-203.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-4.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-6.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-7.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-8.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/book-9.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/bravado.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/chainswd.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/dam_huge.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/dam_lots.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/dam_med.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/dam_none.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/dam_xxx.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/dead.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/death.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/elvish.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/error.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/mondeath.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/monfear.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/monspeak.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/news.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/news2.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/rart_f.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/rart_s.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/rumors.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/sfail.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/silly.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/smeagol.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/smeagolr.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/speakpet.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/timefun.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/file/timenorm.txt | specs/flavor-tables/spec.md | 随机文案表/画件/命名表/书文本实例登记（消费者调用点见 spec） |
| 2026-10-05 | lib/mods/mods_aux.lua | specs/modules/spec.md | 模块系统全量：注册与缺省值/五查询件/存档兼容/布局重定向/额外扫描；沙箱块注释停用现状记录 |
| 2026-10-05 | lib/mods/modules.lua | specs/modules/spec.md | 装载入口 |
| 2026-10-05 | lib/module.lua | specs/modules/spec.md | ToME 2.3.5 模块声明（条目原缺 specs/lines 键，已补） |
| 2026-10-05 | lib/scpt/bounty.lua | specs/scpt-misc/spec.md | 赏金任务全量：注册/接取 54 号动作/交尸 55 号动作与双技能教授 |
| 2026-10-05 | lib/scpt/corrupt.lua | specs/corruption/spec.md | 十二腐化定义全量：Balrog/Demon/传送/Troll Blood/Vampire 三段组；Vampire 组 removable 恒假与 SUBRACE_SAVE 改写道 |
| 2026-10-05 | lib/scpt/drunk.lua | specs/scpt-misc/spec.md | HOOK_GIVE 醉汉收酒 |
| 2026-10-05 | lib/scpt/fireprof.lua | specs/scpt-misc/spec.md | 防火任务全量：内嵌地图/三钩/56 号动作/fireproof 与 enough_points 拆分道 |
| 2026-10-05 | lib/scpt/god.lua | specs/god-quest/spec.md | 神祇任务全量：注册与八闸授予/圣物生成与收取/神庙选址/五神参数改写/方位指引 |
| 2026-10-05 | lib/scpt/gods.lua | specs/scpt-misc/spec.md | 信条双钩：One Ring 拒信与反魔法弃神 |
| 2026-10-05 | lib/scpt/help.lua | specs/scpt-misc/spec.md | 十九条情境帮助触发定义（文案不复抄）；select_context 六类映射表 |
| 2026-10-05 | lib/scpt/init.lua | specs/scpt-misc/spec.md | lib/scpt 装载序与 dg_test 可缺装载 |
| 2026-10-05 | lib/scpt/intro.lua | specs/scpt-misc/spec.md | 开场动画双幕滑入与按键中断 |
| 2026-10-05 | lib/scpt/joke.lua | specs/scpt-misc/spec.md | joke_monsters 闸与 Neil 彩蛋 |
| 2026-10-05 | lib/scpt/library.lua | specs/scpt-misc/spec.md | 图书馆任务全量：地图布怪/三槽造书器/清场判定/61 号动作成书 |
| 2026-10-05 | lib/scpt/mimic.lua | specs/scpt-misc/spec.md | 十一拟态形定义：nature 八形与域外四形（Bear/Balrog/Maia/Fire Elem.）；Vapour 写自造 levitate 字段现状记录 |
| 2026-10-05 | lib/scpt/mkeys.lua | specs/scpt-misc/spec.md | GF_INSTA_DEATH 与三 mkey（死指/Geomancy/长柄远击） |
| 2026-10-05 | lib/scpt/player.lua | specs/scpt-misc/spec.md | 出生物件：八职业赠书/鼠形斗篷/Vampire 开局三腐化 |
| 2026-10-05 | lib/scpt/powers.lua | specs/scpt-misc/spec.md | POWER_INVISIBILITY/WEB/COR_SPACE_TIME 三能力 |
| 2026-10-05 | lib/scpt/s_air.lua | specs/school-spells/spec.md | Air 六术：Noxious Cloud/Wings of Winds/Invisibility/Poison Blood/Thunderstorm/Sterilize |
| 2026-10-05 | lib/scpt/s_convey.lua | specs/school-spells/spec.md | Conveyance 六术 |
| 2026-10-05 | lib/scpt/s_demon.lua | specs/school-spells/spec.md | Demon 九术与 HOOK_WIELD_SLOT 三魔刃分槽 |
| 2026-10-05 | lib/scpt/s_divin.lua | specs/school-spells/spec.md | Divination 六术 |
| 2026-10-05 | lib/scpt/s_earth.lua | specs/school-spells/spec.md | Earth 五术 |
| 2026-10-05 | lib/scpt/s_eru.lua | specs/school-spells/spec.md | Eru 四术（piety 道） |
| 2026-10-05 | lib/scpt/s_fire.lua | specs/school-spells/spec.md | Fire 五术（含 Fire Golem 控制体 1043 号） |
| 2026-10-05 | lib/scpt/s_geom.lua | specs/school-spells/spec.md | Geomancy 八术+两自定义 GF+掷地形/掘道/channel 表/元素仆从四辅助件 |
| 2026-10-05 | lib/scpt/s_mana.lua | specs/school-spells/spec.md | Mana 四术与 Manathrust 骰组辅助 |
| 2026-10-05 | lib/scpt/s_manwe.lua | specs/school-spells/spec.md | Manwe 四术（piety 道） |
| 2026-10-05 | lib/scpt/s_melkor.lua | specs/school-spells/spec.md | Melkor 三术（piety 道）与 do_melkor_curse 共享件 |
| 2026-10-05 | lib/scpt/s_meta.lua | specs/school-spells/spec.md | Meta 五术与惯性控制系统（定时器/双钩/停件） |
| 2026-10-05 | lib/scpt/s_mind.lua | specs/school-spells/spec.md | Mind 四术 |
| 2026-10-05 | lib/scpt/s_music.lua | specs/school-spells/spec.md | Music 十四曲：停止/鼓三/琴五持续曲/号四瞬发（WIND 与 YLMIR 错引他术级现状记录） |
| 2026-10-05 | lib/scpt/s_nature.lua | specs/school-spells/spec.md | Nature 五术 |
| 2026-10-05 | lib/scpt/s_stick.lua | specs/school-spells/spec.md | Device 十三术：杖杖八件与神器激活五件（Eternal Flame 四终极神器铸道） |
| 2026-10-05 | lib/scpt/s_tempo.lua | specs/school-spells/spec.md | Temporal 四术 |
| 2026-10-05 | lib/scpt/s_tulkas.lua | specs/school-spells/spec.md | Tulkas 三术（piety 道） |
| 2026-10-05 | lib/scpt/s_udun.lua | specs/school-spells/spec.md | Udun 四术与 udun_in_book/levels_in_book 双检件 |
| 2026-10-05 | lib/scpt/s_water.lua | specs/school-spells/spec.md | Water 五术与 Geyser 骰组 |
| 2026-10-05 | lib/scpt/s_yavann.lua | specs/school-spells/spec.md | Yavanna 五术（piety 道） |
| 2026-10-05 | lib/scpt/spells.lua | specs/school-magic/spec.md | 二十二学派注册（gods 配比/depend/bonus_level）与三十五册 school_book 与装载序 |
| 2026-10-05 | lib/scpt/stores.lua | specs/scpt-misc/spec.md | 九店收购清单与 Magic/Temple 随机书上架钩 |
| 2026-10-05 | lib/scpt/test.lua | specs/scpt-misc/spec.md | Shiny-Test 法术组与 dungeon2 生成器测试（特征循环空转死代码现状） |
| 2026-10-05 | src/birth.c | specs/character-birth/spec.md | 角色创建全量提取：清单界面/清扫/双路线属性/史/装备/随机任务/问答流/收尾；modify_stat_value/recalc_skills/follow_god 随其所在文件引用 |
| 2026-10-05 | src/bldg.c | specs/building/spec.md | 建筑运行期全量提取；商店买卖四动作随 store.c 登记；amt/=2 缺括号癖好已记录 |
| 2026-10-05 | src/cave.c | specs/cave-lighting/spec.md | 洞穴运行期全量提取（视线/视野算法/怪物光照/记忆/渲染/小地图/测绘/流动/辅助）；earthquake 等不在本文件 |
| 2026-10-05 | src/cmd1.c | specs/player-melee/spec.md, specs/player-movement/spec.md | 玩家近战与移动全量提取；py_pickup_floor 随 cmd2.c、mon_take_hit 随 melee2.c 登记 |
| 2026-10-05 | src/cmd2.c | specs/player-ranged/spec.md, specs/player-movement/spec.md | 射击投掷回力镖入 player-ranged；楼梯门箱挖掘解陷撞门行走偷窃祭祀等入 player-movement；py_pickup_floor 随 object1.c 登记 |
| 2026-10-05 | src/cmd3.c | specs/inventory-commands/spec.md | 背包装备指令全量提取；inven_carry/inven_drop 等底层随 object1.c/cmd6.c 登记 |
| 2026-10-05 | src/cmd4.c | specs/interface-commands/spec.md | 界面指令全量提取：选项/宏/词形颜色/知识界面/层感/时间/宏录制；display_player 随 xtra2.c 登记 |
| 2026-10-05 | src/cmd5.c | specs/spell-casting/spec.md | 施法接口全量提取：学院法术选目施放誊抄/共生夺舍能力表/野魔法/烙印/取物；法术实体随 lib/scpt 登记 |
| 2026-10-05 | src/cmd6.c | specs/item-usage/spec.md | 物件使用全量提取：尸体/食物/药水/泉水/卷轴/杖魔杖/罗盘罗尖/激活ACT表/诅咒甲武；activate_stick/spell_chance 实体随 lib/scpt 登记 |
| 2026-10-05 | src/cmd7.c | specs/class-powers/spec.md | 职业技能指令全量提取：心灵/拟态/驯兽/炼金全套/奇术/射手/亡灵/符文/拒信/召唤师/共生/夺舍/剑客；random_spells 生成随 spells2 登记 |
| 2026-10-05 | src/defines.h | specs/defines/spec.md | 全局常量总纲全量提取（十二组域登记）；具体数值以锚点为准 |
| 2026-10-05 | src/dungeon.c | specs/game-loop/spec.md | 主循环全量提取：play_game/dungeon/process_player/process_command/process_world 四级循环 + 伪鉴定/图案地砖/回程/复活裁定；activate_ty_curse/activate_dg_curse/extract_energy/wisdom_scale/compact_monsters 随其所在文件登记 |
| 2026-10-05 | src/dungeon.pkg | specs/lua-binding/spec.md | tolua 绑定面全量：公共格式契约+CAVE/FEAT/DF 常量全表+七结构体+变量与别名+十四函数绑定 |
| 2026-10-05 | src/files.c | specs/pref-file/spec.md, specs/player-display/spec.md, specs/death-score/spec.md | 偏好文件/角色显示/死亡计分三能力全量提取；save_player/make_bones 随 loadsave.c、self_knowledge/dump_skills 随其所在文件登记 |
| 2026-10-05 | src/gen_evol.c | specs/dungeon-generation/spec.md | 生命游戏生成器全量提取；调度随 Lua 注册与 generate.c 登记 |
| 2026-10-05 | src/gen_maze.c | specs/dungeon-generation/spec.md | 迷宫生成器全量提取 |
| 2026-10-05 | src/generate.c | specs/dungeon-generation/spec.md | 关卡生成全管线：generate_cave 分派与重掷/cave_gen 编排与配额/level_generate_dungeon 房间抽签与矿脉河流/十二房间家族/七 vault/分形洞/特殊关 process_dungeon_file 调用侧/DF1_DOUBLE/竞技场；地图指令格式归 init1.c |
| 2026-10-05 | src/ghost.c | specs/ghost/spec.md | 玩家幽灵机制整体停用（#if 0），place_ghost 恒败；停用实现的保留形态已记录备查 |
| 2026-10-05 | src/gods.c | specs/god/spec.md | 神祇系统全量提取；数据初始化在 Lua 侧（gods.lua）与启动代码，随对应文件分析登记 |
| 2026-10-05 | src/init1.c | specs/data-loading/spec.md, specs/map-format/spec.md | 词表解析核心全量：旗标名表/包含栈/p_basic 四段/v-f-k-a-al-set-s-ab-e-ra-r-re-t-d-st-ba-ow-wf 各解析器行文法与现状偏差（e_info D停用、s_info A实为f、re_info D无分支、ibncluding 拼写现状）；process_dungeon_file 地图指令文（兑现地图内容族跳过承诺）；条目内容不逐条析 |
| 2026-10-05 | src/init2.c | specs/boot-loading/spec.md | 引导装载全量：目录布局/raw 镜像样板与 al-v 停用快速路径/init_misc 任务神祇初值/扩容件/init_other 选项初值/alloc 两表/守卫标记/init_angband 装载序；p_info 错误文案 df 拼写按现状记录 |
| 2026-10-05 | src/levels.c | specs/dungeon-level/spec.md | 关卡指令文件机制全量提取；装载时机随 dungeon.c/generate.c 追加；旗标名称表随 init1.c 全量分析登记 |
| 2026-10-05 | src/loadsave.c | specs/save-load/spec.md | 存读档全量：字节层序列化/do_item 修复链/do_monster 内联 sr_ptr/do_extra 九十余字段/do_dungeon 与 RLE 网格/save-load 主流程/存档总布局/Lua 扩展槽；现状缺陷：last_teleportation_x 从不落档、junkinit exit_bldg 冗余、BZ do_grid case8 参数不匹配 |
| 2026-10-05 | src/lua_bind.c | specs/lua-binding/spec.md | Lua 游戏辅助 API 全量提取；内容定义本身在 lib/core/*.lua |
| 2026-10-05 | src/main.c | specs/program-entry/spec.md | 程序入口全量：参数解析/目录覆盖/启动序列与后端分派链；main-xxx 平台后端仍按排除范围处理 |
| 2026-10-05 | src/melee1.c | specs/monster-melee/spec.md | 怪物近战全量提取；take_hit/元素伤害例程/震地随其所在文件登记 |
| 2026-10-05 | src/melee2.c | specs/monster-ai/spec.md | 怪物智能全量：对怪/对玩家两套施法案由与学习、choose_attack_spell 七桶、移动决策（包夹/伏击/夺舍/被控）、怪间互击、process_monster 单体推进、summon_maint、process_monsters 主循环；make_attack_normal 归 melee1.c |
| 2026-10-05 | src/modules.c | specs/module/spec.md | 模块系统全量提取；模块清单 Lua（mods_aux/modules/module.lua）随 Lua 分析登记 |
| 2026-10-05 | src/monster.pkg | specs/lua-binding/spec.md | tolua 绑定面全量：MSTATUS/RF1-9/MFLAG/SUMMON 常量全表+三结构体+变量与函数（get_mon_num_hook 钩列表入档）；monster_desc 与 monster_race_desc 双绑现状记录 |
| 2026-10-05 | src/monster1.c | specs/monster-memory/spec.md | 怪物回忆/图鉴与栖息地过滤全量提取；见识累积随 monster2.c 登记 |
| 2026-10-05 | src/monster2.c | specs/monster-generation/spec.md | 怪物生成/登记/抽取/放置/召唤/繁殖/可见性/见识累积全量提取 |
| 2026-10-05 | src/monster3.c | specs/monster-faction/spec.md | 怪物阵营/附身/伙伴/意控全量提取；繁殖生成与控制授予随 monster2.c、施法系统分析登记 |
| 2026-10-05 | src/object.pkg | specs/lua-binding/spec.md | tolua 绑定面全量：TR1-5/ESP/INVEN/TV/SV/IDENT/OBJ_FOUND/SENSE 常量全表+四结构体+变量与函数+单例与默认参与包装宏格式增量 |
| 2026-10-05 | src/object1.c | specs/object-core/spec.md | 物件核心全量：flavor 系统/reset_visuals/object_flags 三层合成/object_desc 全文法/get_item 选取机/py_pickup_floor（挂账兑现）/装备槽/有智武器升级/套装挂卸 |
| 2026-10-05 | src/object2.c | specs/object-gen/spec.md | 物件生成与操作全量：列表管理与压缩/分配抽签与主题钩/价值折算/堆叠判定/object_prep 与 m_bonus/神器 ego 生成/apply_magic 全家/放置掉落/背包操作/尸体腐化 |
| 2026-10-05 | src/player.pkg | specs/lua-binding/spec.md | tolua 绑定面全量：player_type 全字段/race 与 race_mod 结构体（oflags 按级旗标）/PWR 与 GOD 与调度位常量/set_* 再声明/apply_flags 与神系与身份切换函数 |
| 2026-10-05 | src/player_c.pkg | specs/lua-binding/spec.md | tolua 绑定面全量：class/skill/ability 三结构体+SKILL 59 号与 AB 11 号常量+技能函数组 |
| 2026-10-05 | src/plots.c | specs/hook/spec.md, specs/quest/spec.md | 钩子引擎全量提取；q_*.c 为本文件文本包含，剧情行为随各 q_* 文件逐个登记 |
| 2026-10-05 | src/plots.h | specs/quest/spec.md | 纯导出声明：七条剧情线的 init 钩子清单，与 plots.c 包含结构互证 |
| 2026-10-05 | src/powers.c | specs/racial-powers/spec.md | 能力管线全量：power_chance 裁定/select_power 菜单/全部 PWR_* 案由；能力表数据归 tables、归属归 birth/xtra |
| 2026-10-05 | src/q_betwen.c | specs/quest/spec.md | 空跃门连线：45 级门槛、一次性野外伏击、金号角奖励 |
| 2026-10-05 | src/q_dragons.c | specs/quest/spec.md | 龙巢：柱阵 + 八色四阶龙群、清场完结 |
| 2026-10-05 | src/q_eol.c | specs/quest/spec.md | 埃奥尔：程序化洞窟、逃离断线、矮人灯奖励；2^randint 实为异或已记录 |
| 2026-10-05 | src/q_evil.c | specs/quest/spec.md | 卡扎督姆炎魔：六炎魔、直接完结跳过市长（TODO 已记录） |
| 2026-10-05 | src/q_haunted.c | specs/quest/spec.md | 闹鬼之屋：幽灵/闹鬼怪/陷阱三批布设 |
| 2026-10-05 | src/q_hobbit.c | specs/quest/spec.md | 霍比特人寻子：十日冷却镇中事件、回城卷轴救子、法杖奖励 |
| 2026-10-05 | src/q_invas.c | specs/quest/spec.md | 刚多林保卫战：征召、梅格林 AI 接管、+150 生命赐福、陷落结局 |
| 2026-10-05 | src/q_main.c | specs/quest/spec.md | 主线三段推进链与索伦复活条款全量提取 |
| 2026-10-05 | src/q_narsil.c | specs/quest/spec.md | 断剑重铸：鉴定接取、城堡重铸安都瑞尔，支线不推剧情 |
| 2026-10-05 | src/q_nazgul.c | specs/quest/spec.md | 戒灵乌维斯塔：30 级门槛、镇中放置、阿西拉斯奖励、Bree 线终结 |
| 2026-10-05 | src/q_nirna.c | specs/quest/spec.md | 尼尔诺斯：歼灭计数三分之二分档、珠宝店准入（文件名 q_nirna.c，函数 quest_nirnaeth_*） |
| 2026-10-05 | src/q_one.c | specs/quest/spec.md | 至尊戒获取/佩戴代价/销毁/记录全量提取 |
| 2026-10-05 | src/q_poison.c | specs/quest/spec.md | 污染水源：四选一野外污染点、filtrate 怪物铺设、99 净水药水、蓝龙鳞甲精灵称号奖励 |
| 2026-10-05 | src/q_rand.c | specs/quest/spec.md | 随机任务系统全量提取：夺剑/公主两类、嵌入房间、三选一酬谢与神器归还；记录数词缺陷已记录 |
| 2026-10-05 | src/q_shroom.c | specs/quest/spec.md | 农夫蘑菇田：菌田生成、夜间农夫、杀狗败局、投石索奖励 |
| 2026-10-05 | src/q_spider.c | specs/quest/spec.md | 幽暗密林蜘蛛：清场完结、雅万纳 6000 虔诚、强身药水奖励 |
| 2026-10-05 | src/q_thief.c | specs/quest/spec.md | 盗贼团巢穴：缴械入场、警报机关、清场完结、技能比分岔 |
| 2026-10-05 | src/q_thrain.c | specs/quest/spec.md | 瑟莱因营救：拟态密室、双戒灵计数、临终揭示衔接主线、随机神器龙盔 |
| 2026-10-05 | src/q_troll.c | specs/quest/spec.md | 石巨魔巢穴：Glamdring 携带、一次性伏兵、汤姆死亡完结 |
| 2026-10-05 | src/q_ultrae.c | specs/quest-ultra/spec.md | Evil ultra ending 现状空壳 |
| 2026-10-05 | src/q_ultrag.c | specs/quest-ultra/spec.md | Good ultra ending 全量：Galadriel 触发/Void 层规则/Flame Imperishable/Melkor 永灭与 Tik 掉落 |
| 2026-10-05 | src/q_wight.c | specs/quest/spec.md | 尸妖王：of the Wight 布衣、王亡脚下开门、转戒灵线 |
| 2026-10-05 | src/q_wolves.c | specs/quest/spec.md | 罗瑞恩狼群：狼与座狼成批放置、敌对数清场 |
| 2026-10-05 | src/quest.pkg | specs/lua-binding/spec.md | tolua 绑定面全量：QUEST_STATUS 八档+quest_type+quest 取件与三函数 |
| 2026-10-05 | src/randart.c | specs/randart-generation/spec.md | 随机神器生成全量提取；curse_artifact/flag_cost/add_random_ego_flag 随其所在文件分析登记 |
| 2026-10-05 | src/script.c | specs/lua-engine/spec.md | Lua 引擎层全量提取；九个 tolua 绑定包的 API 面随 src/*.pkg 分析登记 |
| 2026-10-05 | src/skills.c | specs/skills/spec.md | 技能系统核心全量：折算/树界面/联动与乱数法术/搏击流派/可激活菜单/赠送技能/禁制谓词/能力系统；dump_skills 挂账兑现；autoskiller 占位现状记录 |
| 2026-10-05 | src/spells.pkg | specs/lua-binding/spec.md | tolua 绑定面全量：GF/PROJECT/EFF 常量全表+magic_power 与 school 系统+project 与 fire_* 与传送族全绑定；spell_type 类型别名与 @dgonly 格式增量 |
| 2026-10-05 | src/spells1.c | specs/spell-effects/spec.md | 投射系统全量：传送家族/take_hit 管线/元素伤害与物件损毁/属性操作/project_path/project_f-o-m-p 四面体全案由/project 主函数/药水碎裂/乱数法术生成；fire_ball 等发射件归 spells2.c |
| 2026-10-05 | src/spells2.c | specs/spell-casting/spec.md | 法术发射件全量：fire_* 家族/project_hack 族/detect 族/鉴定充能附魔/randart 造词四件/毁灭地震/房间明暗/passwall 换位/TY-DG 咴/genocide 族/self_knowledge/bless_weapon/reset_recall/between 门 |
| 2026-10-05 | src/squeltch.c | specs/automatizer/spec.md | 自动拾取器 C 侧全量提取；规则引擎本体（apply_rules/auto_aux）在 Lua 侧，随 lib/core/auto.lua 分析登记 |
| 2026-10-05 | src/status.c | specs/status-screens/spec.md | 状态屏全量：八分页矩阵/az_line 现状用 object_flags 泄露未识旗标的调试态/statline 着色/伙伴列表 |
| 2026-10-05 | src/store.c | specs/store-runtime/spec.md | 商店运行期全量提取；价格取值依赖 object_value（随 object1/2.c 登记） |
| 2026-10-05 | src/tables.c | specs/tables/spec.md | 全局数据表总集全量登记；option_info 缺省值逐项收录 |
| 2026-10-05 | src/traps.c | specs/trap/spec.md | 陷阱运行期全量提取（触发协议/九大家族/布设/怪物陷阱）；earthquake/curse/project 等被调例程随其所在文件登记 |
| 2026-10-05 | src/types.h | specs/core-data/spec.md | 核心数据模型全量提取（12 条 requirement 覆盖全部实体结构） |
| 2026-10-05 | src/util.c | specs/util/spec.md | 工具层全量提取；Term_* 终端抽象随平台层登记 |
| 2026-10-05 | src/util.pkg | specs/lua-binding/spec.md | tolua 绑定面全量：HOOK 0-77 全表与五形 GF_EXEC/TERM 色与 FF1/cave_type/timer/list 结构体/ANGBAND 目录族/adj 表 21 张/输入显示查询钩脚本存档洞穴生成界面临时件全绑定 |
| 2026-10-05 | src/variable.c | specs/global-state/spec.md | 全局变量清单与初值全量提取；变量语义由使用方能力承载 |
| 2026-10-05 | src/wild.c | specs/wilderness/spec.md | 野外与城镇全量：plasma 分形/generate_area/wilderness_gen 双态/reveal/三形态城镇与店铺件；调用入口归 dungeon-generation、地图文归 map-format |
| 2026-10-05 | src/wizard1.c | specs/spoilers/spec.md | 剧透生成器全量提取 |
| 2026-10-05 | src/wizard2.c | specs/wizard-debug/spec.md | 调试指令全量提取；make_wish/status_main 随其所在文件登记 |
| 2026-10-05 | src/xtra1.c | specs/player-derive/spec.md, specs/ui-frames/spec.md | 全量：属性换算/calc_body 躯体与夺舍/calc_gods/apply_flags 全旗标/calc_bonuses 巨构/四资源推算/calc_powers 与 spells 空壳/流派谓词/命运系统；展示半部与五级调度归 ui-frames；血条死亡分支恒假现状记录 |
| 2026-10-05 | src/xtra2.c | specs/player-states/spec.md, specs/experience-system/spec.md, specs/monster-death/spec.md, specs/targeting-panels/spec.md, specs/chaos-patron/spec.md, specs/wish-corruption/spec.md | 全量：set_* 时限态全族与 rush/stun/cut/food 特例/身份切换与 rebirth/check_experience 经验系/mon_take_hit 与 monster_death 全链（make_wish 挂账兑现）/面板与目标系统/chaos 奖励表/腐化桥；switch_subrace 越界判断恒不拒与 create_stairs 死代码现状记录 |
| 2026-10-05 | src/z-rand.c | specs/rng/spec.md | RNG 全量提取；种子管理与存档状态随 init2.c/loadsave.c 登记 |
| 2026-10-05 | src/z-rand.h | specs/rng/spec.md | 便捷宏定义（rand_int/rand_range/rand_spread/randint/magik）与 RNG 常量 |
| 2026-10-05 | src/z_pack.pkg | specs/lua-binding/spec.md | tolua 绑定面全量：TERM_XTRA 十五动作+Term_* 九件+随机包装五件+damroll/maxroll+zsock 套接字钩表（read 停用现状记录） |
