use super::*;

pub fn rule_isomorphism<'a, T: Label + From<&'a str>>() -> Result<Rule<T>, RuleConstructionError> {
    RuleBuilder::new()
        .forall(
            ConstraintsBuilder::new()
                .morphism("f", "A", "B", vec![])
                .morphism("g", "B", "A", vec![]),
        )
        .forall(
            ConstraintsBuilder::new()
                .morphism("id_a", "A", "A", vec![MorphismTag::Identity("A")])
                .morphism("id_b", "B", "B", vec![MorphismTag::Identity("B")])
                .commutes("f", "g", "id_a")
                .commutes("g", "f", "id_b"),
        )
        .exists(ConstraintsBuilder::new().isomorphism(
            "",
            "A",
            "B",
            vec![MorphismTag::Isomorphism("f", "g")],
        ))
        .build()
}
