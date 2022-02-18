use super::*;

pub fn rule_commutativity<'a, T: Label + From<&'a str>>() -> Result<Rule<T>, RuleConstructionError>
{
    RuleBuilder::new()
        .forall(
            ConstraintsBuilder::new()
                .commutes("f", "l", "h")
                .commutes("g", "k", "l"),
        )
        .forall(
            ConstraintsBuilder::new()
                .morphism("f", "A", "B", vec![])
                .morphism("g", "B", "C", vec![])
                .morphism("h", "A", "D", vec![])
                .morphism("k", "C", "D", vec![])
                .morphism("l", "B", "D", vec![]),
        )
        .exists(
            ConstraintsBuilder::new()
                .morphism(
                    "g o f",
                    "A",
                    "C",
                    vec![MorphismTag::Composition {
                        first: "f",
                        second: "g",
                    }],
                )
                .commutes("g o f", "k", "h"),
        )
        .build()
}
