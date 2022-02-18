use super::*;

pub fn rule_terminal<'a, T: Label + From<&'a str>>() -> Result<Rule<T>, RuleConstructionError> {
    RuleBuilder::new()
        .forall(ConstraintsBuilder::new().object("A", vec![]))
        .exists(ConstraintsBuilder::new().object("1", vec![ObjectTag::Terminal]))
        .exists(ConstraintsBuilder::new().morphism("", "A", "1", vec![MorphismTag::Unique]))
        .build()
}
