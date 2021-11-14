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
            geng::Event::Wheel { delta } => {
                let delta = -delta as f32 * ZOOM_SPEED;
                let camera = match self.focused_graph {
                    FocusedGraph::Main => &mut self.camera,
                    FocusedGraph::Rule { index } => self.rules.get_camera_mut(index).unwrap(),
                };
                camera.fov = (camera.fov + delta).clamp(CAMERA_FOV_MIN, CAMERA_FOV_MAX);
            }
            _ => (),
        }
    }

    fn drag_start(&mut self, mouse_position: Vec2<f64>, mouse_button: geng::MouseButton) {
        let world_pos = self
            .camera
            .screen_to_world(self.framebuffer_size, mouse_position.map(|x| x as f32));

        let action = match mouse_button {
            mouse
                if mouse == geng::MouseButton::Left
                    && self.geng.window().is_key_pressed(geng::Key::LCtrl)
                    || mouse == geng::MouseButton::Right =>
            {
                // Drag camera
                let (mouse_pos, camera_pos) = match self.focused_graph {
                    FocusedGraph::Main => (mouse_position.map(|x| x as f32), self.camera.center),
                    FocusedGraph::Rule { index } => {
                        let (mouse, _, _) = self.world_to_rule_pos(world_pos, index);
                        (mouse, self.rules.get_camera(index).unwrap().center)
                    }
                };

                let action = DragAction::Move {
                    target: DragTarget::GraphCamera {
                        graph: self.focused_graph,
                        initial_mouse_pos: mouse_pos,
                        initial_camera_pos: camera_pos,
                    },
                };
                Some(action)
            }
            geng::MouseButton::Left => {
                // Drag vertex
                let (graph, graph_pos) = match self.focused_graph {
                    FocusedGraph::Main => (&self.main_graph, world_pos),
                    FocusedGraph::Rule { index } => (
                        self.rules.get_rule(index).unwrap().graph(),
                        self.world_to_rule_pos(world_pos, index).1,
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
                        // Select rule
                        if let &FocusedGraph::Rule { index } = &self.focused_graph {
                            self.selection = Some(RuleSelection::new(
                                index,
                                self.rules.get_rule(index).unwrap(),
                            ));
                        }
                    }
                }
                DragAction::Move { target } => {
                    let delta = world_pos - dragging.world_start_position;
                    if delta.len().approx_eq(&0.0) {
                        if let &DragTarget::Vertex { id, .. } = target {
                            if let Some(selection) = &mut self.selection {
                                if selection.select(id).is_none() {
                                    let selection = self.selection.take().unwrap();
                                    self.apply_rule(selection);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Returns a local screen position, a local world position inside the rule's graph, and its aabb.
    pub fn world_to_rule_pos(
        &self,
        world_pos: Vec2<f32>,
        rule_index: usize,
    ) -> (Vec2<f32>, Vec2<f32>, AABB<f32>) {
        let rule_aabb = self
            .rules
            .layout(&self.camera, self.framebuffer_size)
            .nth(rule_index)
            .unwrap();
        let framebuffer_size = RULE_RESOLUTION.map(|x| x as f32);
        let mut screen_pos =
            (world_pos - rule_aabb.bottom_left()) / vec2(rule_aabb.width(), rule_aabb.height());
        screen_pos *= framebuffer_size;
        let camera = self.rules.get_camera(rule_index).unwrap();
        (
            screen_pos,
            camera.screen_to_world(framebuffer_size, screen_pos),
            camera_view(camera, framebuffer_size),
        )
    }
}
