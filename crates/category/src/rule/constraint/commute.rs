use super::*;

pub fn constraint_commute<'a, O, M, L: Label>(
    morphism_f: &'a L,
    morphism_g: &'a L,
    morphism_h: &'a L,
    bindings: &'a Bindings<L>,
    category: &'a Category<O, M>,
) -> Box<dyn Iterator<Item = Bindings<L>> + 'a> {
    let constraints = [morphism_f, morphism_g, morphism_h]
        .map(|label| (label.clone(), bindings.get_morphism(label)));

    match constraints {
        [(_, Some(f)), (_, Some(g)), (_, Some(h))] => {
            let mf = category.morphisms.get(&f).unwrap(); // TODO: better error handling?
            let mg = category.morphisms.get(&g).unwrap(); // TODO: better error handling?
            if is_identity(mf)
                .map(|object| {
                    *mg.connection.end_points()[0] == object
                        && (g == h || category.equalities.contains_equality(g, h))
                })
                .or_else(|| {
                    is_identity(mg).map(|object| {
                        *mf.connection.end_points()[1] == object
                            && (f == h || category.equalities.contains_equality(f, h))
                    })
                })
                .unwrap_or(false)
            {
                return Box::new(std::iter::once(Bindings::new()));
            }
        }
        _ => (),
    }

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

fn is_identity<T>(morphism: &Morphism<T>) -> Option<ObjectId> {
    morphism.tags.iter().find_map(|tag| match tag {
        &MorphismTag::Identity(object) => Some(object),
        _ => None,
    })
}
