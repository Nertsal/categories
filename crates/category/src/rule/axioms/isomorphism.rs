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
                .morphism(
                    "g.f",
                    "A",
                    "A",
                    vec![MorphismTag::Composition {
                        first: "f",
                        second: "g",
                    }],
                )
                .morphism(
                    "f.g",
                    "B",
                    "B",
                    vec![MorphismTag::Composition {
                        first: "g",
                        second: "f",
                    }],
                )
                .morphism("id_a", "A", "A", vec![MorphismTag::Identity("A")])
                .morphism("id_b", "B", "B", vec![MorphismTag::Identity("B")])
                .equality("g.f", "id_a")
                .equality("f.g", "id_b"),
        )
        .exists(ConstraintsBuilder::new().isomorphism(
            "",
            "A",
            "B",
            vec![MorphismTag::Isomorphism("f", "g")],
        ))
        .build()
}
