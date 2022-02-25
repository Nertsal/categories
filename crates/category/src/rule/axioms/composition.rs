use super::*;

pub fn rule_composition<'a, T: Label + From<&'a str>>() -> Result<Rule<T>, RuleConstructionError> {
    RuleBuilder::new()
        .forall(
            ConstraintsBuilder::new()
                .morphism("f", "A", "B", vec![])
                .morphism("g", "B", "C", vec![]),
        )
        .exists(ConstraintsBuilder::new().morphism(
            "g.f",
            "A",
            "C",
            vec![MorphismTag::Composition {
                first: "f",
                second: "g",
            }],
        ))
        .build()
}
