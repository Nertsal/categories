use super::*;

impl GameState {
    pub fn handle_event_impl(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::Space => {
                    // Anchor vertex
                    if let Some(dragging) = &self.dragging {
                        if let DragAction::Move {
                            target: DragTarget::Vertex { graph, id },
                        } = &dragging.action
                        {
                            let graph = *graph;
                            let id = *id;
                            let vertex = self
                                .get_graph_mut(&graph, Vec2::ZERO)
                                .unwrap()
                                .0
                                .graph
                                .vertices
                                .get_mut(&id)
                                .unwrap();
                            vertex.is_anchor = !vertex.is_anchor;
                        }
                    }
                }
                geng::Key::Escape => {
                    // Clear selection
                    self.selection = None;
                }
                geng::Key::Z if self.geng.window().is_key_pressed(geng::Key::LCtrl) => {
                    self.action_undo();
                }
                _ => (),
            },
            geng::Event::MouseDown { position, button } => {
                self.drag_start(position, button);
            }
            geng::Event::MouseMove { position, .. } => {
                self.drag_move(position);
            }
            geng::Event::MouseUp { position, button } => {
                self.drag_stop(position, button);
            }
            geng::Event::Wheel { delta } => {
                if self.geng.window().is_key_pressed(geng::Key::LCtrl) {
                    // Zoom
                    let delta = -delta as f32 * ZOOM_SPEED;
                    let camera = self.focused_camera_mut();
                    camera.fov = (camera.fov + delta).clamp(CAMERA_FOV_MIN, CAMERA_FOV_MAX);
                } else {
                    // Scroll
                    let delta = -delta as f32 * SCROLL_SPEED;
                    self.rules.scroll(delta);
                }
            }
            geng::Event::TouchStart { touches } => match &touches[..] {
                [touch] => {
                    self.drag_start(touch.position, geng::MouseButton::Left);
                }
                [touch0, touch1] => {
                    self.focus(touch0.position);
                    let camera = self.focused_camera();
                    let world_pos = camera
                        .screen_to_world(self.framebuffer_size, touch0.position.map(|x| x as f32));
                    self.dragging = Some(Dragging {
                        mouse_start_position: touch0.position,
                        world_start_position: world_pos,
                        action: DragAction::TwoTouchMove {
                            initial_camera_fov: camera.fov,
                            // initial_camera_rotation: camera.rotation,
                            initial_touch: touch0.position,
                            initial_touch_other: touch1.position,
                        },
                    })
                }
                _ => (),
            },
            geng::Event::TouchMove { touches } => match &touches[..] {
                [touch] => {
                    self.drag_move(touch.position);
                }
                [touch0, touch1] => {
                    if let Some(dragging) = &self.dragging {
                        if let &DragAction::TwoTouchMove {
                            // initial_camera_rotation,
                            initial_camera_fov,
                            initial_touch,
                            initial_touch_other,
                        } = &dragging.action
                        {
                            let framebuffer_size = self.framebuffer_size;
                            let camera = self.focused_camera_mut();

                            let initial_delta = camera
                                .screen_to_world(framebuffer_size, initial_touch.map(|x| x as f32))
                                - camera.screen_to_world(
                                    framebuffer_size,
                                    initial_touch_other.map(|x| x as f32),
                                );

                            let initial_distance = initial_delta.len();
                            // let initial_angle = initial_delta.arg();

                            let delta = camera.screen_to_world(
                                framebuffer_size,
                                touch0.position.map(|x| x as f32),
                            ) - camera.screen_to_world(
                                framebuffer_size,
                                touch1.position.map(|x| x as f32),
                            );

                            let distance = delta.len();
                            // let angle = delta.arg();

                            camera.fov = (initial_camera_fov / distance * initial_distance)
                                .clamp(CAMERA_FOV_MIN, CAMERA_FOV_MAX);
                            // camera.rotation = initial_camera_rotation + angle - initial_angle;
                        }
                    }
                }
                _ => (),
            },
            geng::Event::TouchEnd => {
                // TODO: Detect short and long taps
                self.dragging = None;
                self.focused_graph = FocusedGraph::Main;
            }
            _ => (),
        }
    }

    fn drag_start(&mut self, mouse_position: Vec2<f64>, mouse_button: geng::MouseButton) {
        // Focus
        self.focus(mouse_position);

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
                    .unwrap_or_else(|| DragAction::Selection {
                        current_mouse_position: mouse_position,
                    });
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

    fn drag_move(&mut self, mouse_position: Vec2<f64>) {
        // Focus
        self.focus(mouse_position);

        // Drag
        if let Some(dragging) = &mut self.dragging {
            match &mut dragging.action {
                DragAction::Move { target } => {
                    let world_pos = self
                        .camera
                        .screen_to_world(self.framebuffer_size, mouse_position.map(|x| x as f32));
                    let updated = match target {
                        &mut DragTarget::GraphCamera {
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
                            let delta = initial - graph_pos.clamp_aabb(graph_aabb);
                            camera.center = initial_camera_pos + delta;
                            true
                        }
                        &mut DragTarget::Vertex { graph, id } => {
                            let (graph, graph_pos, graph_aabb) =
                                self.get_graph_mut(&graph, world_pos).unwrap();
                            graph
                                .graph
                                .vertices
                                .get_mut(&id)
                                .map(|vertex| {
                                    vertex.body.position = graph_pos.clamp_aabb(graph_aabb)
                                })
                                .is_some()
                        }
                        &mut DragTarget::Edge { graph, id } => {
                            let (graph, graph_pos, graph_aabb) =
                                self.get_graph_mut(&graph, world_pos).unwrap();
                            graph
                                .graph
                                .edges
                                .get_mut(&id)
                                .map(|edge| {
                                    edge.get_center_mut().unwrap().position =
                                        graph_pos.clamp_aabb(graph_aabb)
                                })
                                .is_some()
                        }
                    };
                    if !updated {
                        self.dragging = None;
                    }
                }
                DragAction::Selection {
                    current_mouse_position,
                } => {
                    *current_mouse_position = mouse_position;
                }
                _ => (),
            }
        }
    }

    fn drag_stop(&mut self, mouse_position: Vec2<f64>, _mouse_button: geng::MouseButton) {
        if let Some(dragging) = self.dragging.take() {
            let world_pos = self
                .camera
                .screen_to_world(self.framebuffer_size, mouse_position.map(|x| x as f32));
            match &dragging.action {
                DragAction::Selection { .. } => {
                    let dragged_delta = mouse_position - dragging.mouse_start_position;
                    if dragged_delta.len().approx_eq(&0.0) {
                        // Select rule
                        if let &FocusedGraph::Rule { index } = &self.focused_graph {
                            self.selection =
                                Some(RuleSelection::new(&self.main_graph, index, &self.rules));
                        }
                    }
                }
                DragAction::Move { target } => {
                    let delta = world_pos - dragging.world_start_position;
                    if delta.len().approx_eq(&0.0) {
                        let selected = match target {
                            &DragTarget::Vertex { graph, id } if graph.is_main() => {
                                Some(GraphObject::Vertex { id })
                            }
                            &DragTarget::Edge { graph, id } if graph.is_main() => {
                                Some(GraphObject::Edge { id })
                            }
                            _ => None,
                        };
                        if let Some(selection) = &mut self.selection {
                            if let Some(selected) = selected {
                                if let Some(options) = selection.inferred_options() {
                                    if options.contains(&selected)
                                        && selection
                                            .select(&self.main_graph, selected, &self.rules)
                                            .is_none()
                                    {
                                        let selection = self.selection.take().unwrap();
                                        self.apply_rule(selection);
                                    }
                                } else {
                                    // No possible selection options
                                    self.selection = None;
                                }
                            }
                        }
                    }
                }
                _ => (),
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
