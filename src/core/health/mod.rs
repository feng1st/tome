//! Hit points: the shared injury ledger of every woundable creature.
//! The ceiling's source differs per side — players derive it from
//! constitution at birth, monsters from their kind's hit dice — but the
//! component and its meaning are one.

pub mod components;
pub mod constants;
pub mod utils;
