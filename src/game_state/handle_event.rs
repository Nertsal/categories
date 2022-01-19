use super::*;

impl GameState {
    pub fn handle_event_impl(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::Space => {
                    // Anchor vertex
                    if let Some(dragging) = &self.dragging {
                        if let DragAction::Move {
                            target: DragTarget::Vertex { category, id },
                        } = &dragging.action
                        {
                            let category = *category;
                            let id = *id;
                            todo!()
                            // let vertex = self
                            //     .get_category_mut(&graph)
                            //     .unwrap()
                            //     .vertices
                            //     .get_mut(&id)
                            //     .unwrap();
                            // vertex.is_anchor = !vertex.is_anchor;
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
                self.focused_category = FocusedCategory::Fact;
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
                let focused_category = self.focused_category;
                self.world_to_category_pos(&focused_category, world_pos)
                    .and_then(|(screen_pos, _, _)| {
                        self.get_category_camera_mut(&focused_category)
                            .map(|(camera, _)| DragAction::Move {
                                target: DragTarget::Camera {
                                    category: focused_category,
                                    initial_mouse_pos: screen_pos,
                                    initial_camera_pos: camera.center,
                                },
                            })
                    })
            }
            geng::MouseButton::Left => {
                // Drag vertex
                let focused_category = self.focused_category;
                self.world_to_category(&focused_category, world_pos).map(
                    |(category, local_pos, _)| {
                        selection::objects_under_point(&category.inner, local_pos)
                            .next()
                            .map(|(&id, _)| DragAction::Move {
                                target: DragTarget::Vertex {
                                    id,
                                    category: focused_category,
                                },
                            })
                            .or_else(|| {
                                // Drag edge
                                selection::morphisms_under_point(&category.inner, local_pos)
                                    .next()
                                    .map(|(&id, _)| DragAction::Move {
                                        target: DragTarget::Edge {
                                            id,
                                            category: focused_category,
                                        },
                                    })
                            })
                            .unwrap_or_else(|| DragAction::Selection {})
                    },
                )
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
                        &mut DragTarget::Camera {
                            category,
                            initial_camera_pos,
                            initial_mouse_pos,
                        } => self
                            .world_to_category(&category, world_pos)
                            .map(|(_, local, local_aabb)| (local, local_aabb))
                            .and_then(|(local_pos, local_aabb)| {
                                self.get_category_camera_mut(&category).map(
                                    |(camera, framebuffer_size)| {
                                        let initial = camera.screen_to_world(
                                            framebuffer_size.map(|x| x as f32),
                                            initial_mouse_pos,
                                        );
                                        let delta = initial - local_pos.clamp_aabb(local_aabb);
                                        camera.center = initial_camera_pos + delta;
                                    },
                                )
                            })
                            .is_some(),
                        &mut DragTarget::Vertex { category, id } => self
                            .world_to_category(&category, world_pos)
                            .and_then(|(category, local_pos, local_aabb)| {
                                category.inner.objects.get_mut(&id).map(|object| {
                                    object.position = local_pos.clamp_aabb(local_aabb);
                                })
                            })
                            .is_some(),
                        &mut DragTarget::Edge { category, id } => self
                            .world_to_category(&category, world_pos)
                            .and_then(|(category, local_pos, local_aabb)| {
                                category.inner.morphisms.get_mut(&id).map(|morphism| {
                                    let positions = &mut morphism.inner.positions;
                                    let center = positions.len() / 2;
                                    if let Some(pos) = positions.get_mut(center) {
                                        *pos = local_pos.clamp_aabb(local_aabb);
                                    }
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
                        if let &FocusedCategory::Rule { index } = &self.focused_category {
                            let main_selection = RuleSelection::new(
                                &self.fact_category.inner,
                                &self.fact_category.equalities,
                                index,
                                &self.rules,
                                false,
                            );
                            let goal_selection = RuleSelection::new(
                                &self.goal_category.inner,
                                &self.goal_category.equalities,
                                index,
                                &self.rules,
                                true,
                            );
                            match main_selection.current() {
                                Some(_) => {
                                    self.main_selection = Some(main_selection);
                                }
                                None => {
                                    self.apply_rule(FocusedCategory::Fact, main_selection);
                                }
                            }
                            match goal_selection.current() {
                                Some(_) => {
                                    self.goal_selection = Some(goal_selection);
                                }
                                None => {
                                    self.apply_rule(FocusedCategory::Goal, goal_selection);
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
                            &DragTarget::Vertex {
                                category: graph,
                                id,
                            } => Some((graph, CategoryThing::Object { id })),
                            &DragTarget::Edge {
                                category: graph,
                                id,
                            } => Some((graph, CategoryThing::Morphism { id })),
                            _ => None,
                        };

                        if let Some((focused_graph, selected)) = selected {
                            let selection = match focused_graph {
                                FocusedCategory::Rule { .. } => None,
                                FocusedCategory::Fact => Some((
                                    &self.fact_category.inner,
                                    &self.fact_category.equalities,
                                    &mut self.main_selection,
                                )),
                                FocusedCategory::Goal => Some((
                                    &self.goal_category.inner,
                                    &self.goal_category.equalities,
                                    &mut self.goal_selection,
                                )),
                            };

                            if let Some((graph, equalities, selection)) = selection {
                                match selection
                                    .as_ref()
                                    .and_then(|selection| selection.inferred_options().as_ref())
                                {
                                    Some(options) => {
                                        if options.contains(&selected)
                                            && selection
                                                .as_mut()
                                                .unwrap()
                                                .select(graph, equalities, selected, &self.rules)
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
