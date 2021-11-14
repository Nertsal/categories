use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        // Focus
        self.focus();

        // Update graphs
        self.main_graph.update(delta_time);
        self.rules.update(delta_time);

        // Drag
        if let Some(dragging) = &mut self.dragging {
            let mouse_position = self.geng.window().mouse_pos();
            match &dragging.action {
                DragAction::Move { target } => {
                    fn clamp(vec: Vec2<f32>, aabb: AABB<f32>) -> Vec2<f32> {
                        vec2(
                            vec.x.clamp(aabb.x_min, aabb.x_max),
                            vec.y.clamp(aabb.y_min, aabb.y_max),
                        )
                    }

                    let world_pos = self
                        .camera
                        .screen_to_world(self.framebuffer_size, mouse_position.map(|x| x as f32));
                    let updated = match target {
                        &DragTarget::GraphCamera {
                            graph,
                            initial_camera_pos,
                            initial_mouse_pos,
                        } => {
                            let (_, graph_pos, graph_aabb) =
                                self.get_graph_mut(&graph, world_pos).unwrap();
                            let (camera, framebuffer_size) = match graph {
                                FocusedGraph::Main => (&mut self.camera, self.framebuffer_size),
                                FocusedGraph::Rule { index } => (
                                    self.rules.get_camera_mut(index).unwrap(),
                                    RULE_RESOLUTION.map(|x| x as f32),
                                ),
                            };
                            let initial =
                                camera.screen_to_world(framebuffer_size, initial_mouse_pos);
                            let delta = initial - clamp(graph_pos, graph_aabb);
                            camera.center = initial_camera_pos + delta;
                            true
                        }
                        &DragTarget::Vertex { graph, id } => {
                            let (graph, graph_pos, graph_aabb) =
                                self.get_graph_mut(&graph, world_pos).unwrap();
                            graph
                                .graph
                                .vertices
                                .get_mut(&id)
                                .map(|vertex| vertex.body.position = clamp(graph_pos, graph_aabb))
                                .is_some()
                        }
                        &DragTarget::Edge { graph, id } => {
                            let (graph, graph_pos, graph_aabb) =
                                self.get_graph_mut(&graph, world_pos).unwrap();
                            graph
                                .graph
                                .edges
                                .get_mut(&id)
                                .map(|edge| {
                                    edge.get_center_mut().unwrap().position =
                                        clamp(graph_pos, graph_aabb)
                                })
                                .is_some()
                        }
                    };
                    if !updated {
                        self.dragging = None;
                    }
                }
                _ => (),
            }
        }
    }
}
