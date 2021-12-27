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
            vec![ObjectTag::Product(Some("A".into()), Some("B".into()))],
            Color::WHITE,
            false,
        )
        .object(
            "BxC",
            vec![ObjectTag::Product(Some("B".into()), Some("C".into()))],
            Color::WHITE,
            false,
        )
        .object(
            "(AxB)xC",
            vec![ObjectTag::Product(Some("AxB".into()), Some("C".into()))],
            Color::WHITE,
            false,
        )
        .object(
            "Ax(BxC)",
            vec![ObjectTag::Product(Some("A".into()), Some("BxC".into()))],
            Color::WHITE,
            false,
        )
        .morphism("", "AxB", "A", vec![])
        .morphism("", "AxB", "B", vec![])
        .morphism("", "BxC", "B", vec![])
        .morphism("", "BxC", "C", vec![])
        .morphism("", "(AxB)xC", "AxB", vec![])
        .morphism("", "(AxB)xC", "C", vec![])
        .morphism("", "Ax(BxC)", "A", vec![])
        .morphism("", "Ax(BxC)", "BxC", vec![])
        .build()
}
