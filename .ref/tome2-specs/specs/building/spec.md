# building Specification

## Purpose

Building runtime: the interaction loop after the player steps onto a building grid — the
six actions with their costs and restrictions across the three standing tiers (liked /
normal / hated), the action execution protocol (charging, loan restrictions, delegation
to the default Lua hook), and the semantics of the built-in actions (quest taking, the
arena, the four casino games, the inn, weapon comparison, repair, the bounty corpse
trade, loans, and the rest). The action word-list contract is specified in
specs/building-action/spec.md; the four store trade actions are specified in
specs/store-runtime/spec.md.

## Requirements

### Requirement: Standing Tiers

The relation between the player and a store owner SHALL have three tiers: a hit on the
owner word-list entry's liked or hated race/class list selects the matching tier, and a
miss on both is the normal tier. The normal tier SHALL give way passively to an explicit
liked or hated verdict.

#### Scenario: Both lists miss

- **WHEN** the player's race and class are on neither the owner's liked nor the hated
 list
- **THEN** the standing is the normal tier

- **Anchors**: `src/bldg.c:28-66`

### Requirement: Building Screen

The building interaction SHALL present six action slots: each action shows its menu
letter and name, with the cost taken from the tier matching the current standing — zero
cost shows no tag, liked prices are green, hated prices are red, normal prices are
yellow; restriction tier one shows "closed" for hated players, restriction tier two
shows "closed" for every non-liked player. Either the command letter or the auxiliary
letter triggers the action; Esc leaves; on leaving the runtime SHALL regenerate the
wilderness to activate quests and restore the player coordinates.

#### Scenario: Restriction closes an action

- **WHEN** a player who is not liked views an action of restriction tier two
- **THEN** the action shows "closed" and cannot be run

- **Anchors**: `src/bldg.c:87-183` (presentation), `src/bldg.c:2158-2271` (interaction loop and leave-rebuild)

### Requirement: Action Execution Protocol

Action execution SHALL validate in order: the restriction tier (tier one refuses liked
players, tier two refuses non-liked players); while a loan is overdue only selling,
corpse selling, quest-monster corpse selling, bounty viewing, quest-monster viewing,
examining, stealing and loan repayment are allowed; insufficient gold is refused. When
execution reports that it took effect, the tier cost is charged. An unknown action SHALL
be delegated to the building-action hook (run through Lua, returning the took-effect and
rebuild bits).

#### Scenario: Loan restriction

- **WHEN** a player carrying an overdue loan tries to rest at the inn
- **THEN** the action is refused with a pay-back-first message

**Quirk:** the screen and the execution read restriction tier one in opposite ways —
`show_building` offers tier-one actions to liked and normal players (only the hated see
"closed"), while `bldg_process_command` refuses tier-one actions to liked players; only a
normal-tier player can actually run one.

- **Anchors**: `src/bldg.c:1639-1724` (validation chain), `:2105-2124` (hook delegation and charging)

### Requirement: Quest Taking

The four quest actions SHALL locate the town's quest token grid by action index and take
the quest through its bound plot line: with no plot line the player is told none is
offered; TAKEN-but-unfinished draws a reminder; COMPLETED moves to FINISHED (rewarded)
and fires the finish hook; FAILED moves to FAILED_DONE and fires the fail hook; UNTAKEN
passes the quest-init hook as gatekeeper (a refusal cancels the taking), and on a pass
the quest is taken, its information shown, and C-type quests get their plot hooks
loaded.

#### Scenario: Reward handout

- **WHEN** the player reports back with a COMPLETED quest
- **THEN** the quest moves to FINISHED (rewarded) and the finish hook fires

- **Anchors**: `src/bldg.c:806-896` (state machine), `:1731-1765` (token location)

### Requirement: Arena

The arena SHALL track the win streak: below the full bout count, entering starts an
arena bout (arena mode on, all timed states reset, the return point recorded); at the
full bout count the prize of ten thousand gold is paid and the coronation state begins;
beyond that, entering is only a victory lap. The poster action SHALL announce the
current or next opponent; the rules action shows the help text.

#### Scenario: Clean sweep payout

- **WHEN** the player enters the arena again after the last win
- **THEN** ten thousand gold is paid and the coronation state begins

- **Anchors**: `src/bldg.c:223-300`

### Requirement: Casino Games

The casino SHALL offer four games plus the rules viewer: In-Between (two black dice and
one red die, the red die strictly between the blacks wins, odds three), Craps (a first
roll of seven or eleven wins, two/three/twelve loses, otherwise the point is re-rolled
until it repeats as a win or a seven loses), the wheel (pick 0-9, odds ten) and Dice
Slots (three of a kind pays by the fruit — first fruit odds four, second odds six,
otherwise the pip value squared; two sixes plus any third pays that pip plus one). The
bet cap SHALL be one hundred gold per level (one thousand per level from level ten); a
bet above the cap is trimmed down to the cap, a bet under one gold is topped up to one,
and betting more gold than held gets the player thrown out. Wins can be re-wagered, and
stopping settles with a profit-or-loss announcement.

