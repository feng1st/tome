# player-ranged Specification

## Purpose

Player ranged combat: `src/cmd2.c` carries the three projectile flows - shooting
(do_cmd_fire), throwing (do_cmd_throw) and the boomerang (do_cmd_boomerang): the
projectile advances grid by grid in a straight line, rolls hits per monster, breaks ammo
and pierces, shatters thrown potions, and flies the boomerang back. The projectile
damage's slay/brand multipliers and special effects reuse the player-melee
tot_dam_aux/attack_special (same-file anchors); see specs/player-melee/spec.md. The
monster pain messages are specified with the melee2.c material.

## Requirements

### Requirement: Shooting

Shooting SHALL require a launcher in the bow slot (musical instruments are refused); the
ammo comes first from the quiver slots (when the tval does not match p_ptr->tval_ammo or
the slot is empty, the pack and the floor are scanned by tval instead); invuln and
disrupt_shield are broken by the attack action.

The hit bonus is the three layers player + ammo + bow to_h plus the ranged modifier; the
multiplier comes from the launcher sval (sling and short bow two, long bow and light
crossbow three, heavy crossbow four) plus the extra might tier; the range is ten plus five
times the multiplier; the energy cost is one hundred divided by the shot count.

The flight SHALL advance grid by grid and stop at a wall, the target grid, or the first
monster - the hit rolls by test_hit_fire (decaying per grid of range), the damage is the
dice roll plus double to_d times the multiplier plus the ranged modifier, then passes
through the slay/brand multipliers and the shooting critical table; kill messages switch
to "is destroyed" for demons, undead, stupid monsters and the special d_char set. When
the ammo's pval2 is non-zero (explosive ammo) it SHALL explode at the landing point by
the ammo tier (light tier radius two with half damage, normal tier radius three, heavy
tier radius four with double damage). Unbroken ammo that hits the target pierces (the
bow/crossbow/sling skill above 25 enables it, with a shot count of combat skill / 10
minus one): a 45%-plus-archery-skill chance pierces and keeps flying (until the count is
used up). After the hit, breakage_chance rolls for shattering; explosive ammo always
breaks; the landing uses `drop_near`.

- **Anchors**: `src/cmd2.c:3100-3565`

### Requirement: Throwing

Throwing SHALL pick any object from the pack or the floor (cursed equipment with
no-drop is refused); the strong thrower adds damage and range by the boulder skill; a
thrown wand spreads its charges across the thrown stack.

The range SHALL be the strength table plus twenty times the distance multiplier
(ten plus two times the throwing modifier plus two times the throwing-power modifier),
divided by the weight (floored at ten), capped at the multiplier; the damage is the dice
roll plus to_d plus the throwing-power bonus, times (throwing modifier plus
throwing-power modifier); the hit rolls by the throwing skill plus to_h with per-grid
range decay, criticals go through the shooting table (the boulder uses the boulder
skill). A potion SHALL shatter when it strikes a monster or a wall or when the breakage
roll succeeds, firing `potion_smash_effect` (with the anger-monsters ruling); otherwise
the breakage roll is zeroed and the potion survives. Other objects land by
breakage_chance after the hit.

- **Anchors**: `src/cmd2.c:3584-3972`

### Requirement: Boomerang

The boomerang SHALL take a TV_BOOMERANG from the bow slot: damage is the dice roll plus
to_d times the throwing modifier, the hit rolls by the throwing skill plus the ranged
modifier plus the boomerang skill, and the range follows the throwing formula. The round
trip SHALL first fly to the target or the wall; a non-artifact boomerang that hits may be
destroyed on the spot by its breakage rate, then flies back to the player grid by grid
(the path is for display only, no re-rolling). Special effects and criticals match
shooting.

- **Anchors**: `src/cmd2.c:3984-4311`

### Requirement: Breakage And Multiplier Tables

`breakage_chance` SHALL: potion bottles, lesser potions, oil flasks and food always
break; light sources, scrolls and skeletons half; wands and spikes a quarter; ammo by the
archery skill's folding (arrows half, shots and bolts a quarter, each divided by the
skill's reduction factor); the boomerang one percent; everything else ten percent.
`get_shooter_mult` SHALL set the multiplier by launcher sval (SV_SLING/SV_SHORT_BOW two,
SV_LONG_BOW/SV_LIGHT_XBOW three, SV_HEAVY_XBOW four).

- **Anchors**: `src/cmd2.c:2969-3020` (breakage), `:3025-3069` (multiplier)
