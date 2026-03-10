use audio::Song;
use audio_playing::execute_audio_command_buffer;
use glam::UVec2;
use hecs::World;
use raylib::prelude::*;
use raylib::{ffi::SetTraceLogLevel, prelude::TraceLogLevel};
use window_helpers::{center_window, scale_and_blit_render_texture_to_window};

mod audio;
mod audio_playing;
mod cleanup;
mod components;
mod entity_archetypes;
mod game_mode_transitions;
mod gameplay_input;
mod gameplay_render;
mod input_processing;
mod juice;
mod level_data;
mod level_layouts_data;
mod level_palette;
mod level_rules;
mod message_stream;
mod particles;
mod physics;
mod powerups;
mod render;
mod render_helpers;
mod state;
mod step;
mod timer;
mod window_helpers;

pub const DIMS: UVec2 = UVec2::new(240, 160);
use lazy_static::lazy_static;
lazy_static! {
    pub static ref WINDOW_DIMS: UVec2 = DIMS * 4;
}

const TIMESTEP: f32 = 1.0 / state::FRAMES_PER_SECOND as f32;
const TS_RATIO: f32 = state::FRAMES_PER_SECOND as f32 / 60.0;
fn main() {
    let (mut rl, rlt) = raylib::init().title("raylib-rs-lowres-template").build();
    unsafe {
        SetTraceLogLevel(TraceLogLevel::LOG_WARNING as i32);
    }

    ////////////////    INIT GRAPHICS    ////////////////
    let fullscreen = false;
    rl.set_window_size(WINDOW_DIMS.x as i32, WINDOW_DIMS.y as i32);
    rl.disable_cursor();
    if fullscreen {
        rl.toggle_fullscreen();
        rl.set_window_size(rl.get_screen_width(), rl.get_screen_height());
    }

    center_window(&mut rl, *WINDOW_DIMS);
    let mouse_scale = DIMS.as_vec2() / WINDOW_DIMS.as_vec2();
    rl.set_mouse_scale(mouse_scale.x, mouse_scale.y);

    let mut render_texture = rl
        .load_render_texture(&rlt, DIMS.x, DIMS.y)
        .unwrap_or_else(|e| {
            println!("Error creating render texture: {}", e);
            std::process::exit(1);
        });
    let mut large_render_texture = rl
        .load_render_texture(&rlt, WINDOW_DIMS.x, WINDOW_DIMS.y)
        .unwrap_or_else(|e| {
            println!("Error creating render texture: {}", e);
            std::process::exit(1);
        });

    let mut shaders: Vec<Shader> = vec![];
    let texture_names = vec!["grayscale.fs"];
    for name in texture_names {
        let path = format!("src/shaders/{}", name);
        shaders.push(rl.load_shader(&rlt, None, Some(&path)));
    }

    ////////////////    INIT AUDIO    ////////////////
    let mut audio = audio::Audio::new();
    audio.songs[Song::Playing as usize].play_stream();

    ////////////////    INIT STATE    ////////////////
    let mut state = state::State::new();
    let mut ecs = World::new();

    ////////////////    MAIN LOOP    ////////////////
    let mut fps_history = std::collections::VecDeque::with_capacity(10);
    while state.running && !rl.window_should_close() {
        let time_a = std::time::Instant::now();
        game_mode_transitions::transition_game_mode(&mut ecs, &mut state);
        input_processing::process_input(&mut rl, &mut state);

        // lock mouse to screen
        // if rl.is_window_focused() {
        //     let mouse_pos = rl.get_mouse_position();
        //     let mp = Vec2::new(mouse_pos.x, mouse_pos.y);
        //     let mouse_window_pos = mp / DIMS.as_vec2() * WINDOW_DIMS.as_vec2();
        //     println!("mouse_window_pos: {:?}", mouse_window_pos);
        //     if mouse_window_pos.x < 0.0
        //         || mouse_window_pos.x > WINDOW_DIMS.x as f32
        //         || mouse_window_pos.y < 0.0
        //         || mouse_window_pos.y > WINDOW_DIMS.y as f32
        //     {
        //         rl.set_mouse_position(Vector2::new(
        //             WINDOW_DIMS.x as f32 / 2.0,
        //             WINDOW_DIMS.y as f32 / 2.0,
        //         ));
        //         rl.set_mouse_position(Vector2::new(

        //         ));
        //     }
        // }

        let dt = rl.get_frame_time();
        state.time_since_last_update += dt;
        if state.time_since_last_update > TIMESTEP {
            state.t += 1.0;
            state.time_since_last_update = 0.0;

            state.audio_command_buffer.clear();

            step::step(&mut rl, &mut ecs, &mut state);
            ////////////////    AUDIO STEP  ////////////////
            execute_audio_command_buffer(&mut rl, &mut audio, &mut state.audio_command_buffer);
        }

        audio.songs[Song::Playing as usize].update_stream();

        ////////////////    DRAWING  ////////////////
        let mut draw_handle = rl.begin_drawing(&rlt);
        {
            let low_res_draw_handle =
                &mut draw_handle.begin_texture_mode(&rlt, &mut render_texture);
            low_res_draw_handle.clear_background(Color::BLACK);

            render::draw(&ecs, &state, low_res_draw_handle);
        }
        scale_and_blit_render_texture_to_window(
            &rlt,
            &mut draw_handle,
            &mut render_texture,
            &mut large_render_texture,
            fullscreen,
            *WINDOW_DIMS,
            &mut shaders,
        );

        let time_b = std::time::Instant::now();
        let frame_duration = (time_b - time_a).as_secs_f32();
        let fps = 1.0 / frame_duration;
        fps_history.push_back(fps);
        state.fps = fps_history.iter().sum::<f32>() / fps_history.len() as f32;
    }
}
