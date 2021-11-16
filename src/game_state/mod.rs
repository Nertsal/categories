use force_graph::{ForceBody, ForceEdge, ForceParameters, ForceVertex};
use geng::{prelude::rand::thread_rng, Camera2d};

use graphs::{EdgeId, VertexId};

use super::*;

mod chain;
mod constants;
mod curve;
mod draw;
mod focus;
mod graph_types;
mod handle_event;
mod rules;
mod selection;
mod update;

use chain::*;
use constants::*;
use curve::*;
use focus::*;
use graph_types::*;
use rules::*;

pub struct GameState {
    geng: Geng,
    camera: Camera2d,
    framebuffer_size: Vec2<f32>,
    main_graph: Graph,
    rules: Rules,
    focused_graph: FocusedGraph,
    dragging: Option<Dragging>,
    selection: Option<RuleSelection>,
}

impl GameState {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            dragging: None,
            framebuffer_size: vec2(1.0, 1.0),
            selection: None,
            focused_graph: FocusedGraph::Main,
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 100.0,
            },
            rules: Rules::new(
                geng,
                vec![
                    // Regular composition
                    RuleBuilder::new(3)
                        .with_edge_constraints(
                            [
                                ArrowConstraint::new(0, 1, ArrowConnection::Regular),
                                ArrowConstraint::new(1, 2, ArrowConnection::Regular),
                            ]
                            .into_iter(),
                        )
                        .with_new_edges(
                            [Arrow::new("", 0, 2, ArrowConnection::Regular)].into_iter(),
                        )
                        .build(),
                    // Best Cone
                    RuleBuilder::new(4)
                        .with_edge_constraints(
                            [
                                ArrowConstraint::new(2, 0, ArrowConnection::Best),
                                ArrowConstraint::new(2, 1, ArrowConnection::Best),
                                ArrowConstraint::new(3, 0, ArrowConnection::Regular),
                                ArrowConstraint::new(3, 1, ArrowConnection::Regular),
                            ]
                            .into_iter(),
                        )
                        .with_new_edges([Arrow::new("", 3, 2, ArrowConnection::Unique)].into_iter())
                        .build(),
                ],
            ),
            main_graph: {
                let mut graph = Graph::new(ForceParameters::default());

                let mut rng = thread_rng();

                let mut point = |label: &str, color: Color<f32>, anchor: bool| {
                    graph.graph.new_vertex(ForceVertex {
                        is_anchor: anchor,
                        body: ForceBody {
                            position: vec2(rng.gen(), rng.gen()),
                            mass: POINT_MASS,
                            velocity: Vec2::ZERO,
                        },
                        vertex: Point {
                            label: label.to_owned(),
                            radius: POINT_RADIUS,
                            color,
                        },
                    })
                };

                let vertices = vec![
                    point("A", Color::WHITE, false),
                    point("B", Color::WHITE, false),
                    point("C", Color::WHITE, false),
                    point("AxB", Color::WHITE, false),
                    point("BxC", Color::WHITE, false),
                    point("(AxB)xC", Color::WHITE, false),
                    point("Ax(BxC)", Color::WHITE, false),
                ];

                let mut connect =
                    |label: &str, from: usize, to: usize, connection: ArrowConnection| {
                        graph.graph.new_edge(ForceEdge::new(
                            vec2(rng.gen(), rng.gen()),
                            vec2(rng.gen(), rng.gen()),
                            ARROW_BODIES,
                            ARROW_MASS,
                            Arrow::new(label, vertices[from], vertices[to], connection),
                        ))
                    };

                connect("", 3, 0, ArrowConnection::Best);
                connect("", 3, 1, ArrowConnection::Best);
                connect("", 4, 1, ArrowConnection::Best);
                connect("", 4, 2, ArrowConnection::Best);
                connect("", 5, 3, ArrowConnection::Best);
                connect("", 5, 2, ArrowConnection::Best);
                connect("", 6, 0, ArrowConnection::Best);
                connect("", 6, 4, ArrowConnection::Best);

                graph
            },
        }
    }
}

impl geng::State for GameState {
    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;
        self.update_impl(delta_time);
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.draw_impl(framebuffer);
    }

    fn handle_event(&mut self, event: geng::Event) {
        self.handle_event_impl(event);
    }
}

struct Dragging {
    mouse_start_position: Vec2<f64>,
    world_start_position: Vec2<f32>,
    action: DragAction,
}

enum DragAction {
    Move { target: DragTarget },
    Selection,
}

enum DragTarget {
    GraphCamera {
        graph: FocusedGraph,
        initial_mouse_pos: Vec2<f32>,
        initial_camera_pos: Vec2<f32>,
    },
    Vertex {
        graph: FocusedGraph,
        id: VertexId,
    },
    Edge {
        graph: FocusedGraph,
        id: EdgeId,
    },
}

fn camera_view(camera: &Camera2d, framebuffer_size: Vec2<f32>) -> AABB<f32> {
    AABB::point(camera.center).extend_symmetric(
        vec2(
            camera.fov / framebuffer_size.y * framebuffer_size.x,
            camera.fov,
        ) / 2.0,
    )
}
