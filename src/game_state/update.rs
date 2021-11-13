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
                    let world_pos = self
                        .camera
                        .screen_to_world(self.framebuffer_size, mouse_position.map(|x| x as f32));
                    let updated = match target {
                        &DragTarget::Vertex { graph, id } => {
                            let (graph, graph_pos) = self.get_graph_mut(&graph, world_pos).unwrap();
                            graph
                                .graph
                                .vertices
                                .get_mut(&id)
                                .map(|vertex| vertex.body.position = graph_pos)
                                .is_some()
                        }
                        &DragTarget::Edge { graph, id } => {
                            let (graph, graph_pos) = self.get_graph_mut(&graph, world_pos).unwrap();
                            graph
                                .graph
                                .edges
                                .get_mut(&id)
                                .map(|edge| edge.get_center_mut().unwrap().position = graph_pos)
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
