use super::*;

pub fn random_shift() -> Vec2<f32> {
    let mut rng = global_rng();
    vec2(rng.gen(), rng.gen())
}

pub fn camera_view(camera: &Camera2d, framebuffer_size: Vec2<f32>) -> AABB<f32> {
    AABB::point(camera.center).extend_symmetric(
        vec2(
            camera.fov / framebuffer_size.y * framebuffer_size.x,
            camera.fov,
        ) / 2.0,
    )
}
