use super::*;

pub fn rule_product<'a, T: Label + From<&'a str>>() -> Result<Rule<T>, RuleConstructionError> {
    RuleBuilder::new()
        .forall(
            ConstraintsBuilder::new()
                .object("A", vec![])
                .object("B", vec![]),
        )
        .exists(ConstraintsBuilder::new().object("AxB", vec![ObjectTag::Product("A", "B")]))
        .exists(ConstraintsBuilder::new().morphism(
            "id",
            "AxB",
            "AxB",
            vec![MorphismTag::Identity("AxB")],
        ))
        .exists(
            ConstraintsBuilder::new()
                .morphism("p1", "AxB", "A", vec![MorphismTag::ProductP1])
                .morphism("p2", "AxB", "B", vec![MorphismTag::ProductP2]),
        )
        .forall(
            ConstraintsBuilder::new()
                .object("C", vec![])
                .morphism("f", "C", "A", vec![])
                .morphism("g", "C", "B", vec![]),
        )
        .exists(
            ConstraintsBuilder::new()
                .morphism("m", "C", "AxB", vec![])
                .equality(vec!["m", "p1"], vec!["f"])
                .equality(vec!["m", "p2"], vec!["g"]),
        )
        .forall(
            ConstraintsBuilder::new()
                .morphism("m'", "C", "AxB", vec![])
                .equality(vec!["m'", "p1"], vec!["f"])
                .equality(vec!["m'", "p2"], vec!["g"]),
        )
        .exists(ConstraintsBuilder::new().equality(vec!["m"], vec!["m'"]))
        .build()
}
