//! The appearance key: which look a creature wears.

use bevy::prelude::*;

/// Which appearance a creature wears — the core→display protocol key.
/// Variants name *looks*, not creature kinds: the hero's appearance
/// derives from class and armor tier, a wolf's from its species, so one
/// creature kind may wear several appearances over its lifetime (donning
/// armor rewrites this component) and several kinds may share one look.
/// The display side resolves it through its `Appearances` registry;
/// `Changed<AppearanceKind>` detection re-attaches the look for free.
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AppearanceKind {
    /// The unarmored warrior (hero tier 0).
    Warrior,
}
