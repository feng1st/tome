//! The combat domain: the attack pipeline's numbers — the blows and
//! armor-class components and their player-side derivation — plus the
//! composite formulas built on the combat-bonus component and the stat
//! bonus tables. The attack action itself and its executor are the
//! action domain's; this domain supplies the numbers they spend.

pub mod components;
pub mod constants;
pub mod messages;
pub mod systems;
pub mod types;
pub mod utils;

use bevy::prelude::*;

use self::messages::attacked::Attacked;
use crate::core::core_phase::CorePhase;

/// Register the combat domain: the attacked fact message exists
/// (emitted by the action domain's executor), and the derive chain
/// keeps the bonus and attack components fresh in the Derive phase,
/// before any turn logic reads them.
pub fn register(app: &mut App) {
    app.add_message::<Attacked>().add_systems(
        Update,
        ((
            systems::derive_combat_bonuses::derive_combat_bonuses,
            systems::derive_attack::derive_attack,
        )
            .chain()
            .in_set(CorePhase::Derive),),
    );
}
