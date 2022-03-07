mod composition;
mod identity;
mod initial;
mod isomorphism;
mod product;
mod terminal;
mod unique;

use super::*;

pub use composition::*;
pub use identity::*;
pub use initial::*;
pub use isomorphism::*;
pub use product::*;
pub use terminal::*;
pub use unique::*;

pub fn rule_axioms<'a, T: Label + From<&'a str>>() -> Vec<Rule<T>> {
    get_axioms().expect("Axioms are expected to be valid")
}

fn get_axioms<'a, T: Label + From<&'a str>>() -> Result<Vec<Rule<T>>, RuleConstructionError> {
    Ok(vec![
        rule_identity()?,
        rule_composition()?,
        // rule_terminal()?,
        rule_initial()?,
        rule_product()?,
        rule_unique()?,
        rule_isomorphism()?,
    ])
}
