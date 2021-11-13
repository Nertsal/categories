use super::*;

impl GameState {
    pub fn handle_event_impl(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseDown { position, button } => {
                self.drag_start(position, button);
            }
            geng::Event::MouseMove { position, delta } => {
                self.drag_move(position, delta);
            }
            geng::Event::MouseUp { position, button } => {
                self.drag_stop(position, button);
            }
            _ => (),
        }
    }

    fn drag_start(&mut self, mouse_position: Vec2<f64>, mouse_button: geng::MouseButton) {
        let world_pos = self
            .camera
            .screen_to_world(self.framebuffer_size, mouse_position.map(|x| x as f32));

        let action = match mouse_button {
            geng::MouseButton::Left => {
                // Drag vertex
                let (graph, graph_pos) = match self.focused_graph {
                    FocusedGraph::Main => (&self.main_graph, world_pos),
                    FocusedGraph::Rule { index } => (
                        self.rules.get_rule(index).unwrap().graph(),
                        self.world_to_rule_pos(world_pos, index),
                    ),
                };

                let action = Self::vertices_under_point(graph, graph_pos)
                    .next()
                    .map(|(&id, _)| DragAction::Move {
                        target: DragTarget::Vertex {
                            id,
                            graph: self.focused_graph,
                        },
                    })
                    .or_else(|| {
                        // Drag edge
                        Self::edges_under_point(graph, graph_pos)
                            .next()
                            .map(|(&id, _)| DragAction::Move {
                                target: DragTarget::Edge {
                                    id,
                                    graph: self.focused_graph,
                                },
                            })
                    })
                    .unwrap_or_else(|| DragAction::Selection);
                Some(action)
            }
            _ => None,
        };

        self.dragging = action.map(|action| Dragging {
            mouse_start_position: mouse_position,
            world_start_position: world_pos,
            action,
        });
    }

    fn drag_move(&mut self, _mouse_position: Vec2<f64>, _mouse_delta: Vec2<f64>) {}

    fn drag_stop(&mut self, mouse_position: Vec2<f64>, _mouse_button: geng::MouseButton) {
        if let Some(dragging) = self.dragging.take() {
            let world_pos = self
                .camera
                .screen_to_world(self.framebuffer_size, mouse_position.map(|x| x as f32));
            match &dragging.action {
                DragAction::Selection => {
                    let dragged_delta = mouse_position - dragging.mouse_start_position;
                    if dragged_delta.len().approx_eq(&0.0) {
                        // Click
                        self.select_point(dragging.world_start_position, SelectionOptions::New);
                    } else {
                        // Drag
                        self.select_area(
                            AABB::from_corners(dragging.world_start_position, world_pos),
                            SelectionOptions::New,
                        );
                    }
                }
                DragAction::Move { target } => {
                    let delta = world_pos - dragging.world_start_position;
                    if delta.len().approx_eq(&0.0) {
                        // Select
                        let (vertices, edges) = match target {
                            DragTarget::Vertex { graph, id } if graph.is_main() => {
                                (vec![*id], vec![])
                            }
                            DragTarget::Edge { graph, id } if graph.is_main() => {
                                (vec![], vec![*id])
                            }
                            _ => return,
                        };
                        self.select(vertices, edges, SelectionOptions::New);
                    }
                }
            }
        }
    }

    pub fn world_to_rule_pos(&self, world_pos: Vec2<f32>, rule_index: usize) -> Vec2<f32> {
        let rule_aabb = self
            .rules
            .layout(&self.camera, self.framebuffer_size)
            .nth(rule_index)
            .unwrap();
        let framebuffer_size = RULE_RESOLUTION.map(|x| x as f32);
        let mut screen_pos =
            (world_pos - rule_aabb.bottom_left()) / vec2(rule_aabb.width(), rule_aabb.height());
        // screen_pos.y *= -1.0;
        screen_pos *= framebuffer_size;
        let camera = self.rules.get_camera(rule_index).unwrap();
        camera.screen_to_world(framebuffer_size, screen_pos)
    }

    fn select_point(&mut self, position: Vec2<f32>, options: SelectionOptions) {
        // Vertices
        let selected_vertices = Self::vertices_under_point(&self.main_graph, position)
            .map(|(&id, _)| id)
            .collect();

        // Edges
        let selected_edges = Self::edges_under_point(&self.main_graph, position)
            .map(|(&id, _)| id)
            .collect();

        // Add to selection
        self.select(selected_vertices, selected_edges, options);
    }

    fn select_area(&mut self, area: AABB<f32>, options: SelectionOptions) {
        // Vertices
        let selected_vertices = self.vertices_in_area(area).map(|(&id, _)| id).collect();

        // Edges
        let selected_edges = Self::edges_in_area(&self.main_graph, area)
            .map(|(&id, _)| id)
            .collect();

        // Add to selection
        self.select(selected_vertices, selected_edges, options);
    }

    fn select(&mut self, vertices: Vec<VertexId>, edges: Vec<EdgeId>, options: SelectionOptions) {
        match options {
            SelectionOptions::New => {
                self.selection.clear_all();
                self.selection.select_vertices(vertices.into_iter());
                self.selection.select_edges(edges.into_iter());
            }
            SelectionOptions::Add => {
                self.selection.select_vertices(vertices.into_iter());
                self.selection.select_edges(edges.into_iter());
            }
            SelectionOptions::Change => {
                self.selection.change_vertices(vertices.into_iter());
                self.selection.change_edges(edges.into_iter());
            }
        }

        self.apply_rule(0);
    }
}

enum SelectionOptions {
    /// Clear previous selection
    New,
    /// Add the items in the area to the selection
    Add,
    /// Change the selection of the items in the area
    Change,
}
