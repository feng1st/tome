use crate::core::speed::constants::speed::SPEED_RATE_TABLE;
use crate::core::speed::constants::speed::STANDARD_SPEED_RATE;

pub fn action_duration(base: i32, speed: i32) -> i64 {
    let speed_rate = SPEED_RATE_TABLE[speed as usize];
    ((base * STANDARD_SPEED_RATE + speed_rate / 2) / speed_rate) as i64
}