#### Scenario: In-Between win

- **WHEN** the red die lands strictly between the two black dice
- **THEN** the payout is three times the wager

- **Anchors**: `src/bldg.c:406-655` (bet clamping `:437-470`, the four games `:485-611`)

### Requirement: Inn

The inn SHALL offer three actions: a meal (fills food to the maximum; a vampire race is
offered none); a night's rest (ordinary races may book it only at night, vampires only
by day; the poisoned or bleeding are refused; time advances in ten-minute steps to
sunrise or sunset, HP and mana are fully restored, blindness, confusion and stun are
cleared, the building is left, and the bounty list is refreshed); and rumors (one random
line from the rumor text).

#### Scenario: Vampire day sleep

- **WHEN** a vampire character tries to rest at night
- **THEN** the request is refused — the room is offered by daylight only

- **Anchors**: `src/bldg.c:667-782`

### Requirement: Repair And Comparison

The repair action SHALL improve items one step at a time up to a cap of one fifth of the
player level: weapons are sharpened (to-hit and to-dam each +1 up to the cap), armor is
polished (armor +1 up to the cap); a negative value reaching minus three is judged
scrap; an artifact whose abilities are known is judged beyond the craft, while an
artifact not yet known reads as being in fine condition. Weapon comparison SHALL pick
two melee weapons in turn, pretend-equip each to show the single-strike and full-round
damage ranges plus every slay/brand multiplier (animals and evil double; undead, demon,
orc, giant, troll and dragon triple; kill-dragon quintuple; the acid, lightning, fire,
cold and poison brands triple), then restore the original equipment.

#### Scenario: Artifact refused for repair

- **WHEN** an identified artifact is sent for repair
- **THEN** it is judged "beyond our skills" and left unchanged

- **Anchors**: `src/bldg.c:1142-1244` (repair), `:917-1134` (comparison)

### Requirement: Bounty Trade

Bounties SHALL come in two kinds: several standing bounties (level rises by five per
list index plus noise; the bounty value is the monster's exp plus twenty per level plus
noise; the generation hook excludes uniques, corpse-less races, pets and friendly
races), each paying for one monster's corpse; and the quest-monster bounty (entry zero)
drawn by player level, with the required kill count rolled at mean five standard deviation three
(minimum two), raised by multipliers for group spawners and breeders and halved for
aquatic races — in the shipped code the halving after the group branch runs
unconditionally because of the missing braces, a quirk recorded as-is. Selling a corpse
SHALL pay by list match (head/skull corpses add one times the race level) and count
toward the total bounty tally; delivering quest-monster corpses SHALL decrement the
required count, and at zero grants full knowledge of that monster and draws a fresh quest
monster. An inn overnight SHALL reset the whole bounty list.

#### Scenario: Quest monster revealed

- **WHEN** the required quest-monster corpse count is handed in full
- **THEN** that monster's memory insight is maxed out and a new quest monster is issued

**Quirk:** the duplicate-rejection loop in the bounty list fill never redraws (the inner
skip cannot reach a redraw), so duplicate list entries remain possible.

- **Anchors**: `src/bldg.c:1439-1477` (quest-monster draw, missing-braces quirk at `:1466`), `:1582-1634` (list refresh), `:1362-1409` (corpse selling), `:1484-1575` (delivery and full knowledge), `:1260-1334` (viewing and filters)

### Requirement: Service And Miscellaneous Actions

Buildings SHALL also offer: full identification, research item (fully identifies one
item), research monster, recharge (an eighty-point draw), star-heal and healing (restore
200 HP and clear poison, blindness, confusion, bleeding and stun; star-heal also breaks
the Black Breath), restoration of all six stats, word of recall, targeted recall (resets
the recall target level), removing a mimic form, divination (reveals one unknown fate
within a thousand draws, the money kept when there is no fate), loans (the credit line
is cash plus pack value capped at one hundred thousand and additionally clamped so total
gold stays within `PY_MAX_GOLD`, and the overdue timer accumulates with the borrowed
amount) and loan repayment (paying it off clears the overdue timer).
The quest level entrance SHALL read the quest index from the token grid's special and
enter at dungeon level one.

#### Scenario: Loan cap

- **WHEN** the player's assets are valued far above one hundred thousand
- **THEN** the credit line is clamped to one hundred thousand

**Quirk:** the repayment guard is inverted as shipped — a player with an outstanding
loan is told "You have nothing to payback!" and turned away, while the body (which
clears `loan_time` when the loan reaches zero) only runs when no loan exists.

- **Anchors**: `src/bldg.c:1704-2103` (action dispatch), `:2035-2103` (loans and repayment), `:2131-2152` (quest level entrance)
