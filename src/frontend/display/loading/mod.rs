//! The loading mechanism: a readiness barrier for asset loads. Knows no
//! business domain — domains acquire guards via `LoadBuilder::with_guard`
//! at load issuance and gate their work on the `assets_ready` condition.
//!
//! Readiness is a property, not a mode: it lives in the barrier, not in a
//! state. Any side may sign loads into the same barrier at any time (menu
//! assets at startup, map assets on entering `Game`, a new map mid-game
//! via message); the gate closes when a guard is minted and opens when the
//! last one drops.

pub mod resources;

use bevy::prelude::*;

use resources::asset_barrier::AssetBarrier;

/// Run condition: every guarded load on the barrier has finished.
pub fn assets_ready(barrier: Res<AssetBarrier>) -> bool {
    barrier.is_ready()
}

/// Insert the barrier. The initial guard drops here, so the barrier counts
/// only guards minted by domains afterwards.
pub fn register(app: &mut App) {
    let (barrier, guard) = AssetBarrier::new();
    drop(guard);
    app.insert_resource(barrier);
}
