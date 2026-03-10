use glam::Vec2;
use hecs::World;
use raylib::prelude::Color;

use crate::{entity_archetypes::spawn_impact_particle, state::State, DIMS};

const MAX_SHAKE: f32 = 3.0;
const MAX_ZOOM_PULSE: f32 = 0.04;

pub fn update(state: &mut State) {
    state.camera_shake *= 0.82;
    if state.camera_shake < 0.05 {
        state.camera_shake = 0.0;
    }

    state.camera_zoom_pulse *= 0.82;
    if state.camera_zoom_pulse < 0.001 {
        state.camera_zoom_pulse = 0.0;
    }

    state.screen_flash *= 0.74;
    if state.screen_flash < 0.01 {
        state.screen_flash = 0.0;
    }

    state.paddle_pulse *= 0.72;
    if state.paddle_pulse < 0.01 {
        state.paddle_pulse = 0.0;
    }

    state.ball_pulse *= 0.78;
    if state.ball_pulse < 0.01 {
        state.ball_pulse = 0.0;
    }

    state.paddle_recoil *= 0.6;
    if state.paddle_recoil.abs() < 0.01 {
        state.paddle_recoil = 0.0;
    }
}

pub fn consume_hitstop(state: &mut State) -> bool {
    if state.hitstop_frames > 0 {
        state.hitstop_frames -= 1;
        return true;
    }
    false
}

pub fn add_hitstop(state: &mut State, frames: u32) {
    state.hitstop_frames = state.hitstop_frames.max(frames);
}

pub fn add_camera_shake(state: &mut State, amount: f32) {
    state.camera_shake = (state.camera_shake + amount).clamp(0.0, MAX_SHAKE);
}

pub fn add_zoom_pulse(state: &mut State, amount: f32) {
    state.camera_zoom_pulse = (state.camera_zoom_pulse + amount).clamp(0.0, MAX_ZOOM_PULSE);
}

pub fn add_screen_flash(state: &mut State, amount: f32) {
    state.screen_flash = (state.screen_flash + amount).clamp(0.0, 1.0);
}

pub fn pulse_paddle(state: &mut State, amount: f32, recoil: f32) {
    state.paddle_pulse = (state.paddle_pulse + amount).clamp(0.0, 1.0);
    state.paddle_recoil = (state.paddle_recoil + recoil).clamp(-4.0, 4.0);
}

pub fn pulse_ball(state: &mut State, amount: f32) {
    state.ball_pulse = (state.ball_pulse + amount).clamp(0.0, 1.0);
}

pub fn world_offset(state: &State) -> Vec2 {
    if state.camera_shake <= 0.0 {
        return Vec2::ZERO;
    }

    let phase = state.t * 0.37;
    Vec2::new((phase * 2.7).sin(), (phase * 3.9).cos()) * state.camera_shake
}

pub fn world_scale(state: &State) -> f32 {
    1.0 + state.camera_zoom_pulse
}

pub fn world_pos(state: &State, pos: Vec2) -> Vec2 {
    let center = DIMS.as_vec2() * 0.5;
    let scale = world_scale(state);
    (pos - center) * scale + center + world_offset(state)
}

pub fn world_dims(state: &State, dims: Vec2) -> Vec2 {
    dims * world_scale(state)
}

pub fn text_pulse_size(state: &State, base: i32, amplitude: f32) -> i32 {
    let pulse = (state.t * 0.15).sin().abs();
    (base as f32 + pulse * amplitude).round() as i32
}

pub fn spawn_hit_particles(ecs: &mut World, pos: Vec2, color: Color, count: u32, speed: f32) {
    for i in 0..count {
        let t = i as f32 / count.max(1) as f32;
        let angle = t * std::f32::consts::TAU;
        let vel = Vec2::new(angle.cos(), angle.sin()) * speed * (0.7 + t * 0.6);
        let size = if i % 3 == 0 { 2.0 } else { 1.0 };
        spawn_impact_particle(ecs, pos, vel, color, size, 10 + (i % 5));
    }
}
