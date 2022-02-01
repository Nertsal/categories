use super::*;

pub fn constraint_commute<'a, O, M, L: Label>(
    morphism_f: &'a L,
    morphism_g: &'a L,
    morphism_h: &'a L,
    bindings: &'a Bindings<L>,
    category: &'a Category<O, M>,
) -> Box<dyn Iterator<Item = Bindings<L>> + 'a> {
    match (
        bindings.get_morphism(morphism_f),
        bindings.get_morphism(morphism_g),
        bindings.get_morphism(morphism_h),
    ) {
        (Some(_), Some(_), Some(_)) => todo!(),
        (Some(_), Some(_), None) => todo!(),
        (Some(_), None, Some(_)) => todo!(),
        (None, Some(_), Some(_)) => todo!(),
        (None, Some(_), None) => todo!(),
        (Some(_), None, None) => todo!(),
        (None, None, Some(_)) => todo!(),
        (None, None, None) => todo!(),
    }
}
