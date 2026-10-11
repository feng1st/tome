use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::common::by_value::ByValue;

#[allow(dead_code)]
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimKind {
    Idle,
    Run,
    Attack,
    Hit,
    Die,
}

impl ByValue for AnimKind {}
