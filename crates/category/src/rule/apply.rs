use super::*;

impl<O, M> Category<O, M> {
    pub fn apply_rule<L: Label>(&mut self, rule: &Rule<L>) {}
}
