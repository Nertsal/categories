use super::*;

pub fn constraint_equality<'a, O, M, L: Label>(
    morphism_f: &'a L,
    morphism_g: &'a L,
    bindings: &'a Bindings<L>,
    category: &'a Category<O, M>,
) -> Box<dyn Iterator<Item = Bindings<L>> + 'a> {
    match (
        bindings.get_morphism(morphism_f),
        bindings.get_morphism(morphism_g),
    ) {
        (Some(_), Some(_)) => todo!(),
        (Some(_), None) => todo!(),
        (None, Some(_)) => todo!(),
        (None, None) => todo!(),
    }
}
