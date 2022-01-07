use super::*;

pub struct RenderableGraph {
    geng: Geng,
    assets: Rc<Assets>,
    pub graph: Graph,
    pub equalities: GraphEqualities,
    pub camera: Camera2d,
    pub texture: ugli::Texture,
    pub texture_size: Vec2<usize>,
}

impl RenderableGraph {
    pub fn new(
        geng: &Geng,
        assets: &Rc<Assets>,
        graph: Graph,
        equalities: GraphEqualities,
        texture_size: Vec2<usize>,
    ) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            graph,
            equalities,
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 50.0,
            },
            texture: ugli::Texture::new_with(geng.ugli(), texture_size, |_| Color::BLACK),
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
        selection: Option<&Vec<GraphObject>>,
    ) {
        let mut temp_framebuffer = ugli::Framebuffer::new_color(
            self.geng.ugli(),
            ugli::ColorAttachment::Texture(&mut self.texture),
        );
        ugli::clear(&mut temp_framebuffer, Some(background_color), None);

        draw::graph::draw_graph(
            &self.geng,
            &self.assets,
            self.geng.default_font(),
            &mut temp_framebuffer,
            &self.camera,
            &self.graph,
            &self.equalities,
            background_color,
            selection,
        );
    }
}
