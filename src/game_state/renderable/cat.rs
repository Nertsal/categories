use super::*;

pub struct RenderableCategory {
    geng: Geng,
    assets: Rc<Assets>,
    pub inner: Category,
    pub camera: BoundedCamera,
    pub texture: ugli::Texture,
    pub texture_size: Vec2<usize>,
    action_history: Vec<Vec<CategoryAction>>,
    redo_history: Vec<Vec<CategoryAction>>,
}

impl RenderableCategory {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, category: Category) -> Self {
        let texture_size = vec2(1, 1);
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: BoundedCamera::new(50.0),
            texture: ugli::Texture::new_with(geng.ugli(), texture_size, |_| Color::BLACK),
            inner: category,
            action_history: vec![],
            redo_history: vec![],
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
        selection: Option<&Vec<RuleInput<Label>>>,
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
            self.camera.inner(),
            &self.inner,
            background_color,
            selection,
        );
    }

    pub fn action_do(&mut self, actions: Vec<CategoryAction>) {
        if actions.is_empty() {
            return;
        }

        self.redo_history.clear();
        self.action_history.push(actions);
    }

    pub fn action_undo(&mut self) {
        if let Some(actions) = self.action_history.pop() {
            let mut redo_actions = Vec::new();
            for action in actions {
                redo_actions.extend(self.inner.action_do(action));
            }
            self.redo_history.push(redo_actions);
        }
    }

    pub fn action_redo(&mut self) {
        if let Some(actions) = self.redo_history.pop() {
            let mut undo_actions = Vec::new();
            for action in actions {
                undo_actions.extend(self.inner.action_do(action));
            }
            self.action_history.push(undo_actions);
        }
    }
}
