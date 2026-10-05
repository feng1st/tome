# tome2 分析清单

扫描日期: 2026-10-05；状态与指纹的唯一事实来源是 state.json，本文件由 tools/tome2spec.py report 生成，勿手改。

完成判定: `python3 tools/tome2spec.py check` 输出 PASS 即分析完成
（不存在待分析、不存在未分类、哈希无漂移、spec 引用齐备）。

## 数据词表 (lib/edit/*.txt)

| 文件 | 行数 | 状态 | 产物 spec |
|---|---|---|---|
| lib/dngn/dun1.14 | 2 | done | specs/content-maps/spec.md |
| lib/dngn/dun10.0 | 3 | done | specs/content-maps/spec.md |
| lib/dngn/dun11.20 | 2 | done | specs/content-maps/spec.md |
| lib/dngn/dun11.22 | 2 | done | specs/content-maps/spec.md |
| lib/dngn/dun17.15 | 5 | done | specs/content-maps/spec.md |
| lib/dngn/dun18.0 | 2 | done | specs/content-maps/spec.md |
| lib/dngn/dun18.1 | 2 | done | specs/content-maps/spec.md |
| lib/dngn/dun19.11 | 5 | done | specs/content-maps/spec.md |
| lib/dngn/dun2.31 | 2 | done | specs/content-maps/spec.md |
| lib/dngn/dun22.10 | 2 | done | specs/content-maps/spec.md |
| lib/dngn/dun22.5 | 5 | done | specs/content-maps/spec.md |
| lib/dngn/dun24.0 | 3 | done | specs/content-maps/spec.md |
| lib/dngn/dun29.15 | 6 | done | specs/content-maps/spec.md |
| lib/dngn/dun3.18 | 5 | done | specs/content-maps/spec.md |
| lib/dngn/dun3.28 | 5 | done | specs/content-maps/spec.md |
| lib/dngn/dun3.3 | 5 | done | specs/content-maps/spec.md |
| lib/dngn/dun5.0 | 3 | done | specs/content-maps/spec.md |
| lib/dngn/dun5.14 | 14 | done | specs/content-maps/spec.md |
| lib/dngn/dun6.0 | 3 | done | specs/content-maps/spec.md |
| lib/edit/a_info.txt | 2889 | done | specs/artifact/spec.md, specs/edit-format/spec.md |
| lib/edit/ab_info.txt | 118 | done | specs/ability/spec.md, specs/edit-format/spec.md |
| lib/edit/al_info.txt | 2097 | done | specs/alchemy/spec.md, specs/edit-format/spec.md |
| lib/edit/ba_info.txt | 283 | done | specs/building-action/spec.md, specs/edit-format/spec.md |
| lib/edit/between.map | 71 | done | specs/content-maps/spec.md |
| lib/edit/d_info.txt | 512 | done | specs/dungeon/spec.md, specs/edit-format/spec.md |
| lib/edit/dragons.map | 43 | done | specs/content-maps/spec.md |
| lib/edit/e_info.txt | 2210 | done | specs/ego-item/spec.md, specs/edit-format/spec.md |
| lib/edit/evil.map | 52 | done | specs/content-maps/spec.md |
| lib/edit/f_info.txt | 933 | done | specs/terrain/spec.md, specs/edit-format/spec.md |
| lib/edit/haunted.map | 49 | done | specs/content-maps/spec.md |
| lib/edit/k_info.txt | 6418 | done | specs/object/spec.md, specs/edit-format/spec.md |
| lib/edit/maeglin.map | 85 | done | specs/content-maps/spec.md |
| lib/edit/misc.txt | 91 | done | specs/capacities/spec.md |
| lib/edit/nirnaeth.map | 64 | done | specs/content-maps/spec.md |
| lib/edit/numenor.txt | 80 | done | specs/content-maps/spec.md |
| lib/edit/ow_info.txt | 447 | done | specs/store-owner/spec.md, specs/edit-format/spec.md |
| lib/edit/p_info.txt | 1974 | done | specs/player/spec.md, specs/edit-format/spec.md |
| lib/edit/qrand1.map | 32 | done | specs/content-maps/spec.md |
| lib/edit/qrand10.map | 36 | done | specs/content-maps/spec.md |
| lib/edit/qrand11.map | 36 | done | specs/content-maps/spec.md |
| lib/edit/qrand12.map | 36 | done | specs/content-maps/spec.md |
| lib/edit/qrand14.map | 37 | done | specs/content-maps/spec.md |
| lib/edit/qrand5.map | 27 | done | specs/content-maps/spec.md |
| lib/edit/qrand6.map | 37 | done | specs/content-maps/spec.md |
| lib/edit/qrand7.map | 35 | done | specs/content-maps/spec.md |
| lib/edit/r_info.txt | 18978 | done | specs/monster/spec.md |
| lib/edit/ra_info.txt | 1927 | done | specs/randart-part/spec.md, specs/edit-format/spec.md |
| lib/edit/re_info.txt | 183 | done | specs/monster-ego/spec.md, specs/edit-format/spec.md |
| lib/edit/s_crypt.map | 109 | done | specs/content-maps/spec.md |
| lib/edit/s_death.map | 104 | done | specs/content-maps/spec.md |
| lib/edit/s_doom.map | 226 | done | specs/content-maps/spec.md |
| lib/edit/s_factory.map | 238 | done | specs/content-maps/spec.md |
| lib/edit/s_gates.map | 117 | done | specs/content-maps/spec.md |
| lib/edit/s_info.txt | 546 | done | specs/skill/spec.md, specs/edit-format/spec.md |
| lib/edit/s_name.map | 110 | done | specs/content-maps/spec.md |
| lib/edit/s_orc.map | 109 | done | specs/content-maps/spec.md |
| lib/edit/s_ship.map | 239 | done | specs/content-maps/spec.md |
| lib/edit/set_info.txt | 77 | done | specs/item-set/spec.md, specs/edit-format/spec.md |
| lib/edit/special.txt | 67 | done | specs/content-maps/spec.md |
| lib/edit/spiders.map | 66 | done | specs/content-maps/spec.md |
| lib/edit/st_info.txt | 744 | done | specs/store/spec.md, specs/edit-format/spec.md |
| lib/edit/t_basic.txt | 80 | done | specs/content-maps/spec.md |
| lib/edit/t_bree.txt | 131 | done | specs/content-maps/spec.md |
| lib/edit/t_d_bree.txt | 101 | done | specs/content-maps/spec.md |
| lib/edit/t_d_gond.txt | 118 | done | specs/content-maps/spec.md |
| lib/edit/t_d_khaz.txt | 87 | done | specs/content-maps/spec.md |
| lib/edit/t_d_lori.txt | 101 | done | specs/content-maps/spec.md |
| lib/edit/t_d_mina.txt | 91 | done | specs/content-maps/spec.md |
| lib/edit/t_gondol.txt | 219 | done | specs/content-maps/spec.md |
| lib/edit/t_info.txt | 41 | done | specs/content-maps/spec.md |
| lib/edit/t_khazad.txt | 105 | done | specs/content-maps/spec.md |
| lib/edit/t_lorien.txt | 162 | done | specs/content-maps/spec.md |
| lib/edit/t_minas.txt | 144 | done | specs/content-maps/spec.md |
| lib/edit/t_pref.txt | 111 | done | specs/content-maps/spec.md |
| lib/edit/thieves.map | 70 | done | specs/content-maps/spec.md |
| lib/edit/thrain.map | 35 | done | specs/content-maps/spec.md |
| lib/edit/tr_info.txt | 817 | done | specs/trap/spec.md, specs/edit-format/spec.md |
| lib/edit/trolls.map | 58 | done | specs/content-maps/spec.md |
| lib/edit/v_info.txt | 2287 | done | specs/vault/spec.md, specs/edit-format/spec.md |
| lib/edit/volcano.txt | 83 | done | specs/content-maps/spec.md |
| lib/edit/w_info.txt | 120 | done | specs/content-maps/spec.md |
| lib/edit/wf_info.txt | 170 | done | specs/wilderness-terrain/spec.md, specs/edit-format/spec.md |
| lib/edit/wights.map | 82 | done | specs/content-maps/spec.md |
| lib/edit/wolves.map | 55 | done | specs/content-maps/spec.md |
| lib/file/book-0.txt | 86 | done | specs/flavor-tables/spec.md |
| lib/file/book-1.txt | 83 | done | specs/flavor-tables/spec.md |
| lib/file/book-10.txt | 250 | done | specs/flavor-tables/spec.md |
| lib/file/book-101.txt | 4 | done | specs/flavor-tables/spec.md |
| lib/file/book-102.txt | 4 | done | specs/flavor-tables/spec.md |
| lib/file/book-103.txt | 6 | done | specs/flavor-tables/spec.md |
| lib/file/book-104.txt | 6 | done | specs/flavor-tables/spec.md |
| lib/file/book-105.txt | 7 | done | specs/flavor-tables/spec.md |
| lib/file/book-106.txt | 5 | done | specs/flavor-tables/spec.md |
| lib/file/book-107.txt | 6 | done | specs/flavor-tables/spec.md |
| lib/file/book-11.txt | 250 | done | specs/flavor-tables/spec.md |
| lib/file/book-12.txt | 250 | done | specs/flavor-tables/spec.md |
| lib/file/book-13.txt | 250 | done | specs/flavor-tables/spec.md |
| lib/file/book-14.txt | 250 | done | specs/flavor-tables/spec.md |
| lib/file/book-15.txt | 250 | done | specs/flavor-tables/spec.md |
| lib/file/book-16.txt | 250 | done | specs/flavor-tables/spec.md |
| lib/file/book-17.txt | 250 | done | specs/flavor-tables/spec.md |
| lib/file/book-18.txt | 250 | done | specs/flavor-tables/spec.md |
| lib/file/book-19.txt | 250 | done | specs/flavor-tables/spec.md |
| lib/file/book-2.txt | 90 | done | specs/flavor-tables/spec.md |
| lib/file/book-20.txt | 139 | done | specs/flavor-tables/spec.md |
| lib/file/book-200.txt | 5 | done | specs/flavor-tables/spec.md |
| lib/file/book-201.txt | 5 | done | specs/flavor-tables/spec.md |
| lib/file/book-202.txt | 5 | done | specs/flavor-tables/spec.md |
| lib/file/book-203.txt | 5 | done | specs/flavor-tables/spec.md |
| lib/file/book-4.txt | 11 | done | specs/flavor-tables/spec.md |
| lib/file/book-6.txt | 263 | done | specs/flavor-tables/spec.md |
| lib/file/book-7.txt | 141 | done | specs/flavor-tables/spec.md |
| lib/file/book-8.txt | 47 | done | specs/flavor-tables/spec.md |
| lib/file/book-9.txt | 240 | done | specs/flavor-tables/spec.md |
| lib/file/bravado.txt | 106 | done | specs/flavor-tables/spec.md |
| lib/file/chainswd.txt | 8 | done | specs/flavor-tables/spec.md |
| lib/file/dam_huge.txt | 9 | done | specs/flavor-tables/spec.md |
| lib/file/dam_lots.txt | 21 | done | specs/flavor-tables/spec.md |
| lib/file/dam_med.txt | 25 | done | specs/flavor-tables/spec.md |
| lib/file/dam_none.txt | 24 | done | specs/flavor-tables/spec.md |
| lib/file/dam_xxx.txt | 11 | done | specs/flavor-tables/spec.md |
| lib/file/dead.txt | 24 | done | specs/flavor-tables/spec.md |
| lib/file/death.txt | 351 | done | specs/flavor-tables/spec.md |
| lib/file/elvish.txt | 218 | done | specs/flavor-tables/spec.md |
| lib/file/error.txt | 67 | done | specs/flavor-tables/spec.md |
| lib/file/mondeath.txt | 334 | done | specs/flavor-tables/spec.md |
| lib/file/monfear.txt | 63 | done | specs/flavor-tables/spec.md |
| lib/file/monspeak.txt | 361 | done | specs/flavor-tables/spec.md |
| lib/file/news.txt | 24 | done | specs/flavor-tables/spec.md |
| lib/file/news2.txt | 24 | done | specs/flavor-tables/spec.md |
| lib/file/rart_f.txt | 86 | done | specs/flavor-tables/spec.md |
| lib/file/rart_s.txt | 87 | done | specs/flavor-tables/spec.md |
| lib/file/rumors.txt | 201 | done | specs/flavor-tables/spec.md |
| lib/file/sfail.txt | 34 | done | specs/flavor-tables/spec.md |
| lib/file/silly.txt | 301 | done | specs/flavor-tables/spec.md |
| lib/file/smeagol.txt | 29 | done | specs/flavor-tables/spec.md |
| lib/file/smeagolr.txt | 5 | done | specs/flavor-tables/spec.md |
| lib/file/speakpet.txt | 53 | done | specs/flavor-tables/spec.md |
| lib/file/timefun.txt | 92 | done | specs/flavor-tables/spec.md |
| lib/file/timenorm.txt | 83 | done | specs/flavor-tables/spec.md |

