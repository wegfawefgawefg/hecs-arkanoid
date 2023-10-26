use audio::Song;
use audio_playing::execute_audio_command_buffer;
use glam::UVec2;
use hecs::World;
use raylib::prelude::*;
use raylib::{ffi::SetTraceLogLevel, prelude::TraceLogLevel};
use window_helpers::{center_window, scale_and_blit_render_texture_to_window};

mod audio;
mod audio_playing;
mod components;
mod entity_archetypes;
mod game_mode_transitions;
mod input_processing;
mod level_data;
mod message_stream;
mod physics_engine;
mod render;
mod render_commands;
mod state;
mod step;
mod systems;
mod timer;
mod window_helpers;

const DIMS: UVec2 = UVec2::new(240, 160);

const TIMESTEP: f32 = 1.0 / state::FRAMES_PER_SECOND as f32;
fn main() {
    let (mut rl, rlt) = raylib::init().title("raylib-rs-lowres-template").build();
    unsafe {
        SetTraceLogLevel(TraceLogLevel::LOG_WARNING as i32);
    }

    ////////////////    INIT GRAPHICS    ////////////////
    let window_dims = DIMS * 4;
    let fullscreen = false;
    rl.set_window_size(window_dims.x as i32, window_dims.y as i32);
    if fullscreen {
        rl.toggle_fullscreen();
        rl.set_window_size(rl.get_screen_width(), rl.get_screen_height());
    }

    center_window(&mut rl, window_dims);
    let mouse_scale = DIMS.as_vec2() / window_dims.as_vec2();
    rl.set_mouse_scale(mouse_scale.x, mouse_scale.y);

    let mut render_texture = rl
        .load_render_texture(&rlt, DIMS.x, DIMS.y)
        .unwrap_or_else(|e| {
            println!("Error creating render texture: {}", e);
            std::process::exit(1);
        });

    let mut shaders: Vec<Shader> = vec![];
    let texture_names = vec!["grayscale.fs"];
    for name in texture_names {
        let path = format!("src/shaders/{}", name);
        match rl.load_shader(&rlt, None, Some(&path)) {
            Ok(shader) => shaders.push(shader),
            Err(e) => {
                println!("Error loading shader: {}", e);
                std::process::exit(1);
            }
        };
    }

    ////////////////    INIT AUDIO    ////////////////
    let mut audio = audio::Audio::new(&mut rl, &rlt);
    audio
        .rl_audio_device
        .play_music_stream(&mut audio.songs[Song::Playing as usize]);

    ////////////////    INIT STATE    ////////////////
    let mut state = state::State::new();
    let mut ecs = World::new();

    ////////////////    MAIN LOOP    ////////////////
    while state.running && !rl.window_should_close() {
        game_mode_transitions::transition_game_mode(&mut ecs, &mut state);
        input_processing::process_input(&mut rl, &mut state);

        let dt = rl.get_frame_time();
        state.time_since_last_update += dt;
        if state.time_since_last_update > TIMESTEP {
            state.time_since_last_update = 0.0;

            state.render_command_buffer.clear();
            state.audio_command_buffer.clear();

            step::step(&mut rl, &mut ecs, &mut state);
        }

        ////////////////    AUDIO STEP  ////////////////
        execute_audio_command_buffer(&mut rl, &mut audio, &mut state.audio_command_buffer);

        // audio // UNMUTE THIS TO HEAR THE MUSIC
        //     .rl_audio_device
        //     .update_music_stream(&mut audio.songs[Song::Playing as usize]);

        ////////////////    DRAWING  ////////////////
        let mut draw_handle = rl.begin_drawing(&rlt);
        {
            let low_res_draw_handle =
                &mut draw_handle.begin_texture_mode(&rlt, &mut render_texture);
            low_res_draw_handle.clear_background(Color::BLACK);

            render::draw(&state, low_res_draw_handle);
        }
        scale_and_blit_render_texture_to_window(
            &mut draw_handle,
            &mut render_texture,
            fullscreen,
            window_dims,
            &shaders,
        );
    }
}
