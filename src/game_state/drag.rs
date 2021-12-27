use super::*;

pub struct Dragging {
    pub mouse_start_position: Vec2<f64>,
    pub world_start_position: Vec2<f32>,
    pub action: DragAction,
    pub current_mouse_position: Vec2<f64>,
}

pub enum DragAction {
    Move {
        target: DragTarget,
    },
    Selection {},
    TwoTouchMove {
        initial_camera_fov: f32,
        initial_touch: Vec2<f64>,
        initial_touch_other: Vec2<f64>,
    },
}

pub enum DragTarget {
    GraphCamera {
        graph: FocusedGraph,
        initial_mouse_pos: Vec2<f32>,
        initial_camera_pos: Vec2<f32>,
    },
    Vertex {
        graph: FocusedGraph,
        id: VertexId,
    },
    Edge {
        graph: FocusedGraph,
        id: EdgeId,
    },
}
