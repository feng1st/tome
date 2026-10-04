//! Action duration constants: the pricing basis of the world clock.
//! Speed resolves to a rate through this domain's table; this constant
//! is the clock-side half of pricing — the standard action's length at
//! standard speed.

/// Ticks a standard action (one step, one attack) takes at standard
/// speed. Every action kind is priced relative to this base, and speed
/// scales the price — never the other way round.
pub const STANDARD_ACTION_DURATION: u32 = 100;
