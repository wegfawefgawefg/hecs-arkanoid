use glam::Vec2;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibTextureMode, Vector2};

pub type RenderCommandBuffer = Vec<RenderCommand>;

#[derive(Clone)]
pub enum RenderCommand {
    ColoredSquare {
        pos: Vec2,
        color: Color,
    },
    Block {
        pos: Vec2,
        dims: Vec2,
        color: Color,
    },
    Paddle {
        pos: Vec2,
        dims: Vec2,
        color: Color,
    },
    Text {
        pos: Vec2,
        text: String,
        size: i32,
        color: Color,
    },
    Line {
        start: Vec2,
        end: Vec2,
        color: Color,
    },
    Circle {
        pos: Vec2,
        radius: f32,
        color: Color,
    },
}

// defualt entity size
const SIZE: i32 = 1;
const SEGMENTS: usize = 12;
static RADIUS_VARIATIONS: [f32; SEGMENTS] = [
    0.8, 0.75, 0.9, 0.85, 0.7, 0.88, 0.95, 0.78, 0.92, 0.76, 0.87, 0.8,
];

pub fn execute_render_command_buffer(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    render_command_buffer: &RenderCommandBuffer,
) {
    for command in render_command_buffer.iter() {
        match command {
            RenderCommand::ColoredSquare { pos, color } => {
                d.draw_rectangle(pos.x as i32, pos.y as i32, SIZE, SIZE, *color);
            }
            RenderCommand::Block { pos, dims, color } => d.draw_rectangle_lines(
                pos.x as i32,
                pos.y as i32,
                dims.x as i32,
                dims.y as i32,
                color,
            ),
            RenderCommand::Paddle { pos, dims, color } => d.draw_rectangle_lines(
                pos.x as i32,
                pos.y as i32,
                dims.x as i32,
                dims.y as i32,
                color,
            ),
            RenderCommand::Text {
                pos,
                text,
                size,
                color,
            } => {
                d.draw_text(text, pos.x as i32, pos.y as i32, *size, *color);
            }
            RenderCommand::Line { start, end, color } => {
                d.draw_line_v(
                    Vector2::new(start.x, start.y),
                    Vector2::new(end.x, end.y),
                    *color,
                );
            }
            RenderCommand::Circle { pos, radius, color } => {
                d.draw_circle(pos.x as i32, pos.y as i32, *radius, *color);
            }
        }
    }
}
