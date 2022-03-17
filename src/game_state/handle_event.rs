use super::*;

impl GameState {
    pub fn handle_event_impl(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key } => self.handle_key_down(key),
            geng::Event::MouseDown { position, button } => {
                self.handle_mouse_down(position, button);
            }
            geng::Event::MouseMove { position, .. } => {
                self.handle_mouse_move(position);
            }
            geng::Event::MouseUp { .. } => {
                self.handle_mouse_up();
            }
            geng::Event::Wheel { delta } => {
                self.handle_wheel(delta as f32);
            }
            geng::Event::TouchStart { touches } => self.handle_touch_start(touches),
            geng::Event::TouchMove { touches } => self.handle_touch_move(touches),
            geng::Event::TouchEnd => self.handle_touch_end(),
            _ => (),
        }
    }

    fn handle_key_down(&mut self, key: geng::Key) {
        match key {
            geng::Key::Space => {
                // Anchor object
                if let Some(dragging) = &self.dragging {
                    if let DragAction::Move {
                        target: DragTarget::Object { category, id },
                    } = &dragging.action
                    {
                        let category = *category;
                        let id = *id;
                        let object = self
                            .get_category_mut(&category)
                            .unwrap()
                            .objects
                            .get_mut(&id)
                            .unwrap();
                        object.inner.is_anchor = !object.inner.is_anchor;
                    }
                }
            }
            geng::Key::Escape => {
                // Clear selection
                self.fact_selection = None;
                self.goal_selection = None;
            }
            geng::Key::Z if self.geng.window().is_key_pressed(geng::Key::LCtrl) => {
                let active_category = match self.focused_category {
                    FocusedCategory::Rule { .. } => return,
                    FocusedCategory::Fact => &mut self.fact_category,
                    FocusedCategory::Goal => &mut self.goal_category,
                };

                if self.geng.window().is_key_pressed(geng::Key::LShift) {
                    active_category.action_redo();
                } else {
                    active_category.action_undo();
                }
            }
            _ => (),
        }
    }

    fn handle_wheel(&mut self, wheel_delta: f32) {
        if self.geng.window().is_key_pressed(geng::Key::LCtrl) {
            // Zoom
            let delta = -wheel_delta * ZOOM_SPEED;
            let camera = self.focused_camera_mut();
            camera.fov = (camera.fov + delta).clamp(CAMERA_FOV_MIN, CAMERA_FOV_MAX);
        } else {
            // Scroll
            let delta = -wheel_delta * SCROLL_SPEED;
            self.state.scroll_rules(delta);
        }
    }

    fn handle_touch_start(&mut self, touches: Vec<geng::TouchPoint>) {
        match &touches[..] {
            [touch] => {
                let mouse_position = touch.position;
                self.focus(mouse_position);

                let world_pos = self.screen_to_ui(mouse_position);

                // Drag target or camera
                let action =
                    self.drag_target(self.focused_category, world_pos)
                        .and_then(|target| {
                            target
                                .map(|target| DragAction::Move { target })
                                .or_else(|| match self.focused_category {
                                    FocusedCategory::Rule { .. } => Some(DragAction::RuleScroll {
                                        initial_scroll: self.state.rules_scroll,
                                        initial_ui_pos: world_pos,
                                    }),
                                    _ => self.drag_camera(self.focused_category, world_pos),
                                })
                        });

                self.dragging = action.map(|action| Dragging {
                    mouse_start_position: mouse_position,
                    world_start_position: world_pos,
                    started_drag: false,
                    current_mouse_position: mouse_position,
                    action,
                });
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
                    started_drag: false,
                    world_start_position: world_pos,
                    action: DragAction::TwoTouchMove {
                        initial_camera_pos: camera.center,
                        initial_camera_fov: camera.fov,
                        // initial_camera_rotation: camera.rotation,
                        initial_touch: touch0.position,
                        initial_touch_other: touch1.position,
                    },
                })
            }
            _ => (),
        }
    }

    fn handle_touch_move(&mut self, touches: Vec<geng::TouchPoint>) {
        match &touches[..] {
            [touch] => {
                self.handle_mouse_move(touch.position);
            }
            [touch0, touch1] => {
                // self.focus(touch0.position);
                if let Some(Dragging {
                    action:
                        DragAction::TwoTouchMove {
                            // initial_camera_rotation,
                            initial_camera_pos,
                            initial_camera_fov,
                            initial_touch,
                            initial_touch_other,
                        },
                    ref mut current_mouse_position,
                    ..
                }) = self.dragging
                {
                    *current_mouse_position = touch0.position;

                    // Scale camera
                    let initial_delta = initial_touch - initial_touch_other;
                    let initial_distance = initial_delta.len() as f32;
                    // let initial_angle = initial_delta.arg();

                    let delta = touch0.position - touch1.position;
                    let distance = delta.len() as f32;
                    // let angle = delta.arg();

                    // Shift towards touch
                    let to_world = |screen_pos: Vec2<f64>| {
                        let world_pos = self.screen_to_ui(screen_pos);
                        self.world_to_category_pos(&self.focused_category, world_pos)
                            .expect("Failed to find focused category")
                            .1
                    };
                    let initial_center =
                        (to_world(initial_touch) + to_world(initial_touch_other)) / 2.0;
                    let center = (to_world(touch0.position) + to_world(touch1.position)) / 2.0;
                    let shift = initial_center - center;

                    // Apply transformations
                    let camera = self.focused_camera_mut();
                    camera.fov = (initial_camera_fov / distance * initial_distance)
                        .clamp(CAMERA_FOV_MIN, CAMERA_FOV_MAX);
                    camera.center = initial_camera_pos + shift;
                    // camera.rotation = initial_camera_rotation + angle - initial_angle;
                }
            }
            _ => (),
        }
    }

    fn handle_touch_end(&mut self) {
        // TODO: Detect short and long taps
        self.handle_mouse_up();
        self.focused_category = FocusedCategory::Fact;
    }

    fn handle_mouse_down(&mut self, mouse_position: Vec2<f64>, mouse_button: geng::MouseButton) {
        self.focus(mouse_position);

        let world_pos = self.screen_to_ui(mouse_position);

        let drag_camera = mouse_button == geng::MouseButton::Left
            && self.geng.window().is_key_pressed(geng::Key::LCtrl)
            || mouse_button == geng::MouseButton::Right;
        let action = match mouse_button {
            _ if drag_camera => self.drag_camera(self.focused_category, world_pos),
            geng::MouseButton::Left => {
                // Drag target or select
                self.drag_target(self.focused_category, world_pos)
                    .map(|target| {
                        target
                            .map(|target| DragAction::Move { target })
                            .unwrap_or_else(|| match self.focused_category {
                                FocusedCategory::Rule { .. } => DragAction::RuleScroll {
                                    initial_scroll: self.state.rules_scroll,
                                    initial_ui_pos: world_pos,
                                },
                                _ => DragAction::Selection {},
                            })
                    })
            }
            _ => None,
        };

        self.dragging = action.map(|action| Dragging {
            mouse_start_position: mouse_position,
            world_start_position: world_pos,
            started_drag: false,
            current_mouse_position: mouse_position,
            action,
        });
    }

    fn handle_mouse_move(&mut self, mouse_position: Vec2<f64>) {
        self.focus(mouse_position);
        if let Some(dragging) = &mut self.dragging {
            dragging.started_drag = true;
            dragging.current_mouse_position = mouse_position;
        }
    }

    pub fn drag_update(&mut self) {
        if let Some(mut dragging) = self.dragging.take() {
            let world_pos = self.screen_to_ui(dragging.current_mouse_position);
            match &mut dragging.action {
                DragAction::Move { target } if dragging.started_drag => {
                    let updated = match target {
                        &mut DragTarget::Camera {
                            category,
                            initial_camera_pos,
                            initial_mouse_pos,
                        } => self
                            .world_to_category_mut(&category, world_pos)
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
                        &mut DragTarget::Object { category, id } => self
                            .world_to_category_mut(&category, world_pos)
                            .and_then(|(category, local_pos, local_aabb)| {
                                category.objects.get_mut(&id).map(|object| {
                                    object.inner.position = local_pos.clamp_aabb(local_aabb);
                                })
                            })
                            .is_some(),
                        &mut DragTarget::Morphism { category, id } => self
                            .world_to_category_mut(&category, world_pos)
                            .and_then(|(category, local_pos, local_aabb)| {
                                category.morphisms.get_mut(&id).map(|morphism| {
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
                        // Ensure self.dragging is None
                        return;
                    }
                }
                &mut DragAction::RuleScroll {
                    initial_scroll: initial_shift,
                    initial_ui_pos,
                } => {
                    let delta = world_pos.y - initial_ui_pos.y;
                    let delta = initial_shift + delta - self.state.rules_scroll;
                    self.state.scroll_rules(delta);
                }
                _ => (),
            }
            self.dragging = Some(dragging);
        }
    }

    fn handle_mouse_up(&mut self) {
        let dragging = match self.dragging.take() {
            Some(x) => x,
            None => return,
        };

        let mouse_position = dragging.current_mouse_position;
        let mouse_world_pos = self.ui_camera.screen_to_world(
            self.state.framebuffer_size,
            mouse_position.map(|x| x as f32),
        );

        match &dragging.action {
            DragAction::Selection { .. } => {
                self.drag_selection_stop(mouse_position, dragging.mouse_start_position);
            }
            DragAction::Move { target } => {
                self.drag_move_stop(mouse_world_pos, dragging.world_start_position, target);
            }
            &DragAction::RuleScroll { initial_ui_pos, .. } => {
                self.drag_scroll_stop(mouse_world_pos, initial_ui_pos);
            }
            _ => (),
        }
    }

    fn drag_selection_stop(&mut self, mouse_position: Vec2<f64>, mouse_start_position: Vec2<f64>) {
        let dragged_delta = mouse_position - mouse_start_position;
        if !dragged_delta.len().approx_eq(&0.0) {
            return;
        }

        if let &FocusedCategory::Rule { index } = &self.focused_category {
            self.select_rule(index);
        }
    }

    fn drag_scroll_stop(&mut self, world_pos: Vec2<f32>, world_start_pos: Vec2<f32>) {
        let delta = world_pos - world_start_pos;
        if !delta.len().approx_eq(&0.0) {
            return;
        }

        if let FocusedCategory::Rule { index } = self.focused_category {
            self.select_rule(index);
        }
    }

    fn drag_move_stop(
        &mut self,
        world_pos: Vec2<f32>,
        world_start_pos: Vec2<f32>,
        target: &DragTarget,
    ) {
        let delta = world_pos - world_start_pos;
        if !delta.len().approx_eq(&0.0) {
            return;
        }

        // Select object or morphism
        let selected = match target {
            &DragTarget::Object {
                category: graph,
                id,
            } => Some((graph, RuleInput::Object { label: (), id })),
            &DragTarget::Morphism {
                category: graph,
                id,
            } => Some((graph, RuleInput::Morphism { label: (), id })),
            _ => None,
        };

        let (focused_category, selected) = match selected {
            Some(x) => x,
            None => return,
        };

        if let FocusedCategory::Rule { index } = focused_category {
            self.select_rule(index);
            return;
        }

        let selection = match focused_category {
            FocusedCategory::Rule { .. } => None,
            FocusedCategory::Fact => {
                Some((&mut self.fact_category.inner, &mut self.fact_selection))
            }
            FocusedCategory::Goal => {
                Some((&mut self.goal_category.inner, &mut self.goal_selection))
            }
        };

        let (category, selection) = match selection {
            Some(x) => x,
            None => return,
        };

        let selected = selection
            .as_ref()
            .and_then(|selection| {
                selection.current().and_then(|current| {
                    selection
                        .inferred_options()
                        .as_ref()
                        .map(|options| (current.clone(), options))
                })
            })
            .and_then(|(current, options)| {
                let selected = match (current, selected) {
                    (RuleInput::Object { label, .. }, RuleInput::Object { id, .. }) => {
                        Some(RuleInput::Object { label, id })
                    }
                    (RuleInput::Object { .. }, _) => None,
                    (RuleInput::Morphism { label, .. }, RuleInput::Morphism { id, .. }) => {
                        Some(RuleInput::Morphism { label, id })
                    }
                    (RuleInput::Morphism { .. }, _) => None,
                    (
                        RuleInput::Equality { left, right },
                        RuleInput::Equality {
                            left: id_left,
                            right: id_right,
                        },
                    ) => Some(RuleInput::Equality {
                        left: left
                            .into_iter()
                            .map(|(label, _)| label)
                            .zip(id_left.into_iter().map(|(_, id)| id))
                            .collect(),
                        right: right
                            .into_iter()
                            .map(|(label, _)| label)
                            .zip(id_right.into_iter().map(|(_, id)| id))
                            .collect(),
                    }),
                    (RuleInput::Equality { .. }, _) => None,
                };
                selected.filter(|selected| options.contains(selected))
            });

        // Add to selection
        match selected {
            Some(selected) => {
                let next = selection
                    .as_mut()
                    .unwrap()
                    .select(category, selected, &self.rules);
                if next.is_none() {
                    let selection = selection.take().unwrap();
                    self.apply_rule(focused_category, selection);
                }
            }
            None => {
                *selection = None;
            }
        }
    }

    /// Returns `None` if the category does not exist,
    /// `Some(None)` if there is no target at the position,
    /// and the target otherwise
    fn drag_target(
        &self,
        focused_category: FocusedCategory,
        world_pos: Vec2<f32>,
    ) -> Option<Option<DragTarget>> {
        self.world_to_category(&focused_category, world_pos)
            .map(|(category, local_pos, _)| {
                selection::targets_under_point(category, focused_category, local_pos)
            })
    }

    /// Returns `None` if the category does not exist,
    /// and camera drag action otherwise
    fn drag_camera(
        &self,
        focused_category: FocusedCategory,
        world_pos: Vec2<f32>,
    ) -> Option<DragAction> {
        self.world_to_category_pos(&focused_category, world_pos)
            .and_then(|(screen_pos, _, _)| {
                self.get_category_camera(&focused_category)
                    .map(|(camera, _)| DragAction::Move {
                        target: DragTarget::Camera {
                            category: focused_category,
                            initial_mouse_pos: screen_pos,
                            initial_camera_pos: camera.center,
                        },
                    })
            })
    }

    /// Transform a position in screen coordinates to ui coordinates
    fn screen_to_ui(&self, mouse_position: Vec2<f64>) -> Vec2<f32> {
        self.ui_camera.screen_to_world(
            self.state.framebuffer_size,
            mouse_position.map(|x| x as f32),
        )
    }

    fn select_rule(&mut self, index: usize) {
        let main_selection =
            RuleSelection::new(&self.fact_category.inner, index, &self.rules, None);
        let goal_selection =
            RuleSelection::new(&self.goal_category.inner, index, &self.rules, Some(0));
        match main_selection.current() {
            Some(_) => {
                self.fact_selection = Some(main_selection);
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
