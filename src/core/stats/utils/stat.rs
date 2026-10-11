/// ToME2, xtra1.c, modify_stat_value
pub fn adjust_stat(value: i32, amount: i32) -> i32 {
    let mut value = value;
    if amount > 0 {
        for _ in 0..amount {
            if value < 18 {
                value += 1;
            } else {
                value += 10;
            }
        }
    } else if amount < 0 {
        for _ in 0..(-amount) {
            if value >= 28 {
                value -= 10;
            } else if value > 18 {
                value = 18;
            } else if value > 3 {
                value -= 1;
            }
        }
    }
    value
}
