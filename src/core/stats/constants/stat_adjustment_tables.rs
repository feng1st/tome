/// ToME2, tables.c, adj_str_th
pub(crate) const STRENGTH_TO_HIT: [i32; 38] = [
    -3, -2, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9,
    10, 11, 12, 13, 14, 15, 15, 16,
];

/// ToME2, tables.c, adj_str_td
pub(crate) const STRENGTH_TO_DAMAGE: [i32; 38] = [
    -2, -2, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 3, 3, 3, 3, 3, 4, 5, 5, 6, 7, 8, 9, 10,
    11, 12, 13, 14, 15, 16, 18, 20,
];

/// ToME2, tables.c, adj_dex_th
pub(crate) const DEXTERITY_TO_HIT: [i32; 38] = [
    -3, -2, -2, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 6, 7, 8, 9, 9,
    10, 11, 12, 13, 14, 15, 15, 16,
];

/// ToME2, tables.c, adj_dex_ta
pub(crate) const DEXTERITY_TO_ARMOR_CLASS: [i32; 38] = [
    -4, -3, -2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 4, 5, 6, 7, 8, 9, 9,
    10, 11, 12, 13, 14, 15, 15, 16,
];

/// ToME2, tables.c, adj_con_mhp
pub(crate) const CONSTITUTION_TO_HALF_HP_PER_LEVEL: [i32; 38] = [
    -5, -3, -2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 3, 4, 4, 4, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
    14, 15, 16, 18, 20, 22, 25, 26, 27,
];
