use super::*;

mod render;
mod rule;

pub use rule::*;

type RuleTexture = ugli::Texture2d<Color<f32>>;

pub struct Rules {
    geng: Geng,
    pub width: f32,
    rules: Vec<Rule>,
    cameras: Vec<Camera2d>,
    textures: Vec<RuleTexture>,
    focused_rule: Option<usize>,
}

impl Rules {
    pub fn new(geng: &Geng, rules: Vec<Rule>) -> Self {
        Self {
            geng: geng.clone(),
            width: 1.0,
            focused_rule: None,
            cameras: (0..rules.len())
                .map(|_| Camera2d {
                    center: Vec2::ZERO,
                    rotation: 0.0,
                    fov: 50.0,
                })
                .collect(),
            textures: rules
                .iter()
                .map(|_| RuleTexture::new_with(geng.ugli(), RULE_RESOLUTION, |_| Color::BLACK))
                .collect(),
            rules,
        }
    }

    pub fn get_camera(&self, index: usize) -> Option<&Camera2d> {
        self.cameras.get(index)
    }

    pub fn get_camera_mut(&mut self, index: usize) -> Option<&mut Camera2d> {
        self.cameras.get_mut(index)
    }

    pub fn get_rule(&self, index: usize) -> Option<&Rule> {
        self.rules.get(index)
    }

    pub fn get_rule_mut(&mut self, index: usize) -> Option<&mut Rule> {
        self.rules.get_mut(index)
    }

    pub fn rules_count(&self) -> usize {
        self.rules.len()
    }

    pub fn update(&mut self, delta_time: f32) {
        for rule in &mut self.rules {
            rule.update_graph(delta_time);
        }
    }

    pub fn focus(&mut self, rule_index: Option<usize>) {
        self.focused_rule = rule_index;
    }

    pub fn layout(
        &self,
        camera: &Camera2d,
        framebuffer_size: Vec2<f32>,
    ) -> impl Iterator<Item = AABB<f32>> {
        layout(self.width, self.rules_count(), camera, framebuffer_size)
    }
}

fn layout(
    width: f32,
    rules: usize,
    camera: &Camera2d,
    framebuffer_size: Vec2<f32>,
) -> impl Iterator<Item = AABB<f32>> {
    let camera_view = camera_view(camera, framebuffer_size);

    let rule_height = width / RULE_RESOLUTION.x as f32 * RULE_RESOLUTION.y as f32;
    let rule_aabb_base = AABB::point(camera_view.top_right())
        .extend_left(width)
        .extend_down(rule_height);

    (0..rules).map(move |rule_index| {
        rule_aabb_base.translate(vec2(0.0, -rule_height * rule_index as f32))
    })
}
