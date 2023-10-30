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
        hp: u32,
        ball_unbreakable: bool,
    },
    Ball {
        pos: Vec2,
        dims: Vec2,
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
    SolidRectangle {
        pos: Vec2,
        dims: Vec2,
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
            RenderCommand::Ball { pos, dims } => d.draw_rectangle_lines(
                pos.x as i32,
                pos.y as i32,
                dims.x as i32,
                dims.y as i32,
                Color::RAYWHITE,
            ),
            RenderCommand::ColoredSquare { pos, color } => {
                d.draw_rectangle(pos.x as i32, pos.y as i32, SIZE, SIZE, *color);
            }
            RenderCommand::Block {
                pos,
                dims,
                color,
                hp,
                ball_unbreakable,
            } => {
                if *ball_unbreakable {
                    d.draw_rectangle(
                        pos.x as i32,
                        pos.y as i32,
                        dims.x as i32,
                        dims.y as i32,
                        *color,
                    );
                    continue;
                } else {
                    d.draw_rectangle_lines(
                        pos.x as i32,
                        pos.y as i32,
                        dims.x as i32,
                        dims.y as i32,
                        color,
                    );
                    if *hp > 1 {
                        d.draw_line_v(
                            Vector2::new(pos.x, pos.y),
                            Vector2::new(pos.x + dims.x - 1.0, pos.y + dims.y),
                            *color,
                        );
                    }
                }
            }
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
            RenderCommand::SolidRectangle { pos, dims, color } => {
                d.draw_rectangle(
                    pos.x as i32,
                    pos.y as i32,
                    dims.x as i32,
                    dims.y as i32,
                    *color,
                );
            }
        }
    }
}