## 引擎源码 (src/*.c)

| 文件 | 行数 | 状态 | 产物 spec |
|---|---|---|---|
| src/birth.c | 3920 | done | specs/character-birth/spec.md |
| src/bldg.c | 2273 | done | specs/building/spec.md |
| src/cave.c | 5266 | done | specs/cave-lighting/spec.md |
| src/cmd1.c | 5501 | done | specs/player-melee/spec.md, specs/player-movement/spec.md |
| src/cmd2.c | 5266 | done | specs/player-ranged/spec.md, specs/player-movement/spec.md |
| src/cmd3.c | 2531 | done | specs/inventory-commands/spec.md |
| src/cmd4.c | 4808 | done | specs/interface-commands/spec.md |
| src/cmd5.c | 2583 | done | specs/spell-casting/spec.md |
| src/cmd6.c | 7948 | done | specs/item-usage/spec.md |
| src/cmd7.c | 7984 | done | specs/class-powers/spec.md |
| src/dungeon.c | 6149 | done | specs/game-loop/spec.md |
| src/files.c | 7219 | done | specs/pref-file/spec.md, specs/player-display/spec.md, specs/death-score/spec.md |
| src/gen_evol.c | 159 | done | specs/dungeon-generation/spec.md |
| src/gen_maze.c | 297 | done | specs/dungeon-generation/spec.md |
| src/generate.c | 9024 | done | specs/dungeon-generation/spec.md |
| src/ghost.c | 1140 | done | specs/ghost/spec.md |
| src/gods.c | 140 | done | specs/god/spec.md |
| src/init1.c | 12170 | done | specs/data-loading/spec.md, specs/map-format/spec.md |
| src/init2.c | 6891 | done | specs/boot-loading/spec.md |
| src/levels.c | 240 | done | specs/dungeon-level/spec.md |
| src/loadsave.c | 3977 | done | specs/save-load/spec.md |
| src/lua_bind.c | 652 | done | specs/lua-binding/spec.md |
| src/main.c | 1076 | done | specs/program-entry/spec.md |
| src/melee1.c | 3108 | done | specs/monster-melee/spec.md |
| src/melee2.c | 7837 | done | specs/monster-ai/spec.md |
| src/modules.c | 289 | done | specs/module/spec.md |
| src/monster1.c | 1998 | done | specs/monster-memory/spec.md |
| src/monster2.c | 4099 | done | specs/monster-generation/spec.md |
| src/monster3.c | 679 | done | specs/monster-faction/spec.md |
| src/object1.c | 6866 | done | specs/object-core/spec.md |
| src/object2.c | 6663 | done | specs/object-gen/spec.md |
| src/plots.c | 473 | done | specs/hook/spec.md, specs/quest/spec.md |
| src/powers.c | 1412 | done | specs/racial-powers/spec.md |
| src/q_betwen.c | 190 | done | specs/quest/spec.md |
| src/q_dragons.c | 149 | done | specs/quest/spec.md |
| src/q_eol.c | 194 | done | specs/quest/spec.md |
| src/q_evil.c | 116 | done | specs/quest/spec.md |
| src/q_haunted.c | 145 | done | specs/quest/spec.md |
| src/q_hobbit.c | 195 | done | specs/quest/spec.md |
| src/q_invas.c | 233 | done | specs/quest/spec.md |
| src/q_main.c | 176 | done | specs/quest/spec.md |
| src/q_narsil.c | 118 | done | specs/quest/spec.md |
| src/q_nazgul.c | 115 | done | specs/quest/spec.md |
| src/q_nirna.c | 111 | done | specs/quest/spec.md |
| src/q_one.c | 363 | done | specs/quest/spec.md |
| src/q_poison.c | 237 | done | specs/quest/spec.md |
| src/q_rand.c | 442 | done | specs/quest/spec.md |
| src/q_shroom.c | 291 | done | specs/quest/spec.md |
| src/q_spider.c | 110 | done | specs/quest/spec.md |
| src/q_thief.c | 172 | done | specs/quest/spec.md |
| src/q_thrain.c | 246 | done | specs/quest/spec.md |
| src/q_troll.c | 180 | done | specs/quest/spec.md |
| src/q_ultrae.c | 11 | done | specs/quest-ultra/spec.md |
| src/q_ultrag.c | 276 | done | specs/quest-ultra/spec.md |
| src/q_wight.c | 156 | done | specs/quest/spec.md |
| src/q_wolves.c | 128 | done | specs/quest/spec.md |
| src/randart.c | 496 | done | specs/randart-generation/spec.md |
| src/script.c | 641 | done | specs/lua-engine/spec.md |
| src/skills.c | 1795 | done | specs/skills/spec.md |
| src/spells1.c | 9412 | done | specs/spell-effects/spec.md |
| src/spells2.c | 8214 | done | specs/spell-casting/spec.md |
| src/squeltch.c | 551 | done | specs/automatizer/spec.md |
| src/status.c | 778 | done | specs/status-screens/spec.md |
| src/store.c | 4531 | done | specs/store-runtime/spec.md |
| src/tables.c | 4804 | done | specs/tables/spec.md |
| src/traps.c | 3426 | done | specs/trap/spec.md |
| src/util.c | 4873 | done | specs/util/spec.md |
| src/variable.c | 1628 | done | specs/global-state/spec.md |
| src/wild.c | 1316 | done | specs/wilderness/spec.md |
| src/wizard1.c | 2830 | done | specs/spoilers/spec.md |
| src/wizard2.c | 2065 | done | specs/wizard-debug/spec.md |
| src/xtra1.c | 4855 | done | specs/player-derive/spec.md, specs/ui-frames/spec.md |
| src/xtra2.c | 7795 | done | specs/player-states/spec.md, specs/experience-system/spec.md, specs/monster-death/spec.md, specs/targeting-panels/spec.md, specs/chaos-patron/spec.md, specs/wish-corruption/spec.md |
| src/z-rand.c | 355 | done | specs/rng/spec.md |

