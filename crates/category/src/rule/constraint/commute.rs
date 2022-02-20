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
            .chain(category.morphisms.iter().flat_map(|(&id, morphism)| {
                let [&from, &to] = morphism.connection.end_points();
                let mut left_id = None;
                let mut right_id = None;
                for (&id, id_morphism) in category.morphisms.iter() {
                    let [&id_from, &id_to] = id_morphism.connection.end_points();
                    if from == id_from && is_identity(id_morphism).is_some() {
                        left_id = Some(id);
                        if right_id.is_some() {
                            break;
                        }
                    } else if to == id_to && is_identity(id_morphism).is_some() {
                        right_id = Some(id);
                        if left_id.is_some() {
                            break;
                        }
                    }
                }
                let commutes = vec![
                    left_id.map(|left_id| (left_id, id, id)),
                    right_id.map(|right_id| (id, right_id, id)),
                    is_composition(morphism).map(|(f, g)| (f, g, id)),
                ];

                commutes.into_iter().filter_map(|x| x)
            }))
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

fn is_composition<T>(morphism: &Morphism<T>) -> Option<(MorphismId, MorphismId)> {
    morphism.tags.iter().find_map(|tag| match tag {
        &MorphismTag::Composition { first, second } => Some((first, second)),
        _ => None,
    })
}
