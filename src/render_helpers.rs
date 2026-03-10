use glam::Vec2;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibTextureMode};

use crate::components::PowerUpType;

pub fn snap_rect(pos: Vec2, dims: Vec2) -> (i32, i32, i32, i32) {
    let left = pos.x.round() as i32;
    let top = pos.y.round() as i32;
    let width = dims.x.round().max(1.0) as i32;
    let height = dims.y.round().max(1.0) as i32;
    (left, top, width, height)
}

pub fn draw_rect_outline(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    pos: Vec2,
    dims: Vec2,
    color: Color,
) {
    let (left, top, width, height) = snap_rect(pos, dims);
    let right = left + width - 1;
    let bottom = top + height - 1;

    d.draw_rectangle(left, top, width, 1, color);
    d.draw_rectangle(left, bottom, width, 1, color);
    d.draw_rectangle(left, top, 1, height, color);
    d.draw_rectangle(right, top, 1, height, color);
}

pub fn powerup_color(power_up_type: PowerUpType) -> Color {
    match power_up_type {
        PowerUpType::Enlarge => Color::GREEN,
        PowerUpType::Shrink => Color::PINK,
        PowerUpType::SpeedUp => Color::ORANGE,
        PowerUpType::SlowDown => Color::SKYBLUE,
        PowerUpType::BallSplit => Color::YELLOW,
        PowerUpType::Catch => Color::LIME,
        PowerUpType::ExtraLife => Color::GOLD,
        PowerUpType::Lasers => Color::RED,
        PowerUpType::BombBall => Color::MAGENTA,
    }
}

pub fn draw_powerup_symbol(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    pos: Vec2,
    power_up_type: PowerUpType,
    color: Color,
) {
    let x = pos.x.round() as i32;
    let y = pos.y.round() as i32;
    match power_up_type {
        PowerUpType::Enlarge => {
            d.draw_line(x + 1, y + 4, x + 6, y + 4, color);
            d.draw_line(x + 1, y + 4, x + 2, y + 3, color);
            d.draw_line(x + 1, y + 4, x + 2, y + 5, color);
            d.draw_line(x + 6, y + 4, x + 5, y + 3, color);
            d.draw_line(x + 6, y + 4, x + 5, y + 5, color);
        }
        PowerUpType::Shrink => {
            d.draw_line(x + 1, y + 4, x + 6, y + 4, color);
            d.draw_line(x + 2, y + 3, x + 3, y + 4, color);
            d.draw_line(x + 2, y + 5, x + 3, y + 4, color);
            d.draw_line(x + 5, y + 3, x + 4, y + 4, color);
            d.draw_line(x + 5, y + 5, x + 4, y + 4, color);
        }
        PowerUpType::SpeedUp => {
            d.draw_text(">>", x, y + 1, 6, color);
        }
        PowerUpType::SlowDown => {
            d.draw_text("<<", x, y + 1, 6, color);
        }
        PowerUpType::BallSplit => {
            d.draw_circle(x + 2, y + 2, 1.0, color);
            d.draw_circle(x + 5, y + 2, 1.0, color);
            d.draw_circle(x + 4, y + 5, 1.0, color);
        }
        PowerUpType::Catch => {
            d.draw_line(x + 2, y + 1, x + 2, y + 6, color);
            d.draw_line(x + 5, y + 1, x + 5, y + 6, color);
            d.draw_line(x + 2, y + 6, x + 5, y + 6, color);
        }
        PowerUpType::ExtraLife => {
            d.draw_line(x + 4, y + 1, x + 4, y + 6, color);
            d.draw_line(x + 2, y + 3, x + 6, y + 3, color);
        }
        PowerUpType::Lasers => {
            d.draw_rectangle(x + 2, y + 1, 1, 6, color);
            d.draw_rectangle(x + 5, y + 1, 1, 6, color);
        }
        PowerUpType::BombBall => {
            d.draw_circle_lines(x + 3, y + 4, 2.0, color);
            d.draw_line(x + 5, y + 2, x + 6, y + 1, color);
            d.draw_pixel(x + 6, y + 1, Color::YELLOW);
        }
    }
}