## 引擎头文件 (src/*.h)

| 文件 | 行数 | 状态 | 产物 spec |
|---|---|---|---|
| src/defines.h | 4722 | done | specs/defines/spec.md |
| src/plots.h | 47 | done | specs/quest/spec.md |
| src/types.h | 2551 | done | specs/core-data/spec.md |
| src/z-rand.h | 89 | done | specs/rng/spec.md |

## Lua 绑定声明 (src/*.pkg)

| 文件 | 行数 | 状态 | 产物 spec |
|---|---|---|---|
| src/dungeon.pkg | 1618 | done | specs/lua-binding/spec.md |
| src/monster.pkg | 2324 | done | specs/lua-binding/spec.md |
| src/object.pkg | 1171 | done | specs/lua-binding/spec.md |
| src/player.pkg | 3525 | done | specs/lua-binding/spec.md |
| src/player_c.pkg | 1060 | done | specs/lua-binding/spec.md |
| src/quest.pkg | 170 | done | specs/lua-binding/spec.md |
| src/spells.pkg | 2498 | done | specs/lua-binding/spec.md |
| src/util.pkg | 2735 | done | specs/lua-binding/spec.md |
| src/z_pack.pkg | 693 | done | specs/lua-binding/spec.md |

## Lua 脚本 (lib/{core,scpt,mods})

