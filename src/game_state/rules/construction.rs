use super::*;

pub enum RuleConstruction {
    Forall(Constraints),
    Exists(Constraints),
    SuchThat,
}

pub type Constraints = Vec<Constraint>;

pub enum Constraint {
    RuleObject(Label, RuleObject),
    MorphismEq(Label, Label),
}
