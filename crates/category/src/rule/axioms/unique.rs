use super::*;

pub fn rule_unique<'a, T: Label + From<&'a str>>() -> Result<Rule<T>, RuleConstructionError> {
    RuleBuilder::new()
        .forall(ConstraintsBuilder::new().morphism("f", "A", "B", vec![]))
        .forall(ConstraintsBuilder::new().morphism("m", "A", "B", vec![MorphismTag::Unique]))
        .exists(ConstraintsBuilder::new().equality("f", "m"))
        .build()
}
