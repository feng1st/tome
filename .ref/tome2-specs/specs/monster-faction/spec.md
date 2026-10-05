# monster-faction Specification

## Purpose

Monster factions and control: the three stances a monster can hold toward the player
(hostile / neutral / friendly, with their subdivisions), monster-vs-monster hostility,
stance reversals (including the Yavanna piety penalty), the breeding AI's crowding
avoidance, a possessor's possessing and leaving the host body (body swap plus an hp
re-roll), the companion count cap, promoting pets to companions, and the player
interface for steering a controlled monster's walking, carrying, and casting. The
actual breeding generation is specified in specs/monster-generation/spec.md; the
granting of control runs through the charm spells (see specs/spell-casting/spec.md).

## Requirements

### Requirement: Stance Classification

Every monster SHALL carry a stance with seven states: enemy, neutral, the friendly
family (friendly neutral, friend, pet, companion), and the hostile family (hostile
neutral, enemy). The friendly family classifies as friendly (1), the hostile family
as hostile (-1), and pure neutral as 0.

#### Scenario: Stance lookup

- **WHEN** the stance of a pet and of an enemy monster is queried
- **THEN** the pet classifies as friendly (1) and the enemy as hostile (-1)

- **Anchors**: `src/monster3.c:19-41`

### Requirement: Monster Versus Monster Hostility

Two monsters SHALL be hostile to each other if and only if: one side is non-neutral
and the other has the breeding ability while the global breeding count exceeds two
thirds of the cap and the two symbols differ; or the two stances are one hostile and
one friendly. In all other cases (both neutral included) they are not hostile.

#### Scenario: Punishing breeders

- **WHEN** the breeding count exceeds two thirds of the cap, one monster is
 non-neutral, the other has the breeding flag, and their symbols differ
- **THEN** the two monsters treat each other as enemies

- **Anchors**: `src/monster3.c:44-61`

### Requirement: Stance Reversal

Stances SHALL be reversible: when a friend or a pet is turned to enemy, a monster
that is an animal and not evil SHALL cost the player Yavanna piety at 4 points per
level; the friendly neutral and the hostile neutral convert into each other;
companions SHALL refuse to reverse.

#### Scenario: Pet rebellion

- **WHEN** a non-evil animal pet has its stance reversed
- **THEN** it becomes an enemy, and the player's Yavanna piety drops by four times
 its level

- **Anchors**: `src/monster3.c:63-97`

### Requirement: Breeding AI

A breeding monster acting SHALL breed according to crowding: it counts the monsters
in the adjacent eight grids including itself, and only below four neighbors and after
passing a crowding-weighted random check does it attempt to breed (a friendly stance
is passed to the offspring); success consumes the turn's energy, and when visible the
breeding flag is memorized into the monster recall.

#### Scenario: Crowding avoidance

- **WHEN** a breeding monster has four or more neighbors
- **THEN** it does not attempt to breed this turn

- **Anchors**: `src/monster3.c:100-142`

### Requirement: Possession And Leaving The Host

A possessor SHALL be able to possess a target race (the source object's `pval2` names
the target race): the body is replaced, stun / confusion / fear / target / sleep are
cleared, full hp is rolled from the target race's hit dice (the force-maxhp flag
takes the maximum value instead), the four blow groups plus armor / level / speed are
copied, experience is converted by the new level, the on-level race totals transfer
between the old and new races (and the global breeding count rises when the new race
breeds), and the original race is
recorded. Leaving the host SHALL restore the recorded original race and clear the
possession marker. When visible, the possession and leaving SHALL be announced.

#### Scenario: Force-maxhp race

- **WHEN** the target race has the force-maxhp flag
- **THEN** after possession the monster's hit points take the hit dice maximum
 instead of a roll

- **Anchors**: `src/monster3.c:145-223` (possession), `src/monster3.c:225-296`
 (leaving)

### Requirement: Companion Cap And Promotion

New companions SHALL be subject to a count cap: allowed while the global companion
count is below `1 +` the Lore skill scale (the skill is scaled with 6 as the
ceiling). Promotion SHALL target a pet: a pet's state is promoted to the companion
state and confirmed; a non-pet target is rejected, and exceeding the cap is rejected.

#### Scenario: Promoting a non-pet

- **WHEN** the player uses promotion on a non-pet monster
- **THEN** the game reports that the target is not the player's pet and its stance
 is unchanged

- **Anchors**: `src/monster3.c:298-316` (cap), `src/monster3.c:643-679` (promotion)

### Requirement: Mind Control Interface

The player controlling a monster SHALL get five operations: walking (repeated
direction input, costing 100 energy and recording the direction), inspecting the
carried objects, picking up objects underfoot (gold is skipped; objects hook onto the
monster's carry pile), dropping everything carried, and casting (list selection from
the race's `RF4`/`RF5`/`RF6` monster power table with letter and number labels;
uppercase requires confirmation; the chosen spell costs 100 energy and is cast
through the monster spell channel). The casting entry also supports permanently
abandoning the body (deleting the monster) or releasing control. Reconnecting SHALL
find the controlled monster again by its control flag.

#### Scenario: Pickup skips gold

- **WHEN** a controlled monster stands on gold and on an object
- **THEN** only the non-gold object enters the monster's carry pile

- **Anchors**: `src/monster3.c:320-339` (walking), `src/monster3.c:341-352`
 (inspecting), `src/monster3.c:354-399` (picking up), `src/monster3.c:401-408`
 (dropping), `src/monster3.c:410-616` (casting and abandoning),
 `src/monster3.c:619-638` (reconnect)
