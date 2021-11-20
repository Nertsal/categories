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
            self.geng.draw_2d(),
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
                DragAction::Selection => {
                    let world_pos = self.camera.screen_to_world(
                        self.framebuffer_size,
                        self.geng.window().mouse_pos().map(|x| x as f32),
                    );
                    self.geng.draw_2d().quad(
                        framebuffer,
                        &self.camera,
                        AABB::from_corners(dragging.world_start_position, world_pos),
                        SELECTION_COLOR,
                    );
                }
                _ => (),
            }
        }
    }
}
