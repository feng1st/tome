/// ToME2, xtra1.c, calc_bonuses, p_ptr->stat_ind
pub fn stat_index(value: i32) -> usize {
    if value <= 3 {
        0
    } else if value <= 18 {
        (value - 3) as usize
    } else if value <= 18 + 219 {
        (15 + (value - 18) / 10) as usize
    } else {
        37
    }
}
