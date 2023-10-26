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

/*

    ////////////////    DRAWING  ////////////////
    let mut draw_handle = rl.begin_drawing(&rlt);
    {
        let low_res_draw_handle =
            &mut draw_handle.begin_texture_mode(&rlt, &mut render_texture);
        low_res_draw_handle.clear_background(Color::BLACK);

        render::draw(&state, low_res_draw_handle);
    }
*/

pub fn scale_and_blit_render_texture_to_window(
    rlt: &RaylibThread,
    draw_handle: &mut RaylibDrawHandle,
    render_texture: &mut RenderTexture2D,
    large_render_texture: &mut RenderTexture2D,
    fullscreen: bool,
    window_dims: UVec2,
    shaders: &[Shader],
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

    {
        let high_res_draw_handle = &mut draw_handle.begin_texture_mode(rlt, large_render_texture);
        high_res_draw_handle.clear_background(Color::BLACK);
        high_res_draw_handle.draw_texture_pro(
            render_texture,
            source_rec,
            dest_rec,
            origin,
            0.0,
            Color::WHITE,
        );
    }

    // now draw the large render texture to the screen
    let source_rec = Rectangle::new(
        0.0,
        0.0,
        large_render_texture.texture.width as f32,
        -large_render_texture.texture.height as f32,
    );
    let dest_rec = Rectangle::new(0.0, 0.0, window_dims.x as f32, window_dims.y as f32);
    let mut shaded_draw_handle = draw_handle.begin_shader_mode(&shaders[0]);
    shaded_draw_handle.draw_texture_pro(
        large_render_texture,
        source_rec,
        dest_rec,
        origin,
        0.0,
        Color::WHITE,
    );
}
