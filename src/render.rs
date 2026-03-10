use glam::Vec2;
use hecs::World;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibTextureMode};

use crate::{
    gameplay_render, juice, level_data,
    state::{GameMode, GameOverMode, LevelCompleteMode, PrepareLevelMode, State, WinGameMode},
    DIMS,
};

pub fn draw(
    ecs: &World,
    state: &State,
    low_res_draw_handle: &mut RaylibTextureMode<RaylibDrawHandle>,
) {
    match state.game_mode {
        GameMode::Title => {
            title_render(state, low_res_draw_handle);
        }
        GameMode::PrepareLevel => {
            prepare_level_render(ecs, state, low_res_draw_handle);
        }
        GameMode::Playing => {
            playing_render(ecs, state, low_res_draw_handle);
        }
        GameMode::LevelComplete => {
            level_complete_render(ecs, state, low_res_draw_handle);
        }
        GameMode::WinGame => {
            win_game_render(ecs, state, low_res_draw_handle);
        }
        GameMode::GameOver => {
            game_over_render(ecs, state, low_res_draw_handle);
        }
    }

    if state.screen_flash > 0.0 {
        let alpha = (state.screen_flash * 96.0).clamp(0.0, 96.0) as u8;
        low_res_draw_handle.draw_rectangle(
            0,
            0,
            DIMS.x as i32,
            DIMS.y as i32,
            Color::new(255, 255, 255, alpha),
        );
    }
}

