use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        self.force_graph.update(delta_time);
        for (rule, _) in &mut self.rules {
            rule.update_graph(delta_time);
        }

        if let Some(dragging) = &mut self.dragging {
            let mouse_position = self.geng.window().mouse_pos();
            match &dragging.action {
                DragAction::MoveVertex { vertex } => {
                    let world_pos = self
                        .camera
                        .screen_to_world(self.framebuffer_size, mouse_position.map(|x| x as f32));
                    match self.force_graph.graph.vertices.get_mut(vertex) {
                        Some(vertex) => {
                            vertex.body.position = world_pos;
                        }
                        None => {
                            self.dragging = None;
                        }
                    }
                }
                DragAction::MoveEdge { edge } => {
                    let world_pos = self
                        .camera
                        .screen_to_world(self.framebuffer_size, mouse_position.map(|x| x as f32));
                    match self.force_graph.graph.edges.get_mut(edge) {
                        Some(edge) => {
                            edge.get_center_mut().unwrap().position = world_pos;
                        }
                        None => {
                            self.dragging = None;
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
