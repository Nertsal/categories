use super::*;

pub fn rule_identity<'a, T: Label + From<&'a str>>() -> Result<Rule<T>, RuleConstructionError> {
    RuleBuilder::new()
        .forall(ConstraintsBuilder::new().object("A", vec![]))
        .exists(ConstraintsBuilder::new().morphism(
            "id",
            "A",
            "A",
            vec![MorphismTag::Identity("A")],
        ))
        .build()
}
