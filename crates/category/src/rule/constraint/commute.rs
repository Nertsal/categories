use super::*;

pub fn constraint_commute<'a, L: Label>(
    morphism_f: &'a L,
    morphism_g: &'a L,
    morphism_h: &'a L,
    bindings: &'a Bindings<L>,
    category: &'a Category,
) -> Box<dyn Iterator<Item = Bindings<L>> + 'a> {
    todo!()
}
