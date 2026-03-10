use glam::Vec2;
use hecs::World;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibTextureMode};

use crate::{
    components::{Block, CTransform, Health, Shape, StrongBlock},
    gameplay_render, juice, level_data,
    render_helpers::draw_rect_outline,
    state::{GameMode, GameOverMode, LevelCompleteMode, PrepareLevelMode, State, WinGameMode},
    DIMS,
};

const LEVEL_GAP_SIZE: f32 = 1.0;
const LEVEL_BLOCK_WIDTH: f32 = 20.0;
const LEVEL_BLOCK_HEIGHT: f32 = 8.0;
const LEVEL_STRIP_OFFSET_X: f32 = 240.0 + 40.0;
const WALL_THICKNESS: f32 = 20.0;

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

fn sigmoid01(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let x = t * 2.0 - 1.0;
    let y = 1.0 / (1.0 + (-6.0 * x).exp());
    let min = 1.0 / (1.0 + 6.0_f32.exp());
    let max = 1.0 / (1.0 + (-6.0_f32).exp());
    ((y - min) / (max - min)).clamp(0.0, 1.0)
}

fn level_world_center(level_offset_x: f32) -> Vec2 {
    Vec2::new(DIMS.x as f32 * 0.5 + level_offset_x, DIMS.y as f32 * 0.5)
}

fn draw_stage_walls(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    camera_center: Vec2,
    zoom: f32,
    level_offset_x: f32,
) {
    let screen_center = DIMS.as_vec2() * 0.5;
    let transform = |world: Vec2| (world - camera_center) * zoom + screen_center;

    let mut draw_wall = |pos: Vec2, dims: Vec2, color: Color| {
        let top_left = transform(pos + Vec2::new(level_offset_x, 0.0));
        let dims = dims * zoom;
        d.draw_rectangle(
            top_left.x.round() as i32,
            top_left.y.round() as i32,
            dims.x.round().max(1.0) as i32,
            dims.y.round().max(1.0) as i32,
            color,
        );
    };

    draw_wall(
        Vec2::new(0.0, -WALL_THICKNESS + 1.0),
        Vec2::new(DIMS.x as f32, WALL_THICKNESS),
        Color::WHITE,
    );
    draw_wall(
        Vec2::new(0.0, DIMS.y as f32 - 1.0),
        Vec2::new(DIMS.x as f32, WALL_THICKNESS),
        Color::RED,
    );
    draw_wall(
        Vec2::new(-WALL_THICKNESS + 1.0, 0.0),
        Vec2::new(WALL_THICKNESS, DIMS.y as f32),
        Color::WHITE,
    );
    draw_wall(
        Vec2::new(DIMS.x as f32 - 1.0, 0.0),
        Vec2::new(WALL_THICKNESS, DIMS.y as f32),
        Color::WHITE,
    );
}

fn draw_block_style(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    pos: Vec2,
    dims: Vec2,
    color: Color,
    strong: bool,
    hp: u32,
) {
    if strong {
        d.draw_rectangle(
            pos.x.round() as i32,
            pos.y.round() as i32,
            dims.x.round().max(1.0) as i32,
            dims.y.round().max(1.0) as i32,
            color,
        );
    } else {
        draw_rect_outline(d, pos, dims, color);
        if hp > 1 {
            let left = pos.x.round() as i32;
            let top = pos.y.round() as i32;
            let width = dims.x.round().max(1.0) as i32;
            let height = dims.y.round().max(1.0) as i32;
            d.draw_line(left, top, left + width - 1, top + height - 1, color);
        }
    }
}

fn draw_level_preview(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    level: u32,
    camera_center: Vec2,
    zoom: f32,
    level_offset_x: f32,
    alpha: u8,
) {
    let level = level.clamp(1, level_data::LEVEL_BLOCK_DATA.len() as u32);
    let layout = level_data::LEVEL_BLOCK_DATA[(level - 1) as usize];
    let screen_center = DIMS.as_vec2() * 0.5;
    let transform = |world: Vec2| (world - camera_center) * zoom + screen_center;

    let cursor_x_start = 4.0 + level_offset_x;
    let mut cursor = Vec2::new(cursor_x_start, 2.0);
    for y in 0..=13 {
        cursor.x = cursor_x_start;
        cursor.y += LEVEL_GAP_SIZE;
        for x in 0..=11 {
            cursor.x += LEVEL_GAP_SIZE;
            if x == 11 {
                break;
            }
            let color_index = layout[y + 2][x];
            if color_index == 0 {
                cursor.x += LEVEL_BLOCK_WIDTH;
                continue;
            }

            let color = level_data::RL_COLOR_PALETTE[color_index as usize];
            let color = Color::new(color.r, color.g, color.b, alpha);
            let pos = transform(cursor);
            let dims = Vec2::new(LEVEL_BLOCK_WIDTH, LEVEL_BLOCK_HEIGHT) * zoom;
            draw_block_style(
                d,
                pos,
                dims,
                color,
                color_index == 10,
                if color_index == 9 { 2 } else { 1 },
            );

            cursor.x += LEVEL_BLOCK_WIDTH;
        }
        cursor.y += LEVEL_BLOCK_HEIGHT;
    }
}

