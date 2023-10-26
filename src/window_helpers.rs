use glam::UVec2;
use raylib::prelude::*;

pub fn center_window(rl: &mut raylib::RaylibHandle, window_dims: UVec2) {
    let screen_dims = UVec2::new(rl.get_screen_width() as u32, rl.get_screen_height() as u32);
    let screen_center = screen_dims / 2;
    let window_center = window_dims / 2;
    let mut offset = window_center - screen_center;
    offset.y += 500;
    rl.set_window_position(offset.x as i32, offset.y as i32);
    rl.set_target_fps(144);
}

pub fn scale_and_blit_render_texture_to_window(
    draw_handle: &mut RaylibDrawHandle,
    render_texture: &mut RenderTexture2D,
    fullscreen: bool,
    window_dims: UVec2,
    shaders: &Vec<Shader>,
) {
    let source_rec = Rectangle::new(
        0.0,
        0.0,
        render_texture.texture.width as f32,
        -render_texture.texture.height as f32,
    );
    // dest rec should be the fullscreen resolution if graphics.fullscreen, otherwise window_dims
    let dest_rec = if fullscreen {
        // get the fullscreen resolution
        let screen_width = draw_handle.get_screen_width();
        let screen_height = draw_handle.get_screen_height();
        Rectangle::new(0.0, 0.0, screen_width as f32, screen_height as f32)
    } else {
        Rectangle::new(0.0, 0.0, window_dims.x as f32, window_dims.y as f32)
    };

    let origin = Vector2::new(0.0, 0.0);

    let mut shaded_draw_handle = draw_handle.begin_shader_mode(&shaders[0]);
    shaded_draw_handle.draw_texture_pro(
        render_texture,
        source_rec,
        dest_rec,
        origin,
        0.0,
        Color::WHITE,
    );
}
