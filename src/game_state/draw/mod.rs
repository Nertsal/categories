use super::*;

mod chain;
mod graph;
mod rule;

use chain::*;
use graph::*;

const RULES_WIDTH: f32 = 20.0;
const RULE_RESOLUTION: Vec2<usize> = vec2(640, 360);
const RULES_SECTION_SEPARATION_WIDTH: f32 = 1.0;
const RULE_SEPARATION_WIDTH: f32 = 0.2;
const RULES_SECTION_SEPARATION_COLOR: Color<f32> = Color::GRAY;
const RULE_SEPARATION_COLOR: Color<f32> = Color::CYAN;

const ARROW_HEAD_WIDTH: f32 = 0.5;
const ARROW_HEAD_LENGTH: f32 = 2.0;
const ARROW_LENGTH_MAX_FRAC: f32 = 0.5;

const ARROW_DASHED_DASH_LENGTH: f32 = 0.7;
const ARROW_DASHED_SPACE_LENGTH: f32 = 0.3;
const ARROW_DASH_FULL_LENGTH: f32 = ARROW_DASHED_DASH_LENGTH + ARROW_DASHED_SPACE_LENGTH;

const CURVE_RESOLUTION: usize = 5;

const SELECTION_COLOR: Color<f32> = Color {
    r: 0.0,
    g: 0.0,
    b: 0.5,
    a: 0.5,
};
const SELECTED_RADIUS: f32 = 0.5;
const SELECTED_COLOR: Color<f32> = Color {
    r: 0.7,
    g: 0.7,
    b: 0.7,
    a: 0.5,
};

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

fn camera_view(camera: &Camera2d, framebuffer_size: Vec2<f32>) -> AABB<f32> {
    AABB::point(camera.center).extend_symmetric(
        vec2(
            camera.fov / framebuffer_size.y * framebuffer_size.x,
            camera.fov,
        ) / 2.0,
    )
}
