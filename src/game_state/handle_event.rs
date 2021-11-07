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
                let action = self
                    .vertices_under_point(world_pos)
                    .next()
                    .map(|(&vertex, _)| DragAction::MoveVertex { vertex })
                    .or_else(|| {
                        // Drag edge
                        self.edges_under_point(world_pos)
                            .next()
                            .map(|(&edge, _)| DragAction::MoveEdge { edge })
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
                DragAction::MoveEdge { edge } => {
                    let delta = world_pos - dragging.world_start_position;
                    if delta.len().approx_eq(&0.0) {
                        // Select
                        self.select(vec![], vec![*edge], SelectionOptions::New);
                    }
                }
                DragAction::MoveVertex { vertex } => {
                    let delta = world_pos - dragging.world_start_position;
                    if delta.len().approx_eq(&0.0) {
                        // Select
                        self.select(vec![*vertex], vec![], SelectionOptions::New);
                    }
                }
            }
        }
    }

    fn select_point(&mut self, position: Vec2<f32>, options: SelectionOptions) {
        // Vertices
        let selected_vertices = self
            .vertices_under_point(position)
            .map(|(&id, _)| id)
            .collect();

        // Edges
        let selected_edges = self
            .edges_under_point(position)
            .map(|(&id, _)| id)
            .collect();

        // Add to selection
        self.select(selected_vertices, selected_edges, options);
    }

    fn select_area(&mut self, area: AABB<f32>, options: SelectionOptions) {
        // Vertices
        let selected_vertices = self.vertices_in_area(area).map(|(&id, _)| id).collect();

        // Edges
        let selected_edges = self.edges_in_area(area).map(|(&id, _)| id).collect();

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
