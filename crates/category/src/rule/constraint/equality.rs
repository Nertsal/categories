use super::*;

pub fn constraint_equality<'a, O, M, L: Label>(
    equality: &'a Equality<L>,
    bindings: &'a Bindings<L>,
    category: &'a Category<O, M>,
) -> Box<dyn Iterator<Item = Bindings<L>> + 'a> {
    let [left_constraints, right_constraints] =
        [equality.left(), equality.right()].map(|eq_side| {
            eq_side
                .iter()
                .map(|label| (label.clone(), bindings.get_morphism(label)))
                .collect::<Vec<_>>()
        });

    // match constraints {
    //     [(_, Some(f)), (_, Some(g))] if f == g => {
    //         return Box::new(std::iter::once(Bindings::new()))
    //     }
    //     _ => (),
    // }

    Box::new(
        category
            .equalities
            .all_equalities()
            .flat_map(move |equality| {
                vec![
                    constraint_ordered(left_constraints.clone(), equality.left().clone()).and_then(
                        |left_binds| {
                            constraint_ordered(right_constraints.clone(), equality.right().clone())
                                .map(|right_binds| {
                                    Bindings::from_morphisms(left_binds.chain(right_binds))
                                })
                        },
                    ),
                    constraint_ordered(right_constraints.clone(), equality.left().clone())
                        .and_then(|left_binds| {
                            constraint_ordered(left_constraints.clone(), equality.right().clone())
                                .map(|right_binds| {
                                    Bindings::from_morphisms(left_binds.chain(right_binds))
                                })
                        }),
                ]
                .into_iter()
                .filter_map(|x| x)
            }),
    )
}
