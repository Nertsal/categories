use geng::Camera2d;

use super::*;

mod draw;

type Graph = graphs::Graph<Point, Arrow>;

pub struct GameState {
    geng: Geng,
    camera: Camera2d,
    graph: Graph,
}

impl GameState {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
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

                let mut connect = |from: usize, to: usize, color: Color<f32>| {
                    graph.add_edge(Arrow {
                        from: vertices[from],
                        to: vertices[to],
                        width: 1.0,
                        color,
                    })
                };

                connect(1, 0, Color::GREEN);
                connect(1, 2, Color::GREEN);
                connect(3, 0, Color::BLUE);
                connect(3, 2, Color::BLUE);
                connect(3, 1, Color::RED);

                graph
            },
        }
    }
}

impl geng::State for GameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.draw_impl(framebuffer);
    }
}

#[derive(Debug, Clone)]
struct Point {
    position: Vec2<f32>,
    radius: f32,
    color: Color<f32>,
}

#[derive(Debug, Clone, PartialEq)]
struct Arrow {
    from: graphs::VertexId,
    to: graphs::VertexId,
    color: Color<f32>,
    width: f32,
}

impl graphs::GraphEdge for Arrow {
    fn end_points(&self) -> [&graphs::VertexId; 2] {
        [&self.from, &self.to]
    }
}
