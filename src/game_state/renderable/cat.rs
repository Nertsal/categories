use geng::Draw2d;

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
    pub undo_button: Option<AABB<f32>>,
    pub redo_button: Option<AABB<f32>>,
    hide_morphisms: bool,
}

impl RenderableCategory {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, category: Category, buttons: bool) -> Self {
        let texture_size = vec2(1, 1);
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: BoundedCamera::new(50.0),
            texture: ugli::Texture::new_with(geng.ugli(), texture_size, |_| Color::BLACK),
            action_history: vec![],
            redo_history: vec![],
            undo_button: if buttons { Some(AABB::ZERO) } else { None },
            redo_button: if buttons { Some(AABB::ZERO) } else { None },
            inner: category,
            texture_size,
            hide_morphisms: buttons,
        }
    }

    pub fn resize_texture(&mut self, new_size: Vec2<usize>) {
        self.texture_size = new_size;
        self.texture = ugli::Texture::new_with(self.geng.ugli(), new_size, |_| Color::BLACK);

        if let Some(button) = &mut self.undo_button {
            *button = AABB::ZERO
                .extend_symmetric(constants::BUTTON_SIZE / 2.0)
                .translate(vec2(
                    self.texture_size.x as f32 / 2.0
                        - constants::BUTTON_SIZE.x / 2.0
                        - constants::BUTTON_EXTRA_SPACE,
                    constants::BUTTON_SIZE.y / 2.0 + constants::BUTTON_EXTRA_SPACE,
                ))
        }
        if let Some(button) = &mut self.redo_button {
            *button = AABB::ZERO
                .extend_symmetric(constants::BUTTON_SIZE / 2.0)
                .translate(vec2(
                    self.texture_size.x as f32 / 2.0
                        + constants::BUTTON_SIZE.x / 2.0
                        + constants::BUTTON_EXTRA_SPACE,
                    constants::BUTTON_SIZE.y / 2.0 + constants::BUTTON_EXTRA_SPACE,
                ))
        }
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
            self.hide_morphisms,
        );

        // Undo/redo buttons
        if let Some(button) = self.undo_button {
            draw_2d::TexturedQuad::colored(button, &self.assets.undo, constants::BUTTON_COLOR)
                .draw_2d(&self.geng, &mut temp_framebuffer, &geng::PixelPerfectCamera);
        }
        if let Some(button) = self.redo_button {
            draw_2d::TexturedQuad::colored(button, &self.assets.redo, constants::BUTTON_COLOR)
                .draw_2d(&self.geng, &mut temp_framebuffer, &geng::PixelPerfectCamera);
        }
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
