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
        .object("1", vec![ObjectTag::Terminal], point("1"))
        .build()
}

pub fn goal_category() -> Category {
    CategoryBuilder::<_, _, _, Label>::new()
        .object("A", vec![], point("A"))
        .object("1", vec![ObjectTag::Terminal], point("1"))
        .object("Ax1", vec![ObjectTag::Product("A", "1")], point("Ax1"))
        .isomorphism("", "1", "Ax1", vec![], isomorphism(""))
        .build()
}
