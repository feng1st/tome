//! Gesture resolvers: interpret gestures into core commands, using game
//! state. This is where interface policy lives — what a primary action on
//! a given target means. One resolver per gesture, mirroring `gestures/`
//! one-to-one; modalities stay semantics-free.

pub mod primary_action_on_cell;
