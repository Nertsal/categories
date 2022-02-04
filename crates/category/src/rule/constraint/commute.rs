use super::*;

pub fn constraint_commute<'a, O, M, L: Label>(
    morphism_f: &'a L,
    morphism_g: &'a L,
    morphism_h: &'a L,
    bindings: &'a Bindings<L>,
    category: &'a Category<O, M>,
) -> Box<dyn Iterator<Item = Bindings<L>> + 'a> {
    let constraints = vec![morphism_f, morphism_g, morphism_h]
        .into_iter()
        .map(|label| (label.clone(), bindings.get_morphism(label)))
        .collect::<Vec<_>>();

    Box::new(
        category
            .equalities
            .all_commutes()
            .filter_map(move |(f, g, h)| {
                constraint_ordered(constraints.iter().cloned(), vec![f, g, h])
                    .map(|binds| Bindings::from_morphisms(binds))
            }),
    )
}
