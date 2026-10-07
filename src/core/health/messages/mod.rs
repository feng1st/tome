//! Damage channel messages: the hurt request and the past-tense fact
//! the application point broadcasts. A request is a noun (`Damage`); a
//! fact is past tense — reading a fact changes nothing, and any number
//! of readers may consume it.

pub mod damage;
pub mod damage_applied;
