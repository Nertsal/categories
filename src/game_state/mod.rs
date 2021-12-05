use force_graph::{ForceBody, ForceEdge, ForceParameters, ForceVertex};
use geng::{prelude::rand::thread_rng, Camera2d};

use graphs::{EdgeId, GraphObject, VertexId};

use super::*;

mod action;
mod constants;
mod draw;
mod focus;
mod graph_types;
mod handle_event;
mod rules;
mod selection;
mod update;

use action::*;
use constants::*;
use focus::*;
use graph_types::*;
use rules::*;

pub struct GameState {
    geng: Geng,
    assets: Rc<Assets>,
    camera: Camera2d,
    framebuffer_size: Vec2<f32>,
    main_graph: Graph,
    rules: Rules,
    focused_graph: FocusedGraph,
    dragging: Option<Dragging>,
    selection: Option<RuleSelection>,
    action_history: Vec<GraphActionUndo>,
}

impl GameState {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            dragging: None,
            framebuffer_size: vec2(1.0, 1.0),
            selection: None,
            focused_graph: FocusedGraph::Main,
            action_history: vec![],
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 100.0,
            },
            rules: Rules::new(
                geng,
                assets,
                vec![
                    // Identity: forall (object A) exists (morphism id A->A [Identity])
                    RuleBuilder::new()
                        .forall(ConstraintsBuilder::new().object("A", vec![]).build())
                        .exists(
                            ConstraintsBuilder::new()
                                .morphism("id", "A", "A", vec![MorphismTag::Identity("A")])
                                .build(),
                        )
                        .build(),
                    // Composition: forall (morphism f A->B, morphism g B->C) exists (morphism g.f A->C [Composition f g])
                    RuleBuilder::new()
                        .forall(
                            ConstraintsBuilder::new()
                                .morphism("f", "A", "B", vec![])
                                .morphism("g", "B", "C", vec![])
                                .build(),
                        )
                        .exists(
                            ConstraintsBuilder::new()
                                .morphism(
                                    "g.f",
                                    "A",
                                    "C",
                                    vec![MorphismTag::Composition {
                                        first: "f",
                                        second: "g",
                                    }],
                                )
                                .build(),
                        )
                        .build(),
                    // Product:  forall (object A, object B)
                    //           exists (object AxB [Product A B], morphism _ AxB->A, morphism _ AxB->B)
                    // such that forall (object C, morphism f C->A, morphism g C->B)
                    //           exists (morphism m C->AxB [Unique])
                    //           forall (morphism m' C->AxB)
                    //                  m = m'
                    RuleBuilder::new()
                        .forall(
                            ConstraintsBuilder::new()
                                .object("A", vec![])
                                .object("B", vec![])
                                .build(),
                        )
                        .exists(
                            ConstraintsBuilder::new()
                                .object("AxB", vec![ObjectTag::Product("A", "B")])
                                .morphism("fst", "AxB", "A", vec![])
                                .morphism("snd", "AxB", "B", vec![])
                                .build(),
                        )
                        .forall(
                            ConstraintsBuilder::new()
                                .object("C", vec![])
                                .morphism("f", "C", "A", vec![])
                                .morphism("g", "C", "B", vec![])
                                .build(),
                        )
                        .exists(
                            ConstraintsBuilder::new()
                                .morphism("m", "C", "AxB", vec![MorphismTag::Unique])
                                .build(),
                        )
                        .forall(
                            ConstraintsBuilder::new()
                                .morphism("m'", "C", "AxB", vec![])
                                .build(),
                        )
                        // TODO: m = m'
                        .build(),
                    // // Isomorphism
                    // RuleBuilder {
                    //     inputs: vec![
                    //         RuleObject::edge("f", "1", "2", ArrowConnection::Regular),
                    //         RuleObject::edge("g", "2", "1", ArrowConnection::Regular),
                    //     ],
                    //     constraints: vec![],
                    //     infers: vec![
                    //         RuleObject::edge("id", "1", "1", ArrowConnection::Regular),
                    //         RuleObject::edge("id", "2", "2", ArrowConnection::Regular),
                    //     ],
                    //     removes: vec![
                    //         RuleObject::edge("f", "1", "2", ArrowConnection::Regular), // TODO: Check labels for edges with non-empty names
                    //         RuleObject::edge("g", "2", "1", ArrowConnection::Regular),
                    //     ],
                    //     outputs: vec![RuleObject::edge("", "1", "2", ArrowConnection::Isomorphism)],
                    // }
                    // .build()
                    // .unwrap(),
                ],
            ),
            main_graph: {
                let mut graph = Graph::new(ForceParameters::default());

                let mut objects = HashMap::new();
                let mut morphisms = HashMap::new();

                let mut rng = thread_rng();

                let mut object = |graph: &mut Graph,
                                  objects: &mut HashMap<Label, VertexId>,
                                  morphisms,
                                  label: &str,
                                  tags: Vec<ObjectTag<&str>>,
                                  color: Color<f32>,
                                  anchor: bool| {
                    let new_object = graph.graph.new_vertex(ForceVertex {
                        is_anchor: anchor,
                        body: ForceBody {
                            position: vec2(rng.gen(), rng.gen()),
                            mass: POINT_MASS,
                            velocity: Vec2::ZERO,
                        },
                        vertex: Point {
                            label: label.to_owned(),
                            radius: POINT_RADIUS,
                            tags: tags
                                .into_iter()
                                .map(|tag| tag.map(|o| objects[o]))
                                .collect(),
                            color,
                        },
                    });
                    objects.insert(label.to_owned(), new_object);
                };

                let mut rng = thread_rng();
                let mut morphism =
                    |graph: &mut Graph,
                     objects: &HashMap<Label, VertexId>,
                     morphisms: &mut HashMap<Label, EdgeId>,
                     label: &str,
                     from: &str,
                     to: &str,
                     tags: Vec<MorphismTag<&str, &str>>| {
                        let new_edge = graph.graph.new_edge(ForceEdge::new(
                            vec2(rng.gen(), rng.gen()),
                            vec2(rng.gen(), rng.gen()),
                            ARROW_BODIES,
                            ARROW_MASS,
                            Arrow::new(
                                label,
                                objects[from],
                                objects[to],
                                tags.into_iter()
                                    .map(|tag| tag.map(|o| objects[o], |m| morphisms[m]))
                                    .collect(),
                                ARROW_REGULAR_COLOR,
                                // connection.color(),
                            ),
                        ));
                        morphisms.insert(label.to_owned(), new_edge.unwrap());
                    };

                object(
                    &mut graph,
                    &mut objects,
                    &morphisms,
                    "A",
                    vec![],
                    Color::WHITE,
                    false,
                );
                object(
                    &mut graph,
                    &mut objects,
                    &morphisms,
                    "B",
                    vec![],
                    Color::WHITE,
                    false,
                );
                object(
                    &mut graph,
                    &mut objects,
                    &morphisms,
                    "C",
                    vec![],
                    Color::WHITE,
                    false,
                );
                object(
                    &mut graph,
                    &mut objects,
                    &morphisms,
                    "AxB",
                    vec![ObjectTag::Product("A", "B")],
                    Color::WHITE,
                    false,
                );
                object(
                    &mut graph,
                    &mut objects,
                    &morphisms,
                    "BxC",
                    vec![ObjectTag::Product("B", "C")],
                    Color::WHITE,
                    false,
                );
                object(
                    &mut graph,
                    &mut objects,
                    &morphisms,
                    "(AxB)xC",
                    vec![ObjectTag::Product("AxB", "C")],
                    Color::WHITE,
                    false,
                );
                object(
                    &mut graph,
                    &mut objects,
                    &morphisms,
                    "Ax(BxC)",
                    vec![ObjectTag::Product("A", "BxC")],
                    Color::WHITE,
                    false,
                );

                morphism(&mut graph, &objects, &mut morphisms, "", "AxB", "A", vec![]);
                morphism(&mut graph, &objects, &mut morphisms, "", "AxB", "B", vec![]);
                morphism(&mut graph, &objects, &mut morphisms, "", "BxC", "B", vec![]);
                morphism(&mut graph, &objects, &mut morphisms, "", "BxC", "C", vec![]);
                morphism(
                    &mut graph,
                    &objects,
                    &mut morphisms,
                    "",
                    "(AxB)xC",
                    "AxB",
                    vec![],
                );
                morphism(
                    &mut graph,
                    &objects,
                    &mut morphisms,
                    "",
                    "(AxB)xC",
                    "C",
                    vec![],
                );
                morphism(
                    &mut graph,
                    &objects,
                    &mut morphisms,
                    "",
                    "Ax(BxC)",
                    "A",
                    vec![],
                );
                morphism(
                    &mut graph,
                    &objects,
                    &mut morphisms,
                    "",
                    "Ax(BxC)",
                    "BxC",
                    vec![],
                );

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
    current_mouse_position: Vec2<f64>,
}

enum DragAction {
    Move {
        target: DragTarget,
    },
    Selection {},
    TwoTouchMove {
        initial_camera_fov: f32,
        initial_touch: Vec2<f64>,
        initial_touch_other: Vec2<f64>,
    },
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

fn random_shift() -> Vec2<f32> {
    let mut rng = global_rng();
    vec2(rng.gen(), rng.gen())
}

fn camera_view(camera: &Camera2d, framebuffer_size: Vec2<f32>) -> AABB<f32> {
    AABB::point(camera.center).extend_symmetric(
        vec2(
            camera.fov / framebuffer_size.y * framebuffer_size.x,
            camera.fov,
        ) / 2.0,
    )
}
