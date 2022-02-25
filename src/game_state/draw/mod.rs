use super::*;

pub mod category;
mod dashed;

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

        let mut selected_rule = match self.focused_category {
            FocusedCategory::Rule { .. } | FocusedCategory::Fact => self.fact_selection.as_ref(),
            FocusedCategory::Goal => self.goal_selection.as_ref(),
        }
        .and_then(|selection| {
            selection
                .current()
                .map(|current| (selection.rule(), current.clone()))
        });

        // Render graphs
        for (current_category, graph_aabb) in
            self.state.graphs_layout.iter().copied().collect::<Vec<_>>()
        {
            // Choose selected objects
            let selection = match (&selected_rule, current_category) {
                (Some((rule_index, _)), FocusedCategory::Rule { index })
                    if index == *rule_index =>
                {
                    let (_, input) = selected_rule.take().unwrap();
                    Some(vec![input])
                }
                (_, category) => match category {
                    FocusedCategory::Fact => self.fact_selection.as_ref(),
                    FocusedCategory::Goal => self.goal_selection.as_ref(),
                    _ => None,
                }
                .and_then(|selection| selection.inferred_options().clone()),
            };

            // Render graph to a texture
            let graph = self.get_renderable_graph_mut(&current_category).unwrap();
            graph.update_texture(Color::BLACK, selection.as_ref());
            let graph = self.get_renderable_graph(&current_category).unwrap();

            // Render texture to the dedicated part on the screen
            self.geng.draw_2d(
                framebuffer,
                &self.ui_camera,
                &draw_2d::TexturedQuad::new(graph_aabb, &graph.texture),
            );

            // Draw graph outline
            let outline_color = if current_category == self.focused_category {
                GRAPH_FOCUSED_OUTLINE_COLOR
            } else {
                GRAPH_OUTLINE_COLOR
            };

            let outline = graph_aabb.extend_uniform(-GRAPH_OUTLINE_WIDTH / 2.0);
            draw_2d::Chain::new(
                Chain::new(vec![
                    outline.bottom_left(),
                    outline.top_left(),
                    outline.top_right(),
                    outline.bottom_right(),
                    outline.bottom_left(),
                ]),
                GRAPH_OUTLINE_WIDTH,
                outline_color,
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
