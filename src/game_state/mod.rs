use geng::Camera2d;

use graphs::{EdgeId, VertexId};

use super::*;

mod draw;
mod graph_types;
mod handle_event;
mod selection;

use graph_types::*;
use selection::*;

pub struct GameState {
    geng: Geng,
    camera: Camera2d,
    framebuffer_size: Vec2<f32>,
    graph: Graph,
    dragging: Option<Dragging>,
    selection: Selection,
}

impl GameState {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            dragging: None,
            framebuffer_size: vec2(1.0, 1.0),
            selection: Selection::new(),
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 100.0,
            },
            graph: {
                let mut graph = Graph::new();

                let mut point = |position: Vec2<f32>, color: Color<f32>| {
                    graph.new_vertex(Point {
                        position,
                        radius: 1.0,
                        color,
                    })
                };

                let vertices = vec![
                    point(vec2(-10.0, 0.0), Color::WHITE),
                    point(vec2(0.0, 0.0), Color::GREEN),
                    point(vec2(10.0, 0.0), Color::WHITE),
                    point(vec2(0.0, 10.0), Color::BLUE),
                ];

                let mut connect =
                    |from: usize, to: usize, color: Color<f32>, connection: ArrowConnection| {
                        graph.add_edge(Arrow {
                            from: vertices[from],
                            to: vertices[to],
                            width: 0.2,
                            color,
                            connection,
                        })
                    };

                connect(1, 0, Color::GREEN, ArrowConnection::Solid);
                connect(1, 2, Color::GREEN, ArrowConnection::Solid);
                connect(3, 0, Color::BLUE, ArrowConnection::Solid);
                connect(3, 2, Color::BLUE, ArrowConnection::Solid);
                connect(3, 1, Color::RED, ArrowConnection::Dashed);

                graph
            },
        }
    }
}

impl geng::State for GameState {
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
