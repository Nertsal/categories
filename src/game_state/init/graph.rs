use super::*;

pub fn main_graph() -> Graph {
    GraphBuilder::new()
        .object("A", vec![], Color::WHITE, false)
        .object("B", vec![], Color::WHITE, false)
        .object("C", vec![], Color::WHITE, false)
        .build()
}

pub fn goal_graph() -> Graph {
    GraphBuilder::new()
        .object("A", vec![], Color::WHITE, false)
        .object("B", vec![], Color::WHITE, false)
        .object("C", vec![], Color::WHITE, false)
        .object(
            "AxB",
            vec![ObjectTag::Product("A".into(), "B".into())],
            Color::WHITE,
            false,
        )
        .object(
            "BxC",
            vec![ObjectTag::Product("B".into(), "C".into())],
            Color::WHITE,
            false,
        )
        .object(
            "(AxB)xC",
            vec![ObjectTag::Product("AxB".into(), "C".into())],
            Color::WHITE,
            false,
        )
        .object(
            "Ax(BxC)",
            vec![ObjectTag::Product("A".into(), "BxC".into())],
            Color::WHITE,
            false,
        )
        .morphism(
            "",
            "Ax(BxC)",
            "(AxB)xC",
            vec![MorphismTag::Isomorphism(Label::Any, Label::Any)],
        )
        .build()
}
