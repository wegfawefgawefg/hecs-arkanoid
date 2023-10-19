use glam::Vec2;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibTextureMode};

use crate::{
    render_commands::execute_render_command_buffer,
    state::{GameMode, State},
    DIMS,
};

pub fn draw(state: &State, low_res_draw_handle: &mut RaylibTextureMode<RaylibDrawHandle>) {
    match state.game_mode {
        GameMode::Title => {
            title_render(state, low_res_draw_handle);
        }
        GameMode::Playing => {
            playing_render(state, low_res_draw_handle);
        }
        GameMode::GameOver => {
            game_over_render(state, low_res_draw_handle);
        }
    }
}

////////////////////////    PER GAME MODE DRAW FUNCTIONS     ////////////////////////
pub fn title_render(_state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    let mut cursor = Vec2::new(DIMS.x as f32 * 0.15, DIMS.y as f32 * 0.4);
    let title = "HECS-arkanoid!";
    let size = 20;
    d.draw_text(title, cursor.x as i32, cursor.y as i32, size, Color::WHITE);
    cursor.y += size as f32 * 1.5;

    let subtitle = "press space to start";
    let size = 1;
    d.draw_text(
        subtitle,
        cursor.x as i32,
        cursor.y as i32,
        size,
        Color::WHITE,
    );
}

pub fn playing_render(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    execute_render_command_buffer(d, &state.render_command_buffer);
}

pub fn game_over_render(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    let mut cursor = Vec2::new(DIMS.x as f32 * 0.28, DIMS.y as f32 * 0.4);
    let title = "GAME OVER!";
    let size = 20;
    d.draw_text(title, cursor.x as i32, cursor.y as i32, size, Color::WHITE);
    cursor.y += size as f32 * 1.5;

    let subtitle = "press space";
    let size = 1;
    d.draw_text(
        subtitle,
        cursor.x as i32,
        cursor.y as i32,
        size,
        Color::WHITE,
    );
}
