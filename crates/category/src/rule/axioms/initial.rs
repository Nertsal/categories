use super::*;

pub fn rule_initial<'a, T: Label + From<&'a str>>() -> Result<Rule<T>, RuleConstructionError> {
    RuleBuilder::new()
        .forall(ConstraintsBuilder::new().object("A", vec![]))
        .forall(ConstraintsBuilder::new().object("0", vec![ObjectTag::Terminal]))
        .exists(ConstraintsBuilder::new().morphism("", "0", "A", vec![MorphismTag::Unique]))
        .build()
}
