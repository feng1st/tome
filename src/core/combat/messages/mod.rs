//! Past-tense combat facts: what already happened, as opposed to the
//! requests that make things happen. A request is a noun (`Damage`); a
//! fact is past tense — the naming convention keeps the two channels
//! apart at a glance. Facts are broadcasts: any number of readers may
//! consume them, and reading one changes nothing.

pub mod attack_resolved;
