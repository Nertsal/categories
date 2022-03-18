use super::*;

#[derive(Debug, Clone)]
pub struct Dragging {
    pub mouse_start_position: Vec2<f64>,
    pub world_start_position: Vec2<f32>,
    pub started_drag: bool,
    pub action: DragAction,
    pub current_mouse_position: Vec2<f64>,
}

#[derive(Debug, Clone)]
pub enum DragAction {
    Move {
        target: DragTarget,
    },
    Selection {},
    TwoTouchMove {
        initial_camera_pos: Vec2<f32>,
        initial_camera_fov: f32,
        initial_touch: Vec2<f64>,
        initial_touch_other: Vec2<f64>,
    },
    RuleScroll {
        initial_scroll: f32,
        initial_ui_pos: Vec2<f32>,
    },
}

#[derive(Debug, Clone)]
pub enum DragTarget {
    Camera {
        category: FocusedCategory,
        initial_world_pos: Vec2<f32>,
        initial_camera_pos: Vec2<f32>,
    },
    Object {
        category: FocusedCategory,
        id: ObjectId,
    },
    Morphism {
        category: FocusedCategory,
        id: MorphismId,
    },
}
