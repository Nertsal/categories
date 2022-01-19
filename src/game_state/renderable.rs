use super::*;

pub struct RenderableCategory {
    geng: Geng,
    assets: Rc<Assets>,
    pub inner: Category,
    pub equalities: Equalities,
    pub camera: Camera2d,
    pub texture: ugli::Texture,
    pub texture_size: Vec2<usize>,
}

impl RenderableCategory {
    pub fn new(
        geng: &Geng,
        assets: &Rc<Assets>,
        category: Category,
        equalities: Equalities,
        texture_size: Vec2<usize>,
    ) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 50.0,
            },
            texture: ugli::Texture::new_with(geng.ugli(), texture_size, |_| Color::BLACK),
            texture_size,
            inner: category,
            equalities,
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
            &self.equalities,
            background_color,
            selection,
        );
    }
}
