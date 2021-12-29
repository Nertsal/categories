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
        .morphism(Label::Any, "AxB", "A", None)
        .morphism(Label::Any, "AxB", "B", None)
        .morphism(Label::Any, "BxC", "B", None)
        .morphism(Label::Any, "BxC", "C", None)
        .morphism(Label::Any, "(AxB)xC", "AxB", None)
        .morphism(Label::Any, "(AxB)xC", "C", None)
        .morphism(Label::Any, "Ax(BxC)", "A", None)
        .morphism(Label::Any, "Ax(BxC)", "BxC", None)
        .morphism(
            Label::Any,
            "Ax(BxC)",
            "(AxB)xC",
            Some(MorphismTag::Isomorphism(Label::Any, Label::Any)),
        )
        .build()
}
