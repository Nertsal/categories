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
        draw_graph(
            self.geng.draw_2d(),
            self.geng.default_font(),
            framebuffer,
            &self.camera,
            &self.force_graph,
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

        // Selection
        for vertex in self
            .selection
            .vertices
            .iter()
            .filter_map(|vertex| self.force_graph.graph.vertices.get(vertex))
        {
            self.geng.draw_2d().circle(
                framebuffer,
                &self.camera,
                vertex.body.position,
                vertex.vertex.radius + SELECTED_RADIUS,
                SELECTED_COLOR,
            )
        }
        for edge_points in self.selection.edges.iter().filter_map(|edge| {
            self.force_graph.graph.edges.get(edge).and_then(|arrow| {
                self.force_graph
                    .graph
                    .vertices
                    .get(&arrow.edge.from)
                    .and_then(|from| {
                        self.force_graph
                            .graph
                            .vertices
                            .get(&arrow.edge.to)
                            .map(|to| {
                                let mut points = Vec::with_capacity(arrow.bodies.len() + 2);
                                points.push(from.body.position);
                                points.extend(arrow.bodies.iter().map(|body| body.position));
                                points.push(to.body.position);
                                points
                            })
                    })
            })
        }) {
            draw_chain(
                self.geng.draw_2d(),
                framebuffer,
                &self.camera,
                CardinalSpline {
                    points: edge_points,
                    tension: 0.5,
                }
                .chain(CURVE_RESOLUTION, ARROW_WIDTH + SELECTED_RADIUS),
                SELECTED_COLOR,
            );
        }
    }
}
