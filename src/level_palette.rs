use raylib::prelude::Color;

#[allow(dead_code)]
pub const PALLETE: [u32; 11] = [
    0x000, 0xfcfcfc, 0xfc7460, 0x3cbcfc, 0x80d10, 0xd8280, 0x70ec, 0xfc74b4, 0xfc9838, 0xbcbcbc,
    0xf0bc3c,
];

pub const RL_COLOR_PALETTE: [Color; 11] = [
    Color {
        r: 0x0,
        g: 0x0,
        b: 0x0,
        a: 255,
    },
    Color {
        r: 0xfc,
        g: 0xfc,
        b: 0xfc,
        a: 255,
    },
    Color {
        r: 0xfc,
        g: 0x74,
        b: 0x60,
        a: 255,
    },
    Color {
        r: 0x3c,
        g: 0xbc,
        b: 0xfc,
        a: 255,
    },
    Color {
        r: 0x80,
        g: 0xd0,
        b: 0x10,
        a: 255,
    },
    Color {
        r: 0xd8,
        g: 0x28,
        b: 0x0,
        a: 255,
    },
    Color {
        r: 0x0,
        g: 0x70,
        b: 0xec,
        a: 255,
    },
    Color {
        r: 0xfc,
        g: 0x74,
        b: 0xb4,
        a: 255,
    },
    Color {
        r: 0xfc,
        g: 0x98,
        b: 0x38,
        a: 255,
    },
    Color {
        r: 0xbc,
        g: 0xbc,
        b: 0xbc,
        a: 255,
    },
    Color {
        r: 0xf0,
        g: 0xbc,
        b: 0x3c,
        a: 255,
    },
];
