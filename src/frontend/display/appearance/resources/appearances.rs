//! The appearance registry: every `AppearanceKind` resolved to its look.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::core::appearance::components::appearance_kind::AppearanceKind;
use crate::frontend::display::appearance::types::appearance::Appearance;

/// Registry of all appearances, built once at load time from the
/// code-defined appearance data (data files later). Instances never own
/// this data — they carry only the `AppearanceKind` key and cloned sprite
/// handles; systems look appearances up through this resource.
#[derive(Resource)]
pub struct Appearances(pub HashMap<AppearanceKind, Appearance>);

impl Appearances {
    /// The appearance registered for `kind`. Missing keys are a build-time
    /// bug (the registry forgot an `AppearanceKind`), not a runtime case.
    pub fn appearance(&self, kind: AppearanceKind) -> &Appearance {
        self.0
            .get(&kind)
            .expect("every AppearanceKind has a registered appearance")
    }
}
