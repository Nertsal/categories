use super::*;

pub fn constraint_equality<'a, O, M, L: Label>(
    morphism_f: &'a L,
    morphism_g: &'a L,
    bindings: &'a Bindings<L>,
    category: &'a Category<O, M>,
) -> Box<dyn Iterator<Item = Bindings<L>> + 'a> {
    let constraints = vec![morphism_f, morphism_g]
        .into_iter()
        .map(|label| (label.clone(), bindings.get_morphism(label)))
        .collect::<Vec<_>>();

    Box::new(
        category
            .equalities
            .all_equalities()
            .filter_map(move |(f, g)| {
                constraint_unordered(constraints.iter().cloned(), vec![f, g])
                    .map(|binds| Bindings::from_morphisms(binds))
            }),
    )
}
