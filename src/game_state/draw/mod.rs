use super::*;

pub mod chain;
pub mod graph;
mod rule;

use chain::*;
use graph::*;

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Main graph
        let rule_options = self
            .selection
            .as_ref()
            .and_then(|selection| selection.inferred_options().as_ref());
        draw_graph(
            &self.geng,
            &self.assets,
            self.geng.default_font(),
            framebuffer,
            &self.camera,
            &self.main_graph,
            Color::BLACK,
            rule_options,
        );

        // Rules
        self.draw_rules(framebuffer);

        // Dragging
        if let Some(dragging) = &self.dragging {
            match &dragging.action {
                DragAction::Selection {
                    current_mouse_position,
                } => {
                    let world_pos = self.camera.screen_to_world(
                        self.framebuffer_size,
                        current_mouse_position.map(|x| x as f32),
                    );
                    self.geng.draw_2d(
                        framebuffer,
                        &self.camera,
                        &draw_2d::Quad::new(
                            AABB::from_corners(dragging.world_start_position, world_pos),
                            SELECTION_COLOR,
                        ),
                    );
                }
                _ => (),
            }
        }
    }
}
