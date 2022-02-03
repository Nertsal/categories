use super::*;

pub fn default_rules(geng: &Geng, assets: &Rc<Assets>) -> Vec<Rule> {
    ::category::axioms::rule_axioms()
}
