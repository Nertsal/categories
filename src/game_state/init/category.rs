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

fn arrow(label: impl Into<Label>) -> Arrow {
    Arrow {
        label: label.into(),
        positions: (0..ARROW_BODIES).map(|_| util::random_shift()).collect(),
        velocities: (0..ARROW_BODIES).map(|_| Vec2::ZERO).collect(),
        color: ARROW_REGULAR_COLOR,
    }
}

pub fn fact_category() -> Category {
    CategoryBuilder::<_, _, Label>::new()
        .object("A", vec![], point("A"))
        .object("B", vec![], point("B"))
        .object("C", vec![], point("C"))
        .build()
}

pub fn goal_category() -> Category {
    CategoryBuilder::<_, _, Label>::new()
        .object("A", vec![], point("A"))
        .object("B", vec![], point("B"))
        .object("C", vec![], point("C"))
        .object(
            "AxB",
            vec![ObjectTag::Product("A".into(), "B".into())],
            point("AxB"),
        )
        .object(
            "BxC",
            vec![ObjectTag::Product("B".into(), "C".into())],
            point("BxC"),
        )
        .object(
            "(AxB)xC",
            vec![ObjectTag::Product("AxB".into(), "C".into())],
            point("(AxB)xC"),
        )
        .object(
            "Ax(BxC)",
            vec![ObjectTag::Product("A".into(), "BxC".into())],
            point("Ax(BxC)"),
        )
        .isomorphism("", "Ax(BxC)", "(AxB)xC", vec![], arrow(""))
        .build()
}
