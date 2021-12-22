use super::*;

impl GameState {
    pub fn get_renderable_graph(&self, focused_graph: &FocusedGraph) -> Option<&RenderableGraph> {
        match focused_graph {
            FocusedGraph::Rule { index } => self.rules.get(*index).map(|rule| rule.graph()),
            FocusedGraph::Main => Some(&self.main_graph),
            FocusedGraph::Goal => Some(&self.goal_graph),
        }
    }

    pub fn get_renderable_graph_mut(
        &mut self,
        focused_graph: &FocusedGraph,
    ) -> Option<&mut RenderableGraph> {
        match focused_graph {
            FocusedGraph::Rule { index } => self.rules.get_mut(*index).map(|rule| rule.graph_mut()),
            FocusedGraph::Main => Some(&mut self.main_graph),
            FocusedGraph::Goal => Some(&mut self.goal_graph),
        }
    }

    /// Returns the graph, a local position in it, and an aabb representing it
    pub fn world_to_graph(
        &mut self,
        graph: &FocusedGraph,
        world_pos: Vec2<f32>,
    ) -> Option<(&mut Graph, Vec2<f32>, AABB<f32>)> {
        self.world_to_graph_pos(graph, world_pos)
            .and_then(|(_, graph_pos, aabb)| {
                self.get_graph_mut(graph)
                    .map(|graph| (graph, graph_pos, aabb))
            })
    }

    /// Returns a local screen position, a local world position inside the graph, and its aabb;
    /// or returns None if there is no such graph.
    pub fn world_to_graph_pos(
        &self,
        graph: &FocusedGraph,
        world_pos: Vec2<f32>,
    ) -> Option<(Vec2<f32>, Vec2<f32>, AABB<f32>)> {
        self.state.get_graph_layout(graph).map(|aabb| {
            let (framebuffer_size, camera) = match graph {
                FocusedGraph::Rule { index } => {
                    let graph = self.rules[*index].graph(); // The rule is guaranteed to exist, for there exists a layout
                    (graph.texture_size.map(|x| x as f32), &graph.camera)
                }
                FocusedGraph::Main => (
                    self.goal_graph.texture_size.map(|x| x as f32),
                    &self.main_graph.camera,
                ),
                FocusedGraph::Goal => (
                    self.goal_graph.texture_size.map(|x| x as f32),
                    &self.goal_graph.camera,
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

    pub fn get_graph_mut(&mut self, graph: &FocusedGraph) -> Option<&mut Graph> {
        match graph {
            FocusedGraph::Rule { index } => self
                .rules
                .get_mut(*index)
                .map(|rule| &mut rule.graph_mut().graph),
            FocusedGraph::Main => Some(&mut self.main_graph.graph),
            FocusedGraph::Goal => Some(&mut self.goal_graph.graph),
        }
    }

    /// Returns the graph's camera and framebuffer size
    pub fn get_graph_camera_mut(
        &mut self,
        graph: &FocusedGraph,
    ) -> Option<(&mut Camera2d, Vec2<usize>)> {
        match graph {
            FocusedGraph::Rule { index } => self.rules.get_mut(*index).map(|rule| {
                let graph = rule.graph_mut();
                (&mut graph.camera, graph.texture_size)
            }),
            FocusedGraph::Main => Some((&mut self.main_graph.camera, self.main_graph.texture_size)),
            FocusedGraph::Goal => Some((&mut self.goal_graph.camera, self.goal_graph.texture_size)),
        }
    }
}
