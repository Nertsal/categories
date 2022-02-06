use super::*;

pub fn rule_product<'a, T: Label + From<&'a str>>() -> Result<Rule<T>, RuleConstructionError> {
    RuleBuilder::new()
        .forall(
            ConstraintsBuilder::new()
                .object("A", vec![])
                .object("B", vec![]),
        )
        .exists(ConstraintsBuilder::new().object("AxB", vec![ObjectTag::Product("A", "B")]))
        .exists(
            ConstraintsBuilder::new()
                .morphism("p1", "AxB", "A", vec![])
                .morphism("p2", "AxB", "B", vec![]),
        )
        .forall(
            ConstraintsBuilder::new()
                .object("C", vec![])
                .morphism("f", "C", "A", vec![])
                .morphism("g", "C", "B", vec![]),
        )
        .exists(
            ConstraintsBuilder::new()
                .morphism("m", "C", "AxB", vec![MorphismTag::Unique])
                .commutes("m", "p1", "f")
                .commutes("m", "p2", "g"),
        )
        .forall(
            ConstraintsBuilder::new()
                .morphism("m'", "C", "AxB", vec![])
                .commutes("m'", "p1", "f")
                .commutes("m'", "p2", "g"),
        )
        .exists(ConstraintsBuilder::new().equality("m", "m'"))
        .build()
}
