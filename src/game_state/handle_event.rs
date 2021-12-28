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
                                .get_graph_mut(&graph)
                                .unwrap()
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
                    self.main_selection = None;
                    self.goal_selection = None;
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
                    self.state.scroll_rules(delta, self.rules.len());
                }
            }
            geng::Event::TouchStart { touches } => match &touches[..] {
                [touch] => {
                    self.drag_start(touch.position, geng::MouseButton::Left);
                }
                [touch0, touch1] => {
                    self.focus(touch0.position);
                    let camera = self.focused_camera();
                    let world_pos = camera.screen_to_world(
                        self.state.framebuffer_size,
                        touch0.position.map(|x| x as f32),
                    );
                    self.dragging = Some(Dragging {
                        mouse_start_position: touch0.position,
                        current_mouse_position: touch0.position,
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
                            let framebuffer_size = self.state.framebuffer_size;
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

        let world_pos = self.ui_camera.screen_to_world(
            self.state.framebuffer_size,
            mouse_position.map(|x| x as f32),
        );

        let action = match mouse_button {
            mouse
                if mouse == geng::MouseButton::Left
                    && self.geng.window().is_key_pressed(geng::Key::LCtrl)
                    || mouse == geng::MouseButton::Right =>
            {
                // Drag camera
                let focused_graph = self.focused_graph;
                self.world_to_graph_pos(&focused_graph, world_pos)
                    .and_then(|(screen_pos, _, _)| {
                        self.get_graph_camera_mut(&focused_graph)
                            .map(|(camera, _)| DragAction::Move {
                                target: DragTarget::GraphCamera {
                                    graph: focused_graph,
                                    initial_mouse_pos: screen_pos,
                                    initial_camera_pos: camera.center,
                                },
                            })
                    })
            }
            geng::MouseButton::Left => {
                // Drag vertex
                let focused_graph = self.focused_graph;
                self.world_to_graph(&focused_graph, world_pos)
                    .map(|(graph, graph_pos, _)| {
                        Self::vertices_under_point(graph, graph_pos)
                            .next()
                            .map(|(&id, _)| DragAction::Move {
                                target: DragTarget::Vertex {
                                    id,
                                    graph: focused_graph,
                                },
                            })
                            .or_else(|| {
                                // Drag edge
                                Self::edges_under_point(graph, graph_pos)
                                    .next()
                                    .map(|(&id, _)| DragAction::Move {
                                        target: DragTarget::Edge {
                                            id,
                                            graph: focused_graph,
                                        },
                                    })
                            })
                            .unwrap_or_else(|| DragAction::Selection {})
                    })
            }
            _ => None,
        };

        self.dragging = action.map(|action| Dragging {
            mouse_start_position: mouse_position,
            world_start_position: world_pos,
            current_mouse_position: mouse_position,
            action,
        });
    }

    fn drag_move(&mut self, mouse_position: Vec2<f64>) {
        // Focus
        self.focus(mouse_position);
        if let Some(dragging) = &mut self.dragging {
            dragging.current_mouse_position = mouse_position;
        }
    }

    pub fn drag_update(&mut self) {
        // Drag
        if let Some(dragging) = &mut self.dragging {
            match &mut dragging.action {
                DragAction::Move { target } => {
                    let world_pos = self.ui_camera.screen_to_world(
                        self.state.framebuffer_size,
                        dragging.current_mouse_position.map(|x| x as f32),
                    );
                    let updated = match target {
                        &mut DragTarget::GraphCamera {
                            graph,
                            initial_camera_pos,
                            initial_mouse_pos,
                        } => self
                            .world_to_graph(&graph, world_pos)
                            .map(|(_, graph_pos, graph_aabb)| (graph_pos, graph_aabb))
                            .and_then(|(graph_pos, graph_aabb)| {
                                self.get_graph_camera_mut(&graph).map(
                                    |(camera, framebuffer_size)| {
                                        let initial = camera.screen_to_world(
                                            framebuffer_size.map(|x| x as f32),
                                            initial_mouse_pos,
                                        );
                                        let delta = initial - graph_pos.clamp_aabb(graph_aabb);
                                        camera.center = initial_camera_pos + delta;
                                    },
                                )
                            })
                            .is_some(),
                        &mut DragTarget::Vertex { graph, id } => self
                            .world_to_graph(&graph, world_pos)
                            .and_then(|(graph, graph_pos, graph_aabb)| {
                                graph.graph.vertices.get_mut(&id).map(|vertex| {
                                    vertex.body.position = graph_pos.clamp_aabb(graph_aabb);
                                })
                            })
                            .is_some(),
                        &mut DragTarget::Edge { graph, id } => self
                            .world_to_graph(&graph, world_pos)
                            .and_then(|(graph, graph_pos, graph_aabb)| {
                                graph.graph.edges.get_mut(&id).map(|edge| {
                                    edge.get_center_mut().unwrap().position =
                                        graph_pos.clamp_aabb(graph_aabb)
                                })
                            })
                            .is_some(),
                    };
                    if !updated {
                        self.dragging = None;
                    }
                }
                _ => (),
            }
        }
    }

    fn drag_stop(&mut self, mouse_position: Vec2<f64>, _mouse_button: geng::MouseButton) {
        if let Some(dragging) = self.dragging.take() {
            let world_pos = self.ui_camera.screen_to_world(
                self.state.framebuffer_size,
                mouse_position.map(|x| x as f32),
            );
            match &dragging.action {
                DragAction::Selection { .. } => {
                    let dragged_delta = mouse_position - dragging.mouse_start_position;
                    if dragged_delta.len().approx_eq(&0.0) {
                        // Select rule
                        if let &FocusedGraph::Rule { index } = &self.focused_graph {
                            let main_selection = RuleSelection::new(
                                &self.main_graph.graph,
                                index,
                                &self.rules,
                                false,
                            );
                            let goal_selection = RuleSelection::new(
                                &self.goal_graph.graph,
                                index,
                                &self.rules,
                                true,
                            );
                            match main_selection.current() {
                                Some(_) => {
                                    self.main_selection = Some(main_selection);
                                }
                                None => {
                                    self.apply_rule(FocusedGraph::Main, main_selection);
                                }
                            }
                            match goal_selection.current() {
                                Some(_) => {
                                    self.goal_selection = Some(goal_selection);
                                }
                                None => {
                                    self.apply_rule(FocusedGraph::Goal, goal_selection);
                                }
                            }
                        }
                    }
                }
                DragAction::Move { target } => {
                    let delta = world_pos - dragging.world_start_position;
                    if delta.len().approx_eq(&0.0) {
                        // Select vertex or edge
                        let selected = match target {
                            &DragTarget::Vertex { graph, id } => {
                                Some((graph, GraphObject::Vertex { id }))
                            }
                            &DragTarget::Edge { graph, id } => {
                                Some((graph, GraphObject::Edge { id }))
                            }
                            _ => None,
                        };

                        if let Some((focused_graph, selected)) = selected {
                            let selection = match focused_graph {
                                FocusedGraph::Rule { .. } => None,
                                FocusedGraph::Main => {
                                    Some((&self.main_graph.graph, &mut self.main_selection))
                                }
                                FocusedGraph::Goal => {
                                    Some((&self.goal_graph.graph, &mut self.goal_selection))
                                }
                            };

                            if let Some((graph, selection)) = selection {
                                match selection
                                    .as_ref()
                                    .and_then(|selection| selection.inferred_options().as_ref())
                                {
                                    Some(options) => {
                                        if options.contains(&selected)
                                            && selection
                                                .as_mut()
                                                .unwrap()
                                                .select(graph, selected, &self.rules)
                                                .is_none()
                                        {
                                            let selection = selection.take().unwrap();
                                            self.apply_rule(focused_graph, selection);
                                        }
                                    }
                                    None => {
                                        *selection = None;
                                    }
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
