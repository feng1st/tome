# quest-ultra Specification

## Purpose

Ultra endings: the good branch `q_ultrag.c` (the permanent Melkor banishment line) and
the evil branch `q_ultrae.c` (an empty shell as shipped). Both quests are carried by the
quest table — two `Falling Toward Apotheosis` entries (good and evil, both danger level
150) in `quest_init_tome` under `src/tables.c`; the table mechanics are specified in
specs/quest/spec.md.

## Requirements

### Requirement: Good Ultra Ending

`quest_ultra_good_init_hook` SHALL hook by state — UNTAKEN hooks HOOK_MOVE, TAKEN
through FINISHED hooks HOOK_STAIR/HOOK_RECALL/HOOK_MONSTER_DEATH, and HOOK_CHAR_DUMP is
hooked unconditionally.

- the move hook SHALL, once QUEST_MORGOTH is FINISHED, trigger the long plot dialogue on
 the special=23 shop grid (Galadriel's mirror); a player who refuses to stay on Arda
 gets a FEAT_MORE down door five grids above (special=11), and the quest moves to TAKEN
 with a re-init;
- the stair hook SHALL apply to DUNGEON_VOID only — going up from depth 128 is refused
 (the portal to Arda is closed), going up from depth 150 is refused (the barrier is
 one-way), and going down from depth 149 requires an equipped item with TR4_ULTIMATE (the
 Flame Imperishable) or a magic barrier refuses entry;
- the recall hook SHALL refuse recall in VOID and NETHER_REALM unconditionally;
- the death hook SHALL: on killing race 1044 (Melkor) set total_winner/has_won to
 WINNER_ULTRA, mark the quest FINISHED, open a FEAT_MORE return door at the player's
 feet, remove HOOK_MONSTER_DEATH, and sever the plot line; on killing race 1032
 (Tik'svvrzllat) forge the Flame Imperishable (allowed through `k_allow_special[296]`,
 `apply_magic(-1,TRUE,TRUE,TRUE)`, full knowledge plus IDENT_MENTAL), dropping the last
 pack slot item first when the pack is full, then `inven_carry`;
- the dump hook SHALL append one of two epilogues to the character file — ultra victory,
 or the died-in-the-attempt line.

#### Scenario: Barrier at the last depth

- **WHEN** the player on VOID depth 149 tries to go down without any equipped item
 carrying `TR4_ULTIMATE`
- **THEN** the stair hook blocks the move with the impassable-magic-barrier message;
 with such an item equipped the barrier shatters and the descent proceeds

- **Anchors**: `src/q_ultrag.c:8-78` (move), `:80-141` (stair), `:143-150` (recall), `:152-237` (death), `:238-259` (dump), `:262-276` (init)

### Requirement: Evil Ultra As Shipped

`quest_ultra_evil_init_hook` SHALL always return FALSE — the evil ultra ending is not
implemented as shipped; only the file placeholder and the `cquest` macro redirection
exist.

#### Scenario: Init hook invoked

- **WHEN** the quest table invokes `quest_ultra_evil_init_hook`
- **THEN** it returns FALSE and no hooks are installed

- **Anchors**: `src/q_ultrae.c:1-11`
