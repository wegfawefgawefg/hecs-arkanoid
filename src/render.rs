use glam::Vec2;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibTextureMode};

use crate::{
    render_commands::execute_render_command_buffer,
    state::{GameMode, LevelCompleteMode, PrepareLevelMode, State},
    DIMS,
};

pub fn draw(state: &State, low_res_draw_handle: &mut RaylibTextureMode<RaylibDrawHandle>) {
    match state.game_mode {
        GameMode::Title => {
            title_render(state, low_res_draw_handle);
        }
        GameMode::PrepareLevel => {
            prepare_level_render(state, low_res_draw_handle);
        }
        GameMode::Playing => {
            playing_render(state, low_res_draw_handle);
        }
        GameMode::LevelComplete => {
            level_complete_render(state, low_res_draw_handle);
        }
        GameMode::WinGame => {
            win_game_render(state, low_res_draw_handle);
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

pub fn prepare_level_render(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    let mut cursor = Vec2::new(DIMS.x as f32 * 0.15, DIMS.y as f32 * 0.7);
    let mode_title = "GameMode: PrepareLevel";
    let size = 1;
    d.draw_text(
        mode_title,
        cursor.x as i32,
        cursor.y as i32,
        size,
        Color::WHITE,
    );
    cursor.y = DIMS.y as f32 * 0.8;
    let mode_title = format!("Mode: {}", state.prepare_level_state.mode.to_string());
    d.draw_text(
        mode_title.as_str(),
        cursor.x as i32,
        cursor.y as i32,
        size,
        Color::WHITE,
    );
    cursor.y = DIMS.y as f32 * 0.9;
    let text = format!("Countdown: {}", state.prepare_level_state.countdown);
    d.draw_text(
        text.as_str(),
        cursor.x as i32,
        cursor.y as i32,
        size,
        Color::WHITE,
    );

    if let PrepareLevelMode::AnnounceLevel = state.prepare_level_state.mode {
        let mut cursor = Vec2::new(DIMS.x as f32 * 0.15, DIMS.y as f32 * 0.4);
        let title = format! {"LeveL: {}", state.level};
        let size = 20;
        d.draw_text(
            title.as_str(),
            cursor.x as i32,
            cursor.y as i32,
            size,
            Color::WHITE,
        );
        cursor.y += size as f32 * 1.5;
    }

    playing_render(state, d);
}

pub fn playing_render(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    execute_render_command_buffer(d, &state.render_command_buffer);
}

const MESSAGES_OF_ENCOURAGEMENT: [&str; 35] = [
    "good job",
    "chill",
    "cool",
    "sweet",
    "dope",
    "lit",
    "on point",
    "solid",
    "keep going",
    "smooth",
    "noice",
    "vibin",
    "clutch",
    "fresh",
    "sick",
    "keep it real",
    "killing it",
    "fire",
    "easy",
    "breezy",
    "you got it",
    "right on",
    "savage",
    "clean",
    "crisp",
    "effortless",
    "hype",
    "groovy",
    "stylish",
    "gucci",
    "sleek",
    "rad",
    "gnarly",
    "aces",
    "epic",
];

pub fn level_complete_render(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    if let LevelCompleteMode::Announce = state.level_complete_state.mode {
        let mut cursor = Vec2::new(DIMS.x as f32 * 0.15, DIMS.y as f32 * 0.4);
        let title = MESSAGES_OF_ENCOURAGEMENT[state.level as usize - 1];
        let size = 20;
        d.draw_text(
            title,
            cursor.x as i32,
            cursor.y as i32,
            size,
            Color::RAYWHITE,
        );
        cursor.y += size as f32 * 1.5;
    } else if let LevelCompleteMode::Announce2 = state.level_complete_state.mode {
        let mut cursor = Vec2::new(DIMS.x as f32 * 0.15, DIMS.y as f32 * 0.4);
        let title = if state.level == 1 {
            "you did it"
        } else {
            "keep going"
        };
        let size = 20;
        d.draw_text(
            title,
            cursor.x as i32,
            cursor.y as i32,
            size,
            Color::RAYWHITE,
        );
        cursor.y += size as f32 * 1.5;
    }
}

pub fn win_game_render(_state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    let mut cursor = Vec2::new(DIMS.x as f32 * 0.15, DIMS.y as f32 * 0.4);
    let title = "tally score and stuff";
    let size = 20;
    d.draw_text(
        title,
        cursor.x as i32,
        cursor.y as i32,
        size,
        Color::RAYWHITE,
    );
    cursor.y += size as f32 * 1.5;
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
