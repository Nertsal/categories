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
        (Some(morphism_f), Some(morphism_g)) => {
            if morphism_f == morphism_g
                || category.equalities.check_equality(morphism_f, morphism_g)
            {
                Box::new(std::iter::once(Bindings::new()))
            } else {
                Box::new(vec![].into_iter())
            }
        }
        (Some(morphism_f), None) => Box::new(
            category
                .equalities
                .get_equalities(morphism_f)
                .chain(std::iter::once(morphism_f))
                .map(|id| Bindings::single_morphism(morphism_g.clone(), id)),
        ),
        (None, Some(morphism_g)) => Box::new(
            category
                .equalities
                .get_equalities(morphism_g)
                .chain(std::iter::once(morphism_g))
                .map(|id| Bindings::single_morphism(morphism_f.clone(), id)),
        ),
        (None, None) => Box::new(
            category
                .equalities
                .all_equalities()
                .chain(category.morphisms.iter().map(|(&id, _)| (id, id)))
                .map(|(f, g)| {
                    let mut binds = Bindings::new();
                    binds.bind_morphism(morphism_f.clone(), f);
                    binds.bind_morphism(morphism_g.clone(), g);
                    binds
                }),
        ),
    }
}
