use super::*;

pub struct BoundedCamera {
    camera: Camera2d,
    bounds: AABB<f32>,
    target_bounds: AABB<f32>,
}

impl BoundedCamera {
    pub fn new(fov: f32) -> Self {
        Self {
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov,
            },
            bounds: AABB::ZERO,
            target_bounds: AABB::ZERO,
        }
    }

    pub fn inner(&self) -> &Camera2d {
        &self.camera
    }

    pub fn set_zoom(&mut self, fov: f32) {
        self.camera.fov = fov.clamp(CAMERA_FOV_MIN, CAMERA_FOV_MAX);
    }

    pub fn zoom_out(&mut self, delta: f32) {
        self.camera.fov = (self.camera.fov + delta).clamp(CAMERA_FOV_MIN, CAMERA_FOV_MAX);
    }

    pub fn set_center(&mut self, center: Vec2<f32>) {
        self.camera.center = center.clamp_aabb(self.bounds);
        self.tighten_bounds();
    }

    pub fn update_bounds(&mut self, view_bounds: AABB<f32>, framebuffer_size: Vec2<f32>) {
        let camera_view = util::camera_view(&self.camera, framebuffer_size);
        let view_size = camera_view.size() / 2.0;
        let view_bounds = view_bounds.extend_uniform(constants::CAMERA_EXTRA_SPACE);
        self.target_bounds = AABB::from_corners(
            view_bounds.bottom_left() + view_size,
            view_bounds.top_right() - view_size,
        );
        self.tighten_bounds();
    }

    fn tighten_bounds(&mut self) {
        let pos = self.camera.center;
        let bounds = &self.target_bounds;
        self.bounds = AABB {
            x_min: pos.x.min(bounds.x_min),
            x_max: pos.x.max(bounds.x_max),
            y_min: pos.y.min(bounds.y_min),
            y_max: pos.y.max(bounds.y_max),
        };
    }

    // let right_delta = boundary.x_max - camera_view.x_max;
    // let left_delta = boundary.x_min - camera_view.x_min;
    // let shift = if boundary.width() > camera_view.width() {
    //     if right_delta <= 0.0 {
    //         right_delta
    //     } else {
    //         left_delta.max(0.0)
    //     }
    // } else {
    //     if right_delta >= 0.0 {
    //         right_delta
    //     } else {
    //         left_delta.min(0.0)
    //     }
    // };
    // camera.center.x += shift;
}
