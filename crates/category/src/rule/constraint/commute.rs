use super::*;

pub fn constraint_commute<'a, O, M, L: Label>(
    morphism_f: &'a L,
    morphism_g: &'a L,
    morphism_h: &'a L,
    bindings: &'a Bindings<L>,
    category: &'a Category<O, M>,
) -> Box<dyn Iterator<Item = Bindings<L>> + 'a> {
    todo!()
}
