use glam::Vec2;
use raylib::prelude::Color;

use crate::{
    components::{CTransform, Player},
    render_commands::{RenderCommand, RenderCommandBuffer},
    DIMS,
};

// pub fn game_over(ecs: &mut SubWorld, #[resource] render_command_buffer: &mut RenderCommandBuffer) {
//     let mut players = <(Entity, &CTransform)>::query().filter(component::<Player>());
//     if players.iter(ecs).count() == 0 {
//         let mut cursor = Vec2::new(DIMS.x as f32 * 0.28, DIMS.y as f32 * 0.4);
//         let title = "GAME OVER!";
//         let size = 20;
//         render_command_buffer.push(DrawCommand::Text {
//             pos: cursor,
//             text: title.to_string(),
//             size,
//             color: Color::RED,
//         });

//         cursor.y += size as f32 * 1.5;

//         let subtitle = "press space";
//         let size = 1;
//         render_command_buffer.push(DrawCommand::Text {
//             pos: cursor,
//             text: subtitle.to_string(),
//             size,
//             color: Color::RED,
//         });
//     }
// }
