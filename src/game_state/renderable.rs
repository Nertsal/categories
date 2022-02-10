use super::*;

pub struct RenderableRule {
    pub inner: Rule,
    pub inverse: Vec<Rule>,
    pub category: RenderableCategory,
    pub input: Vec<(Label, CategoryThing)>,
}

pub struct RenderableCategory {
    geng: Geng,
    assets: Rc<Assets>,
    pub inner: Category,
    pub camera: Camera2d,
    pub texture: ugli::Texture,
    pub texture_size: Vec2<usize>,
    pub action_history: Vec<CategoryAction>,
}

impl RenderableRule {
    pub fn from_rule(geng: &Geng, assets: &Rc<Assets>, rule: Rule) -> Self {
        fn part_color(part: category::RulePart) -> Color<f32> {
            match part {
                category::RulePart::Input => RULE_INPUT_COLOR,
                category::RulePart::Forall => RULE_FORALL_COLOR,
                category::RulePart::Exists => RULE_EXISTS_COLOR,
                category::RulePart::Output => RULE_OUTPUT_COLOR,
            }
        }
        let (category, input) = Category::from_rule(
            &rule,
            |part, label, _| Point::new(label, part_color(part)),
            |part, label, _| {
                Arrow::new(
                    label,
                    part_color(part),
                    util::random_shift(),
                    util::random_shift(),
                )
            },
        );
        Self {
            inverse: rule.invert(),
            inner: rule,
            category: RenderableCategory::new(geng, assets, category),
            input,
        }
    }
}

impl RenderableCategory {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, category: Category) -> Self {
        let texture_size = vec2(1, 1);
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 50.0,
            },
            texture: ugli::Texture::new_with(geng.ugli(), texture_size, |_| Color::BLACK),
            inner: category,
            action_history: vec![],
            texture_size,
        }
    }

    pub fn resize_texture(&mut self, new_size: Vec2<usize>) {
        self.texture_size = new_size;
        self.texture = ugli::Texture::new_with(self.geng.ugli(), new_size, |_| Color::BLACK);
    }

    pub fn update_texture(
        &mut self,
        background_color: Color<f32>,
        selection: Option<&Vec<CategoryThing>>,
    ) {
        let mut temp_framebuffer = ugli::Framebuffer::new_color(
            self.geng.ugli(),
            ugli::ColorAttachment::Texture(&mut self.texture),
        );
        ugli::clear(&mut temp_framebuffer, Some(background_color), None);

        draw::category::draw_category(
            &self.geng,
            &self.assets,
            self.geng.default_font(),
            &mut temp_framebuffer,
            &self.camera,
            &self.inner,
            background_color,
            selection,
        );
    }

    pub fn action_undo(&mut self) {
        if let Some(action) = self.action_history.pop() {
            let _actions = self.inner.action_do(action);
            // TODO: redo actions
        }
    }
}