| 文件 | 行数 | 状态 | 产物 spec |
|---|---|---|---|
| lib/core/auto.lua | 803 | done | specs/automatizer/spec.md |
| lib/core/building.lua | 15 | done | specs/lua-core/spec.md |
| lib/core/crpt_aux.lua | 243 | done | specs/corruption/spec.md |
| lib/core/dungeon.lua | 106 | done | specs/lua-core/spec.md |
| lib/core/gen_idx.lua | 261 | done | specs/lua-core/spec.md |
| lib/core/gods.lua | 40 | done | specs/lua-core/spec.md |
| lib/core/help.lua | 141 | done | specs/lua-core/spec.md |
| lib/core/init.lua | 84 | done | specs/lua-core/spec.md |
| lib/core/load.lua | 37 | done | specs/lua-core/spec.md |
| lib/core/load2.lua | 63 | done | specs/lua-core/spec.md |
| lib/core/mimc_aux.lua | 95 | done | specs/lua-core/spec.md |
| lib/core/monsters.lua | 16 | done | specs/lua-core/spec.md |
| lib/core/objects.lua | 45 | done | specs/lua-core/spec.md |
| lib/core/player.lua | 140 | done | specs/lua-core/spec.md |
| lib/core/powers.lua | 105 | done | specs/lua-core/spec.md |
| lib/core/quests.lua | 57 | done | specs/lua-core/spec.md |
| lib/core/s_aux.lua | 742 | done | specs/school-magic/spec.md |
| lib/core/stores.lua | 32 | done | specs/lua-core/spec.md |
| lib/core/util.lua | 257 | done | specs/lua-core/spec.md |
| lib/core/xml.lua | 347 | done | specs/lua-core/spec.md |
| lib/mods/mods_aux.lua | 185 | done | specs/modules/spec.md |
| lib/mods/modules.lua | 5 | done | specs/modules/spec.md |
| lib/module.lua | 36 | done | specs/modules/spec.md |
| lib/scpt/bounty.lua | 90 | done | specs/scpt-misc/spec.md |
| lib/scpt/corrupt.lua | 440 | done | specs/corruption/spec.md |
| lib/scpt/drunk.lua | 21 | done | specs/scpt-misc/spec.md |
| lib/scpt/fireprof.lua | 480 | done | specs/scpt-misc/spec.md |
| lib/scpt/god.lua | 640 | done | specs/god-quest/spec.md |
| lib/scpt/gods.lua | 26 | done | specs/scpt-misc/spec.md |
| lib/scpt/help.lua | 411 | done | specs/scpt-misc/spec.md |
| lib/scpt/init.lua | 46 | done | specs/scpt-misc/spec.md |
| lib/scpt/intro.lua | 105 | done | specs/scpt-misc/spec.md |
| lib/scpt/joke.lua | 31 | done | specs/scpt-misc/spec.md |
| lib/scpt/library.lua | 513 | done | specs/scpt-misc/spec.md |
| lib/scpt/mimic.lua | 385 | done | specs/scpt-misc/spec.md |
| lib/scpt/mkeys.lua | 95 | done | specs/scpt-misc/spec.md |
| lib/scpt/player.lua | 76 | done | specs/scpt-misc/spec.md |
| lib/scpt/powers.lua | 61 | done | specs/scpt-misc/spec.md |
| lib/scpt/s_air.lua | 193 | done | specs/school-spells/spec.md |
| lib/scpt/s_convey.lua | 227 | done | specs/school-spells/spec.md |
| lib/scpt/s_demon.lua | 337 | done | specs/school-spells/spec.md |
| lib/scpt/s_divin.lua | 230 | done | specs/school-spells/spec.md |
| lib/scpt/s_earth.lua | 184 | done | specs/school-spells/spec.md |
| lib/scpt/s_eru.lua | 130 | done | specs/school-spells/spec.md |
| lib/scpt/s_fire.lua | 227 | done | specs/school-spells/spec.md |
| lib/scpt/s_geom.lua | 656 | done | specs/school-spells/spec.md |
| lib/scpt/s_mana.lua | 132 | done | specs/school-spells/spec.md |
| lib/scpt/s_manwe.lua | 144 | done | specs/school-spells/spec.md |
| lib/scpt/s_melkor.lua | 154 | done | specs/school-spells/spec.md |
| lib/scpt/s_meta.lua | 287 | done | specs/school-spells/spec.md |
| lib/scpt/s_mind.lua | 132 | done | specs/school-spells/spec.md |
| lib/scpt/s_music.lua | 443 | done | specs/school-spells/spec.md |
| lib/scpt/s_nature.lua | 152 | done | specs/school-spells/spec.md |
| lib/scpt/s_stick.lua | 444 | done | specs/school-spells/spec.md |
| lib/scpt/s_tempo.lua | 162 | done | specs/school-spells/spec.md |
| lib/scpt/s_tulkas.lua | 81 | done | specs/school-spells/spec.md |
| lib/scpt/s_udun.lua | 180 | done | specs/school-spells/spec.md |
| lib/scpt/s_water.lua | 154 | done | specs/school-spells/spec.md |
| lib/scpt/s_yavann.lua | 157 | done | specs/school-spells/spec.md |
| lib/scpt/spells.lua | 475 | done | specs/school-magic/spec.md |
| lib/scpt/stores.lua | 151 | done | specs/scpt-misc/spec.md |
| lib/scpt/test.lua | 364 | done | specs/scpt-misc/spec.md |

## 跳过项

以下文件不进入分析；每项必须带理由，理由缺失会被 check 判失败。