////////////////////////    PER GAME MODE DRAW FUNCTIONS     ////////////////////////
pub fn title_render(_state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    let mut cursor = Vec2::new(DIMS.x as f32 * 0.15, DIMS.y as f32 * 0.4);
    let title = "HECS-arkanoid!";
    let size = juice::text_pulse_size(_state, 20, 3.0);
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

pub fn prepare_level_render(
    ecs: &World,
    state: &State,
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
) {
    // let mut cursor = Vec2::new(DIMS.x as f32 * 0.15, DIMS.y as f32 * 0.7);
    // let mode_title = "GameMode: PrepareLevel";
    // let size = 1;
    // d.draw_text(
    //     mode_title,
    //     cursor.x as i32,
    //     cursor.y as i32,
    //     size,
    //     Color::WHITE,
    // );
    // cursor.y = DIMS.y as f32 * 0.8;
    // let mode_title = format!("Mode: {}", state.prepare_level_state.mode.to_string());
    // d.draw_text(
    //     mode_title.as_str(),
    //     cursor.x as i32,
    //     cursor.y as i32,
    //     size,
    //     Color::WHITE,
    // );
    // cursor.y = DIMS.y as f32 * 0.9;
    // let text = format!("Countdown: {}", state.prepare_level_state.countdown);
    // d.draw_text(
    //     text.as_str(),
    //     cursor.x as i32,
    //     cursor.y as i32,
    //     size,
    //     Color::WHITE,
    // );

    playing_render(ecs, state, d);

    if let PrepareLevelMode::AnnounceLevel = state.prepare_level_state.mode {
        let mut cursor = Vec2::new(DIMS.x as f32 * 0.15, DIMS.y as f32 * 0.4);
        let title = format! {"LeveL: {}", state.level};
        let size = juice::text_pulse_size(state, 20, 3.0);
        d.draw_text(
            title.as_str(),
            cursor.x as i32,
            cursor.y as i32,
            size,
            Color::WHITE,
        );
        cursor.y += size as f32 * 1.5;
    }
}

pub fn playing_render(ecs: &World, state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    gameplay_render::render(ecs, state, d);
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

fn smoothstep01(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn draw_level_preview(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    level: u32,
    origin: Vec2,
    cell: Vec2,
    cell_gap: f32,
    block_alpha: u8,
) -> Vec2 {
    let level = level.clamp(1, level_data::LEVEL_BLOCK_DATA.len() as u32);
    let layout = level_data::LEVEL_BLOCK_DATA[(level - 1) as usize];

    for (y, row) in layout.iter().enumerate() {
        for (x, color_index) in row.iter().enumerate() {
            if *color_index == 0 {
                continue;
            }
            let color = level_data::RL_COLOR_PALETTE[*color_index as usize];
            let color = Color::new(color.r, color.g, color.b, block_alpha);
            let pos = origin
                + Vec2::new(
                    x as f32 * (cell.x + cell_gap),
                    y as f32 * (cell.y + cell_gap),
                );
            d.draw_rectangle(
                pos.x.round() as i32,
                pos.y.round() as i32,
                cell.x.round() as i32,
                cell.y.round() as i32,
                color,
            );
        }
    }

    let preview_dims = Vec2::new(
        11.0 * cell.x + 10.0 * cell_gap,
        28.0 * cell.y + 27.0 * cell_gap,
    );
    preview_dims
}

fn draw_stage_transition_strip(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    let announce = (60.0 * crate::TS_RATIO) as u32;
    let announce2 = (40.0 * crate::TS_RATIO) as u32;
    let pause = (40.0 * crate::TS_RATIO) as u32;
    let total = (announce + announce2 + pause) as f32;
    let elapsed = match state.level_complete_state.mode {
        LevelCompleteMode::Announce => {
            announce.saturating_sub(state.level_complete_state.countdown)
        }
        LevelCompleteMode::Announce2 => {
            announce + announce2.saturating_sub(state.level_complete_state.countdown)
        }
        LevelCompleteMode::Pause => {
            announce + announce2 + pause.saturating_sub(state.level_complete_state.countdown)
        }
    } as f32;
    let progress = (elapsed / total).clamp(0.0, 1.0);

    let cell = Vec2::new(7.0, 3.0);
    let cell_gap = 1.0;
    let preview_gap = 18.0;
    let preview_dims = Vec2::new(
        11.0 * cell.x + 10.0 * cell_gap,
        28.0 * cell.y + 27.0 * cell_gap,
    );
    let strip_origin = Vec2::new(
        ((DIMS.x as f32) - (preview_dims.x * 2.0 + preview_gap)) * 0.5,
        22.0,
    );
    let current_origin = strip_origin;
    let next_origin = strip_origin + Vec2::new(preview_dims.x + preview_gap, 0.0);

    let current_center = current_origin + preview_dims * 0.5;
    let next_center = next_origin + preview_dims * 0.5;

    let (camera_center, zoom) = if progress < 0.34 {
        let t = smoothstep01(progress / 0.34);
        (current_center, 2.15 + (1.0 - 2.15) * t)
    } else if progress < 0.7 {
        let t = smoothstep01((progress - 0.34) / 0.36);
        (current_center.lerp(next_center, t), 1.0)
    } else {
        let t = smoothstep01((progress - 0.7) / 0.3);
        (next_center, 1.0 + (2.15 - 1.0) * t)
    };

    let screen_center = DIMS.as_vec2() * 0.5;
    let transform = |p: Vec2| (p - camera_center) * zoom + screen_center;

    let current_draw = transform(current_origin);
    let next_draw = transform(next_origin);
    let dims_draw = preview_dims * zoom;

    d.draw_rectangle(0, 0, DIMS.x as i32, DIMS.y as i32, Color::BLACK);

    let current_frame_color = Color::new(180, 180, 180, 255);
    let next_frame_color = Color::new(140, 140, 140, 255);
    d.draw_rectangle_lines(
        current_draw.x.round() as i32 - 2,
        current_draw.y.round() as i32 - 2,
        dims_draw.x.round() as i32 + 4,
        dims_draw.y.round() as i32 + 4,
        current_frame_color,
    );
    d.draw_rectangle_lines(
        next_draw.x.round() as i32 - 2,
        next_draw.y.round() as i32 - 2,
        dims_draw.x.round() as i32 + 4,
        dims_draw.y.round() as i32 + 4,
        next_frame_color,
    );

    let scaled_cell = cell * zoom;
    let scaled_gap = cell_gap * zoom;
    let _ = draw_level_preview(d, state.level, current_draw, scaled_cell, scaled_gap, 255);
    let _ = draw_level_preview(
        d,
        (state.level + 1).clamp(1, level_data::LEVEL_BLOCK_DATA.len() as u32),
        next_draw,
        scaled_cell,
        scaled_gap,
        220,
    );

    let ribbon_y = (screen_center.y + dims_draw.y * 0.5 + 8.0).round() as i32;
    d.draw_text(
        format!("{} -> {}", state.level, state.level + 1).as_str(),
        screen_center.x.round() as i32 - 18,
        ribbon_y,
        10,
        Color::WHITE,
    );
}

pub fn level_complete_render(
    ecs: &World,
    state: &State,
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
) {
    let _ = ecs;
    draw_stage_transition_strip(state, d);

    if let LevelCompleteMode::Announce = state.level_complete_state.mode {
        let mut cursor = Vec2::new(DIMS.x as f32 * 0.15, DIMS.y as f32 * 0.4);
        let title = MESSAGES_OF_ENCOURAGEMENT[state.level as usize - 1];
        let size = juice::text_pulse_size(state, 20, 4.0);
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
        let size = juice::text_pulse_size(state, 20, 4.0);
        d.draw_text(
            title,
            cursor.x as i32,
            cursor.y as i32,
            size,
            Color::RAYWHITE,
        );
        cursor.y += size as f32 * 1.5;
    } else if let LevelCompleteMode::Pause = state.level_complete_state.mode {
        let cursor = Vec2::new(DIMS.x as f32 * 0.36, DIMS.y as f32 * 0.12);
        let title = format!("Level {}", state.level + 1);
        let size = juice::text_pulse_size(state, 16, 3.0);
        d.draw_text(
            title.as_str(),
            cursor.x as i32,
            cursor.y as i32,
            size,
            Color::RAYWHITE,
        );
    }
}

pub fn win_game_render(ecs: &World, state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    playing_render(ecs, state, d);

    if let WinGameMode::Announce = state.win_game_state.mode {
        let mut cursor = Vec2::new(DIMS.x as f32 * 0.15, DIMS.y as f32 * 0.4);
        let title = "you did it";
        let size = juice::text_pulse_size(state, 20, 4.0);
        d.draw_text(
            title,
            cursor.x as i32,
            cursor.y as i32,
            size,
            Color::RAYWHITE,
        );
        cursor.y += size as f32 * 1.5;
    } else if let WinGameMode::Announce2 = state.win_game_state.mode {
        let mut cursor = Vec2::new(DIMS.x as f32 * 0.15, DIMS.y as f32 * 0.4);
        let title = "see you soon";
        let size = juice::text_pulse_size(state, 20, 4.0);
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

pub fn game_over_render(ecs: &World, state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    playing_render(ecs, state, d);

    if let GameOverMode::Announce = state.game_over_state.mode {
        let mut cursor = Vec2::new(DIMS.x as f32 * 0.15, DIMS.y as f32 * 0.4);
        let title = "too bad";
        let size = juice::text_pulse_size(state, 20, 4.0);
        d.draw_text(
            title,
            cursor.x as i32,
            cursor.y as i32,
            size,
            Color::RAYWHITE,
        );
        cursor.y += size as f32 * 1.5;
    } else if let GameOverMode::Announce2 = state.game_over_state.mode {
        let mut cursor = Vec2::new(DIMS.x as f32 * 0.15, DIMS.y as f32 * 0.4);
        let title = "try again?";
        let size = juice::text_pulse_size(state, 20, 4.0);
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
