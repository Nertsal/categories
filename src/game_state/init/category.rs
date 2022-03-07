use ::category::CategoryBuilder;

use super::*;

fn point(label: impl Into<Label>) -> Point {
    Point {
        label: label.into(),
        is_anchor: false,
        position: util::random_shift(),
        velocity: Vec2::ZERO,
        radius: POINT_RADIUS,
        color: Color::WHITE,
    }
}

fn isomorphism(label: impl Into<Label>) -> Arrow {
    Arrow::new(
        Some(label.into()),
        ARROW_ISOMORPHISM_COLOR,
        Vec2::ZERO,
        Vec2::ZERO,
    )
}

pub fn fact_category() -> Category {
    CategoryBuilder::<_, _, _, Label>::new()
        .object("A", vec![], point("A"))
        .object("0", vec![ObjectTag::Initial], point("0"))
        .build()
}

pub fn goal_category() -> Category {
    CategoryBuilder::<_, _, _, Label>::new()
        .object("A", vec![], point("A"))
        .object("0", vec![ObjectTag::Initial], point("0"))
        .object("Ax0", vec![ObjectTag::Product("A", "0")], point("Ax0"))
        .isomorphism("", "0", "Ax0", vec![], isomorphism(""))
        .build()
}
