use super::*;

pub const SCROLL_SPEED: f32 = 0.1;

pub const ZOOM_SPEED: f32 = 0.1;
pub const CAMERA_FOV_MIN: f32 = 25.0;
pub const CAMERA_FOV_MAX: f32 = 200.0;

pub const POINT_OUTLINE_WIDTH: f32 = 0.2;
pub const POINT_RADIUS: f32 = 2.0;
pub const POINT_MASS: f32 = 10.0;

pub const ARROW_WIDTH: f32 = 0.4;
pub const ARROW_MASS: f32 = 1.0;
pub const ARROW_BODIES: usize = 3;

pub const CHAIN_OUTLINE_WIDTH: f32 = 0.8;

pub const ARROW_BEST_COLOR: Color<f32> = Color::GREEN;
pub const ARROW_REGULAR_COLOR: Color<f32> = Color::BLUE;
pub const ARROW_UNIQUE_COLOR: Color<f32> = Color::RED;
pub const ARROW_ISOMORPHISM_COLOR: Color<f32> = Color::RED;

pub const SELECTION_RADIUS: f32 = 0.5;

pub const ARROW_LABEL_FONT_SIZE: f32 = 2.0;
pub const ARROW_ICON_RADIUS: f32 = 1.5;
pub const ARROW_ICON_OUTLINE_WIDTH: f32 = 0.2;

pub const RULE_RESOLUTION: Vec2<usize> = vec2(1024, 768);
pub const RULES_WIDTH_FRAC: f32 = 0.3;
pub const RULES_SECTION_SEPARATION_WIDTH_FRAC: f32 = 0.005;
pub const RULE_SEPARATION_WIDTH_FRAC: f32 = 0.005;
pub const RULES_SECTION_SEPARATION_COLOR: Color<f32> = Color::GRAY;
pub const RULE_SEPARATION_COLOR: Color<f32> = Color::GRAY;
pub const RULE_SELECTION_COLOR: Color<f32> = Color {
    r: 0.1,
    g: 0.1,
    b: 0.1,
    a: 1.0,
};

pub const ARROW_HEAD_WIDTH: f32 = 0.5;
pub const ARROW_HEAD_LENGTH: f32 = 2.0;
pub const ARROW_LENGTH_MAX_FRAC: f32 = 0.5;

pub const ARROW_DASHED_DASH_LENGTH: f32 = 0.7;
pub const ARROW_DASHED_SPACE_LENGTH: f32 = 0.3;
pub const ARROW_DASH_FULL_LENGTH: f32 = ARROW_DASHED_DASH_LENGTH + ARROW_DASHED_SPACE_LENGTH;

pub const CURVE_RESOLUTION: usize = 5;

pub const SELECTION_COLOR: Color<f32> = Color {
    r: 0.0,
    g: 0.0,
    b: 0.5,
    a: 0.5,
};
pub const SELECTED_RADIUS: f32 = 0.5;
pub const SELECTED_COLOR: Color<f32> = Color {
    r: 0.7,
    g: 0.7,
    b: 0.7,
    a: 1.0,
};

pub const RULE_INPUT_COLOR: Color<f32> = Color::BLUE;
pub const RULE_INFER_CONTEXT_COLOR: Color<f32> = Color::MAGENTA;
pub const RULE_INFER_COLOR: Color<f32> = Color::GREEN;
pub const RULE_OUTPUT_COLOR: Color<f32> = Color::RED;
