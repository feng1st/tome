# god Specification

## Purpose

The faith system: establishing and abandoning a deity, the increase, decrease and
overflow clamping of piety (`grace`), the anti-magic skill's ban on worshipping,
the faith hooks and the Melkor special case (unlocking the Udun skill), and the
religion information display with its wisdom scaling. The deity data structures are
initialized at boot time from the Lua side (`reinit_gods` plus the Lua definitions;
see specs/boot-loading/spec.md). Prayer and god-spell casting run through the
school-magic engine; see specs/school-magic/spec.md.

## Requirements

### Requirement: Establishing Faith

A character SHALL follow at most one deity: faith can be established while
unaffiliated, and a second conversion while already faithful MUST have no effect.
Establishing faith SHALL first consult the faith hook (a hook veto rejects the
conversion), and after establishment SHALL fire the faith-completed hook.
Converting to Melkor SHALL unhide the Udun skill and report it; with a non-zero
anti-magic skill the conversion MUST be refused with a no-faith message.

#### Scenario: Second conversion while faithful

- **WHEN** a character who already has a deity calls the conversion routine
- **THEN** the current deity is unchanged and the Melkor special case and the
 faith-completed hook do not fire (the faith hook is still consulted, and its
 veto still rejects)

#### Scenario: Anti-magic ban

- **WHEN** a character with a non-zero anti-magic skill level attempts to convert
- **THEN** the conversion is refused and a message prints

- **Anchors**: `src/gods.c:49-76`

### Requirement: Piety Increase And Decrease

Piety SHALL only apply to the current deity (or to `GOD_ALL` as the all-deities
target): both directions go through one shared entry point. After a positive
increment the piety value MUST NOT be lower than before (a violation sets it to the
large positive 300000), and after a negative increment it MUST NOT be higher than
before (a violation sets it to the large negative -300000) - this overflow detection
blocks out-of-range piety writes from upstream callers.

#### Scenario: Overflow clamp

- **WHEN** a positive piety increment leaves the value lower instead of higher
- **THEN** piety is set to 300000 instead of the out-of-range result

- **Anchors**: `src/gods.c:18-32`

### Requirement: Abandoning Faith

A character SHALL be able to abandon the current deity (or the all-deities target):
after abandonment the faith is cleared and piety reset to zero; abandoning without a
deity MUST have no effect.

#### Scenario: Abandonment takes effect

- **WHEN** a faithful character abandons the deity
- **THEN** the faith is set to no-god and piety returns to zero

- **Anchors**: `src/gods.c:37-44`

### Requirement: Religion Information Display

Displaying religion information SHALL print the no-faith hint and return failure
when `pgod < 0`; otherwise it SHALL show the deity name and up to ten description
lines, saving and restoring the terminal screen and setting the interaction lock
around the display.

- **Discrepancy:** `GOD_NONE` is 0 (`src/defines.h:4665`) and every setter
 assigns 0 for the faithless state (`src/birth.c:3408`, `src/gods.c:41`), so the
 `pgod < 0` hint branch never fires in normal play - a faithless character falls
 through to the deity display path instead.

#### Scenario: Display without faith

- **WHEN** a faithless character (pgod 0) views the religion information
- **THEN** the `pgod < 0` hint branch does not trigger and the display falls
 through to the `deity_info[pgod]` path

- **Anchors**: `src/gods.c:81-117`, `src/defines.h:4664-4666`

### Requirement: Wisdom Scaling And Deity Lookup

God-spell related scaling SHALL take the current wisdom adjustment value and scale
it by 37ths up to a given cap (`(i * max) / 37` on `stat_ind[A_WIS]`); looking up a
deity by name SHALL walk the deity table with a case-sensitive exact full-name
comparison and return -1 when nothing matches.

#### Scenario: Lookup of an unknown name

- **WHEN** a deity is looked up by a name that does not exist
- **THEN** the lookup returns -1

- **Anchors**: `src/gods.c:122-140`
