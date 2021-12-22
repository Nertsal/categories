use super::*;

mod dashed;
pub mod graph;

use dashed::*;
use geng::Draw2d;

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        let old_framebuffer_size = self.state.framebuffer_size;
        self.state.update(framebuffer_size, self.rules.len());
        if old_framebuffer_size != framebuffer_size {
            self.resize_textures();
        }

        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Render graphs
        for (focused_graph, graph_aabb) in
            self.state.graphs_layout.iter().copied().collect::<Vec<_>>()
        {
            let focused_rule = self.focused_rule;
            let is_selected_rule = match (&focused_rule, focused_graph) {
                (Some(focused_rule), FocusedGraph::Rule { index }) if index == *focused_rule => {
                    true
                }
                _ => false,
            };

            // Choose background color
            let background_color = if is_selected_rule {
                SELECTED_COLOR
            } else {
                Color::BLACK
            };

            // Choose selected objects
            let selection = if is_selected_rule {
                self.main_selection
                    .as_mut()
                    .or(self.goal_selection.as_mut())
                    .and_then(|selection| selection.current().map(|&current| vec![current]))
            } else {
                match focused_graph {
                    FocusedGraph::Main => self.main_selection.as_ref(),
                    FocusedGraph::Goal => self.goal_selection.as_ref(),
                    _ => None,
                }
                .and_then(|selection| selection.inferred_options().clone())
            };

            // Render graph to a texture
            let graph = self.get_renderable_graph_mut(&focused_graph).unwrap();
            graph.update_texture(background_color, selection.as_ref());
            let graph = self.get_renderable_graph(&focused_graph).unwrap();

            // Render texture to the dedicated part on the screen
            self.geng.draw_2d(
                framebuffer,
                &self.ui_camera,
                &draw_2d::TexturedQuad::new(graph_aabb, &graph.texture),
            );

            // Draw graph outline
            draw_2d::Chain::new(
                Chain::new(vec![
                    graph_aabb.bottom_left(),
                    graph_aabb.top_left(),
                    graph_aabb.top_right(),
                    graph_aabb.bottom_right(),
                ]),
                GRAPH_OUTLINE_WIDTH,
                GRAPH_OUTLINE_COLOR,
                0,
            )
            .draw_2d(&self.geng, framebuffer, &self.ui_camera);
        }
    }

    fn resize_textures(&mut self) {
        for (graph, graph_aabb) in self.state.graphs_layout.clone() {
            let graph = self.get_renderable_graph_mut(&graph).unwrap();
            let texture_width = graph_aabb.width() * GRAPH_TEXTURE_SCALE;
            let texture_height = texture_width / graph_aabb.width() * graph_aabb.height();
            graph.resize_texture(vec2(texture_width, texture_height).map(|x| x.ceil() as usize));
        }
    }
}
