use super::*;

impl GameState {
    pub fn get_renderable_graph(
        &self,
        focused_category: &FocusedCategory,
    ) -> Option<&RenderableCategory> {
        match focused_category {
            FocusedCategory::Rule { index } => self.rules.get(*index).map(|rule| &rule.category),
            FocusedCategory::Fact => Some(&self.fact_category),
            FocusedCategory::Goal => Some(&self.goal_category),
        }
    }

    pub fn get_renderable_graph_mut(
        &mut self,
        focused_category: &FocusedCategory,
    ) -> Option<&mut RenderableCategory> {
        match focused_category {
            FocusedCategory::Rule { index } => {
                self.rules.get_mut(*index).map(|rule| &mut rule.category)
            }
            FocusedCategory::Fact => Some(&mut self.fact_category),
            FocusedCategory::Goal => Some(&mut self.goal_category),
        }
    }

    /// Returns the category, a local position in it, and an aabb representing it
    pub fn world_to_category(
        &self,
        category: &FocusedCategory,
        world_pos: Vec2<f32>,
    ) -> Option<(&Category, Vec2<f32>, AABB<f32>)> {
        self.world_to_category_pos(category, world_pos)
            .and_then(|(_, graph_pos, aabb)| {
                self.get_category(category)
                    .map(|graph| (graph, graph_pos, aabb))
            })
    }

    /// Returns the category, a local position in it, and an aabb representing it
    pub fn world_to_category_mut(
        &mut self,
        category: &FocusedCategory,
        world_pos: Vec2<f32>,
    ) -> Option<(&mut Category, Vec2<f32>, AABB<f32>)> {
        self.world_to_category_pos(category, world_pos)
            .and_then(|(_, graph_pos, aabb)| {
                self.get_category_mut(category)
                    .map(|graph| (graph, graph_pos, aabb))
            })
    }

    /// Returns a local screen position, a local world position inside the category, and its aabb;
    /// or returns None if there is no such graph.
    pub fn world_to_category_pos(
        &self,
        category: &FocusedCategory,
        world_pos: Vec2<f32>,
    ) -> Option<(Vec2<f32>, Vec2<f32>, AABB<f32>)> {
        self.state.get_graph_layout(category).map(|aabb| {
            let (framebuffer_size, camera) = match category {
                FocusedCategory::Rule { index } => {
                    let category = &self.rules[*index].category; // The rule is guaranteed to exist, for there exists a layout
                    (category.texture_size.map(|x| x as f32), &category.camera)
                }
                FocusedCategory::Fact => (
                    self.goal_category.texture_size.map(|x| x as f32),
                    &self.fact_category.camera,
                ),
                FocusedCategory::Goal => (
                    self.goal_category.texture_size.map(|x| x as f32),
                    &self.goal_category.camera,
                ),
            };
            let screen_pos = (world_pos - aabb.bottom_left()) / vec2(aabb.width(), aabb.height())
                * framebuffer_size;
            (
                screen_pos,
                camera.screen_to_world(framebuffer_size, screen_pos),
                util::camera_view(camera, framebuffer_size),
            )
        })
    }

    pub fn get_category(&self, category: &FocusedCategory) -> Option<&Category> {
        match category {
            FocusedCategory::Rule { index } => {
                self.rules.get(*index).map(|rule| &rule.category.inner)
            }
            FocusedCategory::Fact => Some(&self.fact_category.inner),
            FocusedCategory::Goal => Some(&self.goal_category.inner),
        }
    }

    pub fn get_category_mut(&mut self, category: &FocusedCategory) -> Option<&mut Category> {
        match category {
            FocusedCategory::Rule { index } => self
                .rules
                .get_mut(*index)
                .map(|rule| &mut rule.category.inner),
            FocusedCategory::Fact => Some(&mut self.fact_category.inner),
            FocusedCategory::Goal => Some(&mut self.goal_category.inner),
        }
    }

    /// Returns the category's camera and framebuffer size
    pub fn get_category_camera(
        &self,
        category: &FocusedCategory,
    ) -> Option<(&Camera2d, Vec2<usize>)> {
        match category {
            FocusedCategory::Rule { index } => self.rules.get(*index).map(|rule| {
                let category = &rule.category;
                (&category.camera, category.texture_size)
            }),
            FocusedCategory::Fact => {
                Some((&self.fact_category.camera, self.fact_category.texture_size))
            }
            FocusedCategory::Goal => {
                Some((&self.goal_category.camera, self.goal_category.texture_size))
            }
        }
    }

    /// Returns the category's camera and framebuffer size
    pub fn get_category_camera_mut(
        &mut self,
        category: &FocusedCategory,
    ) -> Option<(&mut Camera2d, Vec2<usize>)> {
        match category {
            FocusedCategory::Rule { index } => self.rules.get_mut(*index).map(|rule| {
                let category = &mut rule.category;
                (&mut category.camera, category.texture_size)
            }),
            FocusedCategory::Fact => Some((
                &mut self.fact_category.camera,
                self.fact_category.texture_size,
            )),
            FocusedCategory::Goal => Some((
                &mut self.goal_category.camera,
                self.goal_category.texture_size,
            )),
        }
    }
}