fn draw_live_stage_preview(
    ecs: &World,
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    camera_center: Vec2,
    zoom: f32,
    level_offset_x: f32,
) {
    let screen_center = DIMS.as_vec2() * 0.5;
    let transform = |world: Vec2| (world - camera_center) * zoom + screen_center;

    for (entity, block, ctransform, shape, health) in ecs
        .query::<(hecs::Entity, &Block, &CTransform, &Shape, &Health)>()
        .iter()
    {
        let world = ctransform.pos + Vec2::new(level_offset_x, 0.0);
        let pos = transform(world);
        let dims = shape.dims * zoom;
        let strong = ecs.satisfies::<&StrongBlock>(entity);
        draw_block_style(d, pos, dims, block.color, strong, health.hp);
    }
}

fn draw_stage_transition_strip(
    ecs: &World,
    state: &State,
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
) {
    let announce2 = (40.0 * crate::TS_RATIO) as u32;
    let pause = (40.0 * crate::TS_RATIO) as u32;
    d.draw_rectangle(0, 0, DIMS.x as i32, DIMS.y as i32, Color::BLACK);
    let current_center = level_world_center(0.0);
    let next_alpha = if matches!(state.level_complete_state.mode, LevelCompleteMode::Pause) {
        255
    } else {
        220
    };

    let (camera_center, zoom, next_offset_x) = match state.level_complete_state.mode {
        LevelCompleteMode::Announce => (current_center, 1.0, LEVEL_STRIP_OFFSET_X),
        LevelCompleteMode::Announce2 => {
            let t = 1.0 - (state.level_complete_state.countdown as f32 / announce2 as f32);
            let zoom_t = sigmoid01((t / 0.45).clamp(0.0, 1.0));
            let pan_t = sigmoid01(((t - 0.22) / 0.56).clamp(0.0, 1.0));
            let zoom = 1.0 + (0.36 - 1.0) * zoom_t;
            let center = current_center.lerp(level_world_center(LEVEL_STRIP_OFFSET_X), pan_t);
            (center, zoom, LEVEL_STRIP_OFFSET_X)
        }
        LevelCompleteMode::Pause => {
            let t = 1.0 - (state.level_complete_state.countdown as f32 / pause as f32);
            let zoom = 0.36 + (1.0 - 0.36) * sigmoid01(t);
            (
                level_world_center(LEVEL_STRIP_OFFSET_X),
                zoom,
                LEVEL_STRIP_OFFSET_X,
            )
        }
    };

    draw_stage_walls(d, camera_center, zoom, 0.0);
    draw_stage_walls(d, camera_center, zoom, next_offset_x);
    draw_live_stage_preview(ecs, d, camera_center, zoom, 0.0);
    draw_level_preview(
        d,
        state.level_complete_target_level,
        camera_center,
        zoom,
        next_offset_x,
        next_alpha,
    );

    d.draw_text(
        format!("{} -> {}", state.level, state.level_complete_target_level).as_str(),
        DIMS.x as i32 / 2 - 18,
        (DIMS.y as f32 * 0.16).round() as i32,
        10,
        Color::WHITE,
    );
}

pub fn level_complete_render(
    ecs: &World,
    state: &State,
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
) {
    if let LevelCompleteMode::Announce = state.level_complete_state.mode {
        playing_render(ecs, state, d);
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
        draw_stage_transition_strip(ecs, state, d);
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
        draw_stage_transition_strip(ecs, state, d);
        let cursor = Vec2::new(DIMS.x as f32 * 0.36, DIMS.y as f32 * 0.12);
        let title = format!("Level {}", state.level_complete_target_level);
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
