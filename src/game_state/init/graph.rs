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
            vec![ObjectTag::Product("A", "B")],
            Color::WHITE,
            false,
        )
        .object(
            "BxC",
            vec![ObjectTag::Product("B", "C")],
            Color::WHITE,
            false,
        )
        .object(
            "(AxB)xC",
            vec![ObjectTag::Product("AxB", "C")],
            Color::WHITE,
            false,
        )
        .object(
            "Ax(BxC)",
            vec![ObjectTag::Product("A", "BxC")],
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
