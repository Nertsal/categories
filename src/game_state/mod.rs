use force_graph::{ForceBody, ForceEdge, ForceParameters, ForceVertex};
use geng::Camera2d;

use graphs::{EdgeId, VertexId};

use super::*;

mod constants;
mod draw;
mod graph_types;
mod handle_event;
mod rule;
mod selection;

use constants::*;
use graph_types::*;
use rule::*;
use selection::*;

pub struct GameState {
    geng: Geng,
    camera: Camera2d,
    framebuffer_size: Vec2<f32>,
    force_graph: Graph,
    dragging: Option<Dragging>,
    selection: Selection,
    rules: Vec<Rule>,
}

impl GameState {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            dragging: None,
            framebuffer_size: vec2(1.0, 1.0),
            selection: Selection::new(),
            rules: vec![Rule::new(
                2,
                vec![],
                1,
                vec![
                    Arrow {
                        from: 2,
                        to: 0,
                        color: Color::GREEN,
                        width: ARROW_WIDTH,
                        connection: ArrowConnection::Solid,
                    },
                    Arrow {
                        from: 2,
                        to: 1,
                        color: Color::GREEN,
                        width: ARROW_WIDTH,
                        connection: ArrowConnection::Solid,
                    },
                ],
            )
            .unwrap()],
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 100.0,
            },
            force_graph: {
                let mut graph = Graph::new(ForceParameters::default());

                let mut point = |position: Vec2<f32>, color: Color<f32>, anchor: bool| {
                    (
                        position,
                        graph.graph.new_vertex(ForceVertex {
                            is_anchor: anchor,
                            body: ForceBody {
                                position,
                                mass: POINT_MASS,
                                velocity: Vec2::ZERO,
                            },
                            vertex: Point {
                                radius: POINT_RADIUS,
                                color,
                            },
                        }),
                    )
                };

                let vertices = vec![
                    point(vec2(-10.0, 0.0), Color::WHITE, false),
                    point(vec2(0.0, 0.0), Color::GREEN, true),
                    point(vec2(10.0, 0.0), Color::WHITE, false),
                    point(vec2(0.0, 10.0), Color::BLUE, false),
                ];

                let mut connect =
                    |from: usize, to: usize, color: Color<f32>, connection: ArrowConnection| {
                        graph.graph.new_edge(ForceEdge {
                            body: ForceBody {
                                position: (vertices[from].0 + vertices[to].0) / 2.0,
                                mass: ARROW_MASS,
                                velocity: Vec2::ZERO,
                            },
                            edge: Arrow {
                                from: vertices[from].1,
                                to: vertices[to].1,
                                width: ARROW_WIDTH,
                                color,
                                connection,
                            },
                        })
                    };

                connect(1, 0, Color::GREEN, ArrowConnection::Solid);
                connect(1, 2, Color::GREEN, ArrowConnection::Solid);
                connect(3, 0, Color::BLUE, ArrowConnection::Solid);
                connect(3, 2, Color::BLUE, ArrowConnection::Solid);
                connect(3, 1, Color::RED, ArrowConnection::Dashed);
                connect(3, 1, Color::RED, ArrowConnection::Dashed);

                graph
            },
        }
    }
}

impl geng::State for GameState {
    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;
        self.force_graph.update(delta_time);
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        self.draw_impl(framebuffer);
    }

    fn handle_event(&mut self, event: geng::Event) {
        self.handle_event_impl(event);
    }
}

struct Dragging {
    mouse_start_pos: Vec2<f64>,
    world_start_pos: Vec2<f32>,
    mouse_button: geng::MouseButton,
}
