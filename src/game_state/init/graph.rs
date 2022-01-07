use super::*;

pub fn main_graph() -> Graph {
    GraphBuilder::new()
        .object("A", None, Color::WHITE, false)
        .object("B", None, Color::WHITE, false)
        .object("C", None, Color::WHITE, false)
        .build()
}

pub fn goal_graph() -> Graph {
    GraphBuilder::new()
        .object("A", None, Color::WHITE, false)
        .object("B", None, Color::WHITE, false)
        .object("C", None, Color::WHITE, false)
        .object(
            "AxB",
            Some(ObjectTag::Product("A".into(), "B".into())),
            Color::WHITE,
            false,
        )
        .object(
            "BxC",
            Some(ObjectTag::Product("B".into(), "C".into())),
            Color::WHITE,
            false,
        )
        .object(
            "(AxB)xC",
            Some(ObjectTag::Product("AxB".into(), "C".into())),
            Color::WHITE,
            false,
        )
        .object(
            "Ax(BxC)",
            Some(ObjectTag::Product("A".into(), "BxC".into())),
            Color::WHITE,
            false,
        )
        .morphism(
            Label::Unknown,
            "Ax(BxC)",
            "(AxB)xC",
            Some(MorphismTag::Isomorphism(Label::Unknown, Label::Unknown)),
        )
        .build()
}