| 文件 | 理由 |
|---|---|
| .git/COMMIT_EDITMSG | 版本控制内容 |
| .git/FETCH_HEAD | 版本控制内容 |
| .git/HEAD | 版本控制内容 |
| .git/ORIG_HEAD | 版本控制内容 |
| .git/config | 版本控制内容 |
| .git/cursor/crepe/885799917d42ea9e6eb69fc320fa03922cd8cbb4/index.bin | 版本控制内容 |
| .git/cursor/crepe/885799917d42ea9e6eb69fc320fa03922cd8cbb4/metadata.json | 版本控制内容 |
| .git/cursor/crepe/885799917d42ea9e6eb69fc320fa03922cd8cbb4/postings.bin | 版本控制内容 |
| .git/description | 版本控制内容 |
| .git/hooks/applypatch-msg.sample | 版本控制内容 |
| .git/hooks/commit-msg.sample | 版本控制内容 |
| .git/hooks/fsmonitor-watchman.sample | 版本控制内容 |
| .git/hooks/post-update.sample | 版本控制内容 |
| .git/hooks/pre-applypatch.sample | 版本控制内容 |
| .git/hooks/pre-commit.sample | 版本控制内容 |
| .git/hooks/pre-merge-commit.sample | 版本控制内容 |
| .git/hooks/pre-push.sample | 版本控制内容 |
| .git/hooks/pre-rebase.sample | 版本控制内容 |
| .git/hooks/pre-receive.sample | 版本控制内容 |
| .git/hooks/prepare-commit-msg.sample | 版本控制内容 |
| .git/hooks/push-to-checkout.sample | 版本控制内容 |
| .git/hooks/sendemail-validate.sample | 版本控制内容 |
| .git/hooks/update.sample | 版本控制内容 |
| .git/index | 版本控制内容 |
| .git/info/exclude | 版本控制内容 |
| .git/info/refs | 版本控制内容 |
| .git/logs/HEAD | 版本控制内容 |
| .git/logs/refs/heads/master | 版本控制内容 |
| .git/logs/refs/heads/v2.3.5 | 版本控制内容 |
| .git/logs/refs/remotes/origin/HEAD | 版本控制内容 |
| .git/logs/refs/remotes/origin/master | 版本控制内容 |
| .git/objects/00/91535bb7a555d1571a8b730dd11c3f7c32dba1 | 版本控制内容 |
| .git/objects/02/4432794adaa17721b8a2e9c5a24f334b92d4c6 | 版本控制内容 |
| .git/objects/02/8c7457eadc5ec0bdadd3f8442090e8e45a9b31 | 版本控制内容 |
| .git/objects/04/b0a36f37fbf54fe19a2708613cc0727faa344b | 版本控制内容 |
| .git/objects/07/f1b834d922202743d1030b4189be80f8e3c08e | 版本控制内容 |
| .git/objects/0d/1f758bf4dca49fa0e9d92ba276300e705dfa68 | 版本控制内容 |
| .git/objects/11/337e44b551a4e0772dd7d26b978ae159be7f88 | 版本控制内容 |
| .git/objects/13/624841a1ffd217c544d8597f8b2bf5bd7ece44 | 版本控制内容 |
| .git/objects/15/ec08f8abc00e79b2634688d13868579d3bf225 | 版本控制内容 |
| .git/objects/16/956b440b57eea2e146f2ededbf1c12f4837606 | 版本控制内容 |
| .git/objects/16/f02af4b9dec561abc05e42741f8e542a717680 | 版本控制内容 |
| .git/objects/1b/b5bbfaa7d8714b6ba0b414777ea802c1920c26 | 版本控制内容 |
| .git/objects/1d/a9e0eb7bc1c42e038bb2565699177cda7c19ce | 版本控制内容 |
| .git/objects/1d/c486e2b87914465894ce688f928d05a85a8927 | 版本控制内容 |
| .git/objects/1f/cd7fac1e1afe35777c55cea3c5018b4a34ae5e | 版本控制内容 |
| .git/objects/20/35d1e95e4e970448f683dea9decc35d12f5ffa | 版本控制内容 |
| .git/objects/27/146c11940a921a0c307527408e174fe8753b09 | 版本控制内容 |
| .git/objects/29/f301de06a6b0d9ade1b43dc221943cbfadd23e | 版本控制内容 |
| .git/objects/2c/46cbf0e2a670c21ec366060f5552e0693fb245 | 版本控制内容 |
| .git/objects/2d/5ac8f0739b1cd4b2a533e742d978d324f224c9 | 版本控制内容 |
| .git/objects/2d/f3f877d672709141a24d81ad9279487eee696d | 版本控制内容 |
| .git/objects/2e/61d4b430c4cbb3dd8d374f966c5f7cf3957b3d | 版本控制内容 |
| .git/objects/2e/71b6928788b37fb9336fa15e9402dd6db7fa52 | 版本控制内容 |
| .git/objects/2f/21c9a44fb4643a81df30f057beb3aa34cb7d99 | 版本控制内容 |
| .git/objects/35/d57f6181cf47835fc48e8e4d295177f0bd9638 | 版本控制内容 |
| .git/objects/36/94fb9078a02660cc5efa7dd0987cf86719689c | 版本控制内容 |
| .git/objects/38/35aa9a9f828bf6bcca342d9eab61e9cac50407 | 版本控制内容 |
| .git/objects/38/7373f5fb5ab4426c5e5316e9b615d475d0d71e | 版本控制内容 |
| .git/objects/39/0f2d08cd0c22286490783ae1906e5ae6542ef2 | 版本控制内容 |
| .git/objects/39/d5906720a700994d7e2d1fef608b3abdf8d77f | 版本控制内容 |
| .git/objects/3a/242837225e6a35b299c70408ddef04b657be8f | 版本控制内容 |
| .git/objects/3a/d48a9b53c640c865bcc54db3b36506d2e0288d | 版本控制内容 |
| .git/objects/3b/bae78d19a5f2f7bb78fd1ce9ae69b7ca1c22e6 | 版本控制内容 |
| .git/objects/3b/e5289d03a0b34ca97b491c184b66df56498858 | 版本控制内容 |
| .git/objects/3e/b23a97f705f96e645f90a435755380acb483d8 | 版本控制内容 |
| .git/objects/3f/471cc6d3b8636f5797977084bb2732ebbcef8c | 版本控制内容 |
| .git/objects/40/a01d41781c2a5550d57e30a39fd130f5141604 | 版本控制内容 |
| .git/objects/40/df99536a17ef1af6be5b65478d2c9d714e2322 | 版本控制内容 |
| .git/objects/42/31ce1ea7b97ac6622e8f3760794e474c6b7869 | 版本控制内容 |
| .git/objects/43/b38e06b9a093bf395a42ffe272dee781392e07 | 版本控制内容 |
| .git/objects/46/37f1eab7f3429b1da38730acf995f18d026f7b | 版本控制内容 |
| .git/objects/47/a46f939fa9f524cb380cd1f784dc6ca04b955e | 版本控制内容 |
| .git/objects/4a/ee3b4d78cddcc3d18bf056285ad132ccc7ae3f | 版本控制内容 |
| .git/objects/4b/b431a75e2be88c3e34c49764c4595bce7264cd | 版本控制内容 |
| .git/objects/4d/9bc116f5f61d60aa692b66d9bad853c664029d | 版本控制内容 |
| .git/objects/4e/7b03c1ba20c1d963c83f6575172302efa2ae4e | 版本控制内容 |
| .git/objects/4e/a02d42da6c885ae978ae04268b5d7835ece8d6 | 版本控制内容 |
| .git/objects/53/fe5e6cb88a2e5600cdb9ff25abf15c3a24adca | 版本控制内容 |
| .git/objects/5c/cfdca65f1f506c07e71aaed8dd05087bb6dece | 版本控制内容 |
| .git/objects/5f/193f646f3ef05c217ff44527b81d9f30168eeb | 版本控制内容 |
| .git/objects/5f/4d8231da8adb6cf59a9ba98346fbc98f8b16a0 | 版本控制内容 |
| .git/objects/66/f1e42e6d2cb7b7a6bca707de163fdff72bcfdd | 版本控制内容 |
| .git/objects/69/ef7006a8d9a53aa4549c5656d32e38a51e948e | 版本控制内容 |
| .git/objects/6a/d24c6d4736dcada817eeff7067a53a8350ca4d | 版本控制内容 |
| .git/objects/6b/bc7692eece3dff118a2872e3fc02f990ce1911 | 版本控制内容 |
| .git/objects/6c/0143800af67d8c26e359724754b76f550ca2f3 | 版本控制内容 |
| .git/objects/6c/669ea6af4ebd16720caeb3449f08e44ca10aa5 | 版本控制内容 |
| .git/objects/6f/20baa08525de08b48757fd0009bef6bc840b7d | 版本控制内容 |
| .git/objects/75/0c606ba109943d00c3a7d0af785012dda5aa35 | 版本控制内容 |
| .git/objects/75/f6e6b937dbd9cd78dc4e9450ed6d33dd3afde9 | 版本控制内容 |
| .git/objects/77/abccdfcbb770199ab0b07216cec7a8118aeb44 | 版本控制内容 |
| .git/objects/77/dd08590aef7acf81f796407dcdbf3115a04640 | 版本控制内容 |
| .git/objects/79/6aaf71a71e0d521680795a9ab2dfe90d14b610 | 版本控制内容 |
| .git/objects/7d/766a9906d8987aa64fdd353b2ee89e00ded249 | 版本控制内容 |
| .git/objects/80/f33953782b62c6bbaa21cf31fb1deb0582a36d | 版本控制内容 |
| .git/objects/82/7f3c33174107408ca64a3e9d0efd09fb55ccd7 | 版本控制内容 |
| .git/objects/84/5d5299ac84e86f542a810b0580aa713041210a | 版本控制内容 |
| .git/objects/84/bddeb73bbc580a81a4e7f3f39f7d5450a33b69 | 版本控制内容 |
| .git/objects/85/cfa5dbd9f09adf8563f55856b330adfcbe4a31 | 版本控制内容 |
| .git/objects/87/5d3b5d7a73e8e3207ea525612dea5ce6b8fd31 | 版本控制内容 |
| .git/objects/88/111482a9cc08b64a23f28ac6a5c279f7307dda | 版本控制内容 |
| .git/objects/8f/8dfe5594d967322f25401e0c50c4a08f51285d | 版本控制内容 |
| .git/objects/90/0b461b235606763a56129282cf94f8b1c6d123 | 版本控制内容 |
| .git/objects/92/71395c81c62a52958b761e0a77336ecf66ed59 | 版本控制内容 |
| .git/objects/94/65f192b87c8bf414a68706b6b95fcd93b2df5f | 版本控制内容 |
| .git/objects/98/5c5bf62214d6f37f8f7c27b008e4dbac660fdd | 版本控制内容 |
| .git/objects/99/d1a0fe32435201f2c8f0cc000273348acea5ec | 版本控制内容 |
| .git/objects/9c/032492a05bab222bf8c915f248cc712deadfd3 | 版本控制内容 |
| .git/objects/9f/bf1d14bebf71741b0a979281c8f44204c025d6 | 版本控制内容 |
| .git/objects/a0/6ee4330fa923841f2ad25f9fa189e9006f399b | 版本控制内容 |
| .git/objects/a1/f10db17683f88b857012130994f9223b230d24 | 版本控制内容 |
| .git/objects/a3/7a194aa36d659e21fef2894e719790ed085ae6 | 版本控制内容 |
| .git/objects/a7/b85820a5f8e07c5c871a1a44bc4da350305e96 | 版本控制内容 |
| .git/objects/a8/fab36276b6189d4af5949ecf5dca04f748ae03 | 版本控制内容 |
| .git/objects/a9/664e7dc0f21fb023f5068dfa402f3f34bddcb5 | 版本控制内容 |
| .git/objects/ac/fe032fd7ef1496f2ebb048f6071d9a4a944721 | 版本控制内容 |
| .git/objects/ae/0beb5f07fd6de45c8f59c83ac74c1f3657481f | 版本控制内容 |
| .git/objects/ae/5e10f86448c256ffa2dbdb1a02ddddf0eff9de | 版本控制内容 |
| .git/objects/ae/b43d377ef6fd7d2d77269b4a7d448c98013384 | 版本控制内容 |
| .git/objects/af/293dc6945b96f05bced81508738c6bb2dd7042 | 版本控制内容 |
| .git/objects/af/fc1a94fe89418bad8d31d75a3480b2792fc477 | 版本控制内容 |
| .git/objects/c0/8b198515bc4e7d3b00d137ebebec749ad06de5 | 版本控制内容 |
| .git/objects/c1/01d04cb167665df668528bf43cb3ef30e582ee | 版本控制内容 |
| .git/objects/c1/a7af78de2ef5808d2fcd8f5143f7161c929af8 | 版本控制内容 |
| .git/objects/c2/2121e0ab95f28ef87f567c367a0433db251902 | 版本控制内容 |
| .git/objects/c3/abcef7241aa365b1357257d57191021c44234d | 版本控制内容 |
| .git/objects/c4/53480c9b37b5601cfbd8df59fd8ba4ee73e63b | 版本控制内容 |
| .git/objects/c4/c8d19a4a95339c663db5404ef8dd26e8d0b26b | 版本控制内容 |
| .git/objects/c5/7d38913c1eb8efad20a0a0282efa69e6640290 | 版本控制内容 |
| .git/objects/c6/a7c77031ccf1e077f1f57ad1855eef813917f7 | 版本控制内容 |
| .git/objects/c8/331be035a8c9324944ae86d009e22384a6cf6e | 版本控制内容 |
| .git/objects/cb/e4e2d238c1660c55e287f03498206b66430d96 | 版本控制内容 |
| .git/objects/cd/23bbed7bc596540d4b47337bdff05f908e2fa9 | 版本控制内容 |
| .git/objects/cd/8b4c2dd153b5b7e7bee1373b0d7a246f6c8e26 | 版本控制内容 |
| .git/objects/ce/a7da63bc78484d12923a5496f9edc742e7aad0 | 版本控制内容 |
| .git/objects/d4/3151d2a780b3a1ce6823cab08531e60157d4db | 版本控制内容 |
| .git/objects/d5/2cdbc096e44262e1b3c16a3059bb4553ca7d84 | 版本控制内容 |
| .git/objects/d8/b0a9c3f7a054e158efefb41c5ca642a21cf476 | 版本控制内容 |
| .git/objects/db/22ab590d89bc948c4cc7781a5c7827b58d5f55 | 版本控制内容 |
| .git/objects/dc/276840849d4082489e05f3f4a8402d160c479f | 版本控制内容 |
| .git/objects/dd/a8ad1f43117d767df6d117100a92921bc9a647 | 版本控制内容 |
| .git/objects/e0/42e10bddf0c0c2a81a379dd4172282a4fdd3bd | 版本控制内容 |
| .git/objects/e1/4bb577fc7897dd73c27c321fedd40466fc8168 | 版本控制内容 |
| .git/objects/e3/a8183528b28f12cb73946223f1a759d67c13c4 | 版本控制内容 |
| .git/objects/e5/129f83cd75d1cdac85f91c1016357136391453 | 版本控制内容 |
| .git/objects/e6/0b43b5fc9b35e1d8272c8a9b804409f5f30dc1 | 版本控制内容 |
| .git/objects/e7/06f710c00b1ab3718e227ba08662e66fc3d9db | 版本控制内容 |
| .git/objects/e7/e71769043e897bbe19c120c70d9ab9ae062f1d | 版本控制内容 |
| .git/objects/e8/20d248695529680cd4e24461c107752e4a3e55 | 版本控制内容 |
| .git/objects/eb/4bd587bc29b7c05bcc762b9a8e9245d2dfb597 | 版本控制内容 |
| .git/objects/ed/8311ab1d28aff4261ea21afd67af1357a239d5 | 版本控制内容 |
| .git/objects/ef/0df499e33e2df6b9655a69a85d2d2805046d24 | 版本控制内容 |
| .git/objects/ef/1a4572ddd7aac0ecdd2b8934c3153a0b490246 | 版本控制内容 |
| .git/objects/f4/376e40466957aa5414dea9ab69e53898db02cd | 版本控制内容 |
| .git/objects/f6/43ec0772bf52b1d621b7ccc9d65d54fd6ee0d4 | 版本控制内容 |
| .git/objects/f9/67c9799f01ffb433e7f0414060994e781d926a | 版本控制内容 |
| .git/objects/f9/f7085ac46744a3cd85b0948fa9f6d44fa223e6 | 版本控制内容 |
| .git/objects/fe/24895b8be3603d21ccd9f1cef9d52c3847f351 | 版本控制内容 |
| .git/objects/info/commit-graphs/commit-graph-chain | 版本控制内容 |
| .git/objects/info/commit-graphs/graph-01e12b2e7ff93f9a461e112485a617b313f1a929.graph | 版本控制内容 |
| .git/objects/info/packs | 版本控制内容 |
| .git/objects/pack/multi-pack-index | 版本控制内容 |
| .git/objects/pack/pack-3bed8fefc61c6e1c707669ba2bec85e3e62d5001.idx | 版本控制内容 |
| .git/objects/pack/pack-3bed8fefc61c6e1c707669ba2bec85e3e62d5001.pack | 版本控制内容 |
| .git/objects/pack/pack-3bed8fefc61c6e1c707669ba2bec85e3e62d5001.rev | 版本控制内容 |
| .git/objects/pack/pack-d1635267ee368c44b89d086e036d9acdf52a824a.idx | 版本控制内容 |
| .git/objects/pack/pack-d1635267ee368c44b89d086e036d9acdf52a824a.pack | 版本控制内容 |
| .git/objects/pack/pack-d1635267ee368c44b89d086e036d9acdf52a824a.rev | 版本控制内容 |
| .git/opencode | 版本控制内容 |
| .git/packed-refs | 版本控制内容 |
| .git/refs/heads/master | 版本控制内容 |
| .git/refs/heads/v2.3.5 | 版本控制内容 |
| .git/refs/remotes/origin/HEAD | 版本控制内容 |
| .git/refs/remotes/origin/master | 版本控制内容 |
| .gitignore | 版本控制内容 |
| angdos.cfg | 发布说明与平台配置文件 |
| changes.old | 发布说明与平台配置文件 |
| changes.txt | 发布说明与平台配置文件 |
| credits.txt | 发布说明与平台配置文件 |
| lib/apex/delete.me | 运行期产物或占位目录 |
| lib/apex/scores.raw | 运行期产物或占位目录 |
| lib/bone/bone001.012 | 运行期产物或占位目录 |
| lib/bone/bone004.001 | 运行期产物或占位目录 |
| lib/bone/bone004.002 | 运行期产物或占位目录 |
| lib/bone/bone004.003 | 运行期产物或占位目录 |
| lib/bone/bone004.004 | 运行期产物或占位目录 |
| lib/bone/bone004.005 | 运行期产物或占位目录 |
| lib/bone/bone004.006 | 运行期产物或占位目录 |
| lib/bone/bone004.007 | 运行期产物或占位目录 |
| lib/bone/bone004.008 | 运行期产物或占位目录 |
| lib/bone/bone004.009 | 运行期产物或占位目录 |
| lib/bone/bone004.010 | 运行期产物或占位目录 |
| lib/cmov/delete.me | 运行期产物或占位目录 |
| lib/data/.gitignore | 运行期产物或占位目录 |
| lib/data/delete.me | 运行期产物或占位目录 |
| lib/edit/readme.txt | 词表目录说明文本（每个词表一句话介绍），无行为契约；各词表真实契约由对应 spec 承载 |
| lib/file/readme! | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/file/sample.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/TANG.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/ability.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/advanced.hlp | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/attack.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/automat.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/birth.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/bldg.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_alchem.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_archer.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_assass.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_axemas.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_bard.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_demono.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_druid.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_geoman.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_hafted.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_lorema.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_mage.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_merch.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_mimic.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_mindcr.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_monk.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_necro.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_palad.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_polear.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_posses.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_pr_drk.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_pr_eru.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_pr_man.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_priest.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_ranger.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_rogue.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_runecr.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_sorcer.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_summon.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_swordm.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_symbia.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_thaum.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_unbel.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_warper.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/c_warrio.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/command.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/corspoil.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/debug.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/def.aux | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/defines.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/dungeon.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/dunspoil.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/essences.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/experien.hlp | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/explore.hlp | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/fatespoi.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/foot.aux | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/g_eru.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/g_manwe.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/g_melkor.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/g_tulkas.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/g_yavann.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/gambling.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/general.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/gods.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/head.aux | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/help.hlp | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/index.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/inscrip.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/lua.hlp | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/lua_gf.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/lua_intr.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/lua_mon.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/lua_play.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/lua_pow.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/lua_ques.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/lua_skil.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/lua_spel.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/lua_util.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/luckspoi.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_air.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_convey.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_demono.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_divin.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_earth.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_fire.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_geoman.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_mana.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_meta.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_mimic.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_mind.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_mindcr.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_music.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_nature.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_necrom.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_symbio.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_tempo.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_thaum.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_udun.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/m_water.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/macrofaq.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/magic.hlp | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/magic.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/newbie.hlp | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/option.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_beorn.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_deathm.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_drkelf.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_dunad.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_dwarf.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_elf.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_ent.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_gnome.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_hafelf.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_hafogr.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_hielf.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_hobbit.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_human.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_kobold.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_maia.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_orc.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_pettyd.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_rohank.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_thlord.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_troll.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_wodelf.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/r_yeek.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/rm_barb.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/rm_class.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/rm_herm.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/rm_lsoul.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/rm_skel.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/rm_spec.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/rm_vamp.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/rm_zomb.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/skills.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/spoil_faq.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/spoiler.hlp | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/tome_faq.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/version.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/whattome.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/help/wishing.txt | 帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖） |
| lib/info/delete.me | 运行期产物或占位目录 |
| lib/note/delete.me | 运行期产物或占位目录 |
| lib/patch/delete.me | 运行期产物或占位目录 |
| lib/pref/422color.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/colors.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/font-ami.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/font-dos.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/font-ibm.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/font-mac.new | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/font-mac.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/font-win.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/font-x11.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/font-xxx.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/font.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/graf-ami.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/graf-dos.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/graf-ibm.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/graf-iso.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/graf-mac.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/graf-new.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/graf-sdl.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/graf-win.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/graf-x11.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/graf-xxx.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/graf.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/pref-acn.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/pref-ami.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/pref-emx.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/pref-gcu.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/pref-iso.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/pref-mac.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/pref-sdl.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/pref-win.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/pref-x11.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/pref.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/trap-iso.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/trap-xxx.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/user.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/xtra-gcu.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/xtra-new.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/pref/xtra-xxx.prf | 前端展示配置（字体/图形/颜色绑定） |
| lib/save/delete.me | 运行期产物或占位目录 |
| lib/user/automat.atm | 运行期产物或占位目录 |
| lib/user/delete.me | 运行期产物或占位目录 |
| lib/xtra/ang16.bdf | 音乐/字体/音效/图形资源 |
| lib/xtra/angband.fnt | 音乐/字体/音效/图形资源 |
| lib/xtra/font/10X20.FON | 音乐/字体/音效/图形资源 |
| lib/xtra/font/12X24.FON | 音乐/字体/音效/图形资源 |
| lib/xtra/font/5X8.FON | 音乐/字体/音效/图形资源 |
| lib/xtra/font/6X10.FON | 音乐/字体/音效/图形资源 |
| lib/xtra/font/6X12.FON | 音乐/字体/音效/图形资源 |
| lib/xtra/font/6X13.FON | 音乐/字体/音效/图形资源 |
| lib/xtra/font/6X13B.FON | 音乐/字体/音效/图形资源 |
| lib/xtra/font/6X9.FON | 音乐/字体/音效/图形资源 |
| lib/xtra/font/7X13.FON | 音乐/字体/音效/图形资源 |
| lib/xtra/font/7X13B.FON | 音乐/字体/音效/图形资源 |
| lib/xtra/font/8X13.FON | 音乐/字体/音效/图形资源 |
| lib/xtra/font/8X13B.FON | 音乐/字体/音效/图形资源 |
| lib/xtra/font/9X15.FON | 音乐/字体/音效/图形资源 |
| lib/xtra/font/9X15B.FON | 音乐/字体/音效/图形资源 |
| lib/xtra/font/VeraMono.ttf | 音乐/字体/音效/图形资源 |
| lib/xtra/font/XM10X17.FNT | 音乐/字体/音效/图形资源 |
| lib/xtra/font/XM10X17B.FNT | 音乐/字体/音效/图形资源 |
| lib/xtra/font/XM12X20.FNT | 音乐/字体/音效/图形资源 |
| lib/xtra/font/XM12X20B.FNT | 音乐/字体/音效/图形资源 |
| lib/xtra/font/XM16X25.FNT | 音乐/字体/音效/图形资源 |
| lib/xtra/font/XM16X25B.FNT | 音乐/字体/音效/图形资源 |
| lib/xtra/font/XM5X8.FNT | 音乐/字体/音效/图形资源 |
| lib/xtra/font/XM6X12.FNT | 音乐/字体/音效/图形资源 |
| lib/xtra/font/XM6X12B.FNT | 音乐/字体/音效/图形资源 |
| lib/xtra/font/XM8X16B.FNT | 音乐/字体/音效/图形资源 |
| lib/xtra/font/xm4x6.fnt | 音乐/字体/音效/图形资源 |
| lib/xtra/font/xm8x13.fnt | 音乐/字体/音效/图形资源 |
| lib/xtra/font/xm8x13b.fnt | 音乐/字体/音效/图形资源 |
| lib/xtra/font/xm8x16.fnt | 音乐/字体/音效/图形资源 |
| lib/xtra/graf/16x16.bmp | 音乐/字体/音效/图形资源 |
| lib/xtra/graf/16x16.png | 音乐/字体/音效/图形资源 |
| lib/xtra/graf/8x8.bmp | 音乐/字体/音效/图形资源 |
| lib/xtra/graf/8x8.png | 音乐/字体/音效/图形资源 |
| lib/xtra/graf/mask.bmp | 音乐/字体/音效/图形资源 |
| lib/xtra/graf/tome-128.png | 音乐/字体/音效/图形资源 |
| lib/xtra/music/delete.me | 音乐/字体/音效/图形资源 |
| lib/xtra/sound/Sound.cfg | 音乐/字体/音效/图形资源 |
| lib/xtra/sound/readme.txt | 音乐/字体/音效/图形资源 |
| src/.#dungeon.c.1.279.2.4 | 构建脚本、版本控制残留或平台资源 |
| src/.gitignore | 构建脚本、版本控制残留或平台资源 |
| src/A-mac-h.pch | 构建脚本、版本控制残留或平台资源 |
| src/ENGLISH.txt | 构建脚本、版本控制残留或平台资源 |
| src/angband.h | 纯声明清单，语义随对应 .c 分析 |
| src/angband.ico | 构建脚本、版本控制残留或平台资源 |
| src/angband.rc | 构建脚本、版本控制残留或平台资源 |
| src/carbon/Angband.icns | 平台资源（Mac Carbon 图标与清单） |
| src/carbon/Carbon.r | 平台资源（Mac Carbon 图标与清单） |
| src/carbon/Data.icns | 平台资源（Mac Carbon 图标与清单） |
| src/carbon/Edit.icns | 平台资源（Mac Carbon 图标与清单） |
| src/carbon/Image-DS_Store | 平台资源（Mac Carbon 图标与清单） |
| src/carbon/Info.plist | 平台资源（Mac Carbon 图标与清单） |
| src/carbon/Save.icns | 平台资源（Mac Carbon 图标与清单） |
| src/carbon/getversion | 平台资源（Mac Carbon 图标与清单） |
| src/cmovie.c | 与游戏机制无关的杂项：聊天、演示录制、帮助、笔记输出、GIF/位图解码 |
| src/config.h | 系统兼容层与构建开关，无游戏机制 |
| src/externs.h | 纯声明清单，语义随对应 .c 分析 |
| src/h-basic.h | 系统兼容层与构建开关，无游戏机制 |
| src/h-config.h | 系统兼容层与构建开关，无游戏机制 |
| src/h-define.h | 系统兼容层与构建开关，无游戏机制 |
| src/h-system.h | 系统兼容层与构建开关，无游戏机制 |
| src/h-type.h | 系统兼容层与构建开关，无游戏机制 |
| src/help.c | 与游戏机制无关的杂项：聊天、演示录制、帮助、笔记输出、GIF/位图解码 |
| src/irc.c | 与游戏机制无关的杂项：聊天、演示录制、帮助、笔记输出、GIF/位图解码 |
| src/iso/hackdef.h | 平台移植与平台前端主循环，机制不随平台 |
| src/iso/readme.txt | 平台移植与平台前端主循环，机制不随平台 |
| src/iso/simgraph.c | 平台移植与平台前端主循环，机制不随平台 |
| src/iso/simgraph.h | 平台移植与平台前端主循环，机制不随平台 |
| src/iso/simsys.h | 平台移植与平台前端主循环，机制不随平台 |
| src/iso/simview.c | 平台移植与平台前端主循环，机制不随平台 |
| src/iso/simview.h | 平台移植与平台前端主循环，机制不随平台 |
| src/iso/walls4.h | 平台移植与平台前端主循环，机制不随平台 |
| src/iso/walls9.h | 平台移植与平台前端主循环，机制不随平台 |
| src/iso/world_adaptor.c | 平台移植与平台前端主循环，机制不随平台 |
| src/iso/world_adaptor.h | 平台移植与平台前端主循环，机制不随平台 |
| src/iso/world_view.c | 平台移植与平台前端主循环，机制不随平台 |
| src/iso/world_view.h | 平台移植与平台前端主循环，机制不随平台 |
| src/lauxlib.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/load_gif.c | 与游戏机制无关的杂项：聊天、演示录制、帮助、笔记输出、GIF/位图解码 |
| src/lua/array.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/basic.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/class.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/clean.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/code.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/container.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/declaration.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/define.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/doit.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/enumerate.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/feature.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/function.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lapi.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lapi.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lauxlib.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lauxlib.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lbaselib.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lcode.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lcode.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/ldblib.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/ldebug.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/ldebug.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/ldo.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/ldo.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lfunc.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lfunc.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lgc.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lgc.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/liolib.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/llex.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/llex.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/llimits.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lmem.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lmem.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lobject.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lobject.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lopcodes.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lparser.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lparser.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lstate.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lstate.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lstring.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lstring.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lstrlib.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/ltable.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/ltable.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/ltests.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/ltm.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/ltm.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lua.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lua2c.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/luadebug.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lualib.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lundump.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lundump.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lvm.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lvm.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lzio.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/lzio.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/module.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/operator.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/package.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/print.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/tolua.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/tolua.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/tolua_bd.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/tolua_eh.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/tolua_eh.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/tolua_gp.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/tolua_lb.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/tolua_rg.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/tolua_rg.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/tolua_tm.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/tolua_tm.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/tolua_tt.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/tolua_tt.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/tolualua.c | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/tolualua.h | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/tolualua.pkg | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/typedef.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/variable.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/lua/verbatim.lua | tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制） |
| src/maid-x11.c | 平台移植与平台前端主循环，机制不随平台 |
| src/maim-iso.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-ami.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-cap.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-crb.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-dmy.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-dos.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-emx.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-gcu.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-gtk.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-gtk2.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-ibm.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-lsl.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-mac.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-net.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-ros.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-sdl-iso.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-sdl.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-sla.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-vme.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-win.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-x11.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-x11.c.orig | 平台移植与平台前端主循环，机制不随平台 |
| src/main-x11.c.rej | 平台移植与平台前端主循环，机制不随平台 |
| src/main-xaw.c | 平台移植与平台前端主循环，机制不随平台 |
| src/main-xxx.c | 平台移植与平台前端主循环，机制不随平台 |
| src/makefile.WHICH | 构建脚本、版本控制残留或平台资源 |
| src/makefile.ami | 构建脚本、版本控制残留或平台资源 |
| src/makefile.bcc | 构建脚本、版本控制残留或平台资源 |
| src/makefile.bsd | 构建脚本、版本控制残留或平台资源 |
| src/makefile.cyg | 构建脚本、版本控制残留或平台资源 |
| src/makefile.dos | 构建脚本、版本控制残留或平台资源 |
| src/makefile.emx | 构建脚本、版本控制残留或平台资源 |
| src/makefile.gdb | 构建脚本、版本控制残留或平台资源 |
| src/makefile.ibm | 构建脚本、版本控制残留或平台资源 |
| src/makefile.lsl | 构建脚本、版本控制残留或平台资源 |
| src/makefile.mingw | 构建脚本、版本控制残留或平台资源 |
| src/makefile.my | 构建脚本、版本控制残留或平台资源 |
| src/makefile.osx | 构建脚本、版本控制残留或平台资源 |
| src/makefile.ros | 构建脚本、版本控制残留或平台资源 |
| src/makefile.sdliso | 构建脚本、版本控制残留或平台资源 |
| src/makefile.std | 构建脚本、版本控制残留或平台资源 |
| src/makefile.wat | 构建脚本、版本控制残留或平台资源 |
| src/makefile.win | 构建脚本、版本控制残留或平台资源 |
| src/notes.c | 与游戏机制无关的杂项：聊天、演示录制、帮助、笔记输出、GIF/位图解码 |
| src/readdib.c | 与游戏机制无关的杂项：聊天、演示录制、帮助、笔记输出、GIF/位图解码 |
| src/readdib.h | 与游戏机制无关的杂项：聊天、演示录制、帮助、笔记输出、GIF/位图解码 |
| src/z-form.c | 通用支撑层（格式化/内存/套接字/报错），无游戏机制 |
| src/z-form.h | 通用支撑层（格式化/内存/套接字/报错），无游戏机制 |
| src/z-sock.c | 通用支撑层（格式化/内存/套接字/报错），无游戏机制 |
| src/z-sock.h | 通用支撑层（格式化/内存/套接字/报错），无游戏机制 |
| src/z-term.c | 字符终端抽象层；显示由 tome 的 Bevy 前端另行实现 |
| src/z-term.h | 字符终端抽象层；显示由 tome 的 Bevy 前端另行实现 |
| src/z-util.c | 通用支撑层（格式化/内存/套接字/报错），无游戏机制 |
| src/z-util.h | 通用支撑层（格式化/内存/套接字/报错），无游戏机制 |
| src/z-virt.c | 通用支撑层（格式化/内存/套接字/报错），无游戏机制 |
| src/z-virt.h | 通用支撑层（格式化/内存/套接字/报错），无游戏机制 |
| tome.ini | 发布说明与平台配置文件 |
