//! Readiness barrier for in-flight asset loads. Ported from Bevy's
//! `examples/asset/multi_asset_sync.rs`.
//!
//! Deviations from the official example (deliberate, by design):
//! 1. Async waiting is omitted (`notify: Event`, `wait_async()`): we only
//!    poll synchronously (`is_ready` in the `assets_ready` run condition), avoiding the
//!    `event_listener`/`futures_lite` dependencies the example needs for
//!    its async showcase.
//! 2. `AssetBarrier::guard()` is added: the official example issues every
//!    load from one function and clones a local guard; our domains issue
//!    loads independently (self-registration), so guards are minted from
//!    the barrier on demand. Counting semantics are unchanged (+1 on
//!    clone/mint, -1 on drop).
//! 3. The loading-phase placeholder content (`Loading`/`LoadingText`
//!    marker entities despawned `OnExit`) is not adopted: our loads take a
//!    few frames and need no loading screen yet; add it per the official
//!    pattern when a real loading phase appears.

use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

use bevy::prelude::*;

/// Counts in-flight loads; ready when all guards are dropped.
#[derive(Debug, Resource, Deref)]
pub struct AssetBarrier(Arc<AssetBarrierInner>);

/// Acquired per load via `LoadBuilder::with_guard`; the loader holds it
/// until that load finishes, then drops it, decrementing the barrier.
#[derive(Debug, Deref)]
pub struct AssetBarrierGuard(Arc<AssetBarrierInner>);

/// Tracks how many guards are remaining.
#[derive(Debug)]
pub struct AssetBarrierInner {
    count: AtomicU32,
}

impl AssetBarrier {
    pub fn new() -> (AssetBarrier, AssetBarrierGuard) {
        let inner = Arc::new(AssetBarrierInner {
            count: AtomicU32::new(1),
        });
        (AssetBarrier(inner.clone()), AssetBarrierGuard(inner))
    }

    /// Mint a fresh guard for one load issuance. Increments the count —
    /// the official example clones its local guard instead, which
    /// increments via `Clone`; minting must match that semantics.
    pub fn guard(&self) -> AssetBarrierGuard {
        self.count.fetch_add(1, Ordering::AcqRel);
        AssetBarrierGuard(self.0.clone())
    }

    /// Returns true if all [`AssetBarrierGuard`]s are dropped.
    pub fn is_ready(&self) -> bool {
        self.count.load(Ordering::Acquire) == 0
    }
}

// Increment count on clone.
impl Clone for AssetBarrierGuard {
    fn clone(&self) -> Self {
        self.count.fetch_add(1, Ordering::AcqRel);
        AssetBarrierGuard(self.0.clone())
    }
}

// Decrement count on drop.
impl Drop for AssetBarrierGuard {
    fn drop(&mut self) {
        self.count.fetch_sub(1, Ordering::AcqRel);
    }
}
