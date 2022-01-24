use super::*;

pub struct State {
    pub framebuffer_size: Vec2<f32>,
    pub rule_aspect_ratio: f32,
    pub rules_width: f32,
    pub rules_scroll: f32,
    pub graphs_layout: Vec<(FocusedCategory, AABB<f32>)>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            framebuffer_size: vec2(1.0, 1.0),
            rule_aspect_ratio: 16.0 / 9.0,
            rules_width: 1.0,
            rules_scroll: 0.0,
            graphs_layout: vec![],
        }
    }
}

impl State {
    pub fn update(&mut self, framebuffer_size: Vec2<f32>, rules: usize) {
        self.framebuffer_size = framebuffer_size;
        self.rules_width = self.framebuffer_size.x * RULES_WIDTH_FRAC;
        self.graphs_layout = self.layout_graphs(rules, self.rules_scroll).collect();
    }

    pub fn scroll_rules(&mut self, delta: f32, rules: usize) {
        let rule_height = self.rules_width / self.rule_aspect_ratio;
        self.rules_scroll = (self.rules_scroll + delta).clamp(0.0, rule_height * rules as f32);
    }

    fn layout_graphs(
        &self,
        rules: usize,
        rules_scroll: f32,
    ) -> impl Iterator<Item = (FocusedCategory, AABB<f32>)> {
        let camera_view = util::ui_view(self.framebuffer_size);
        let rule_height = self.rules_width / self.rule_aspect_ratio;
        let rule_aabb_base = AABB::point(camera_view.top_left() + vec2(0.0, rules_scroll))
            .extend_right(self.rules_width)
            .extend_down(rule_height);

        (0..rules)
            .map(move |rule_index| {
                (
                    FocusedCategory::Rule { index: rule_index },
                    rule_aabb_base.translate(vec2(0.0, -rule_height * rule_index as f32)),
                )
            })
            .chain(
                vec![
                    (FocusedCategory::Fact, self.layout_main_graph()),
                    (FocusedCategory::Goal, self.layout_goal_graph()),
                ]
                .into_iter(),
            )
    }

    /// Returns tha graph layout or None if a rule's graph has not been layed out yet
    /// (in that case, ensure that you call [layout_graphs] with an appropriate **rules** parameter).
    pub fn get_graph_layout(&self, graph: &FocusedCategory) -> Option<AABB<f32>> {
        let graphs = self.graphs_layout.len();
        let index = match graph {
            FocusedCategory::Rule { index } => {
                if *index >= graphs - 2 {
                    return None;
                }
                *index
            }
            FocusedCategory::Fact => graphs - 2,
            FocusedCategory::Goal => graphs - 1,
        };
        assert_eq!(*graph, self.graphs_layout[index].0);
        Some(self.graphs_layout[index].1)
    }

    fn layout_main_graph(&self) -> AABB<f32> {
        let ruleless_aabb = util::ui_view(self.framebuffer_size).extend_left(-self.rules_width);
        ruleless_aabb.extend_right(-ruleless_aabb.width() / 2.0)
    }

    fn layout_goal_graph(&self) -> AABB<f32> {
        let ruleless_aabb = util::ui_view(self.framebuffer_size).extend_left(-self.rules_width);
        ruleless_aabb.extend_left(-ruleless_aabb.width() / 2.0)
    }
}
