use super::*;

pub fn selection_constraints(
    selection: &Vec<CategoryThing>,
    constraints: &Constraints,
    category: &Category,
    equalities: &Equalities,
) -> Result<Bindings, ()> {
    let mut selection = selection.iter();
    let mut bindings = Bindings::new();

    fn bind_object(bindings: &mut Bindings, label: &Label, constraint: ObjectId) -> bool {
        match bindings.get_object(label) {
            Some(object) => object == constraint,
            None => {
                bindings.bind_object(label.clone(), constraint);
                true
            }
        }
    }

    for constraint in constraints {
        match constraint {
            Constraint::RuleObject(label, object) => match object {
                RuleObject::Object { .. } => match selection.next() {
                    Some(CategoryThing::Object { id }) => {
                        if bindings.bind_object(label.clone(), *id).is_some() {
                            return Err(());
                        }
                    }
                    _ => return Err(()),
                },
                RuleObject::Morphism { constraint } => match selection.next() {
                    Some(CategoryThing::Morphism { id }) => {
                        let morphism = category.morphisms.get(id).unwrap();

                        let objects = match (morphism.connection, &constraint.connection) {
                            (
                                MorphismConnection::Regular { from, to },
                                MorphismConnection::Regular {
                                    from: constraint_from,
                                    to: constraint_to,
                                },
                            ) => [(constraint_from, from), (constraint_to, to)],
                            (
                                MorphismConnection::Isomorphism(a, b),
                                MorphismConnection::Isomorphism(constraint_a, constraint_b),
                            ) => [(constraint_a, a), (constraint_b, b)],
                            _ => return Err(()),
                        };

                        if !objects
                            .into_iter()
                            .all(|(label, id)| bind_object(&mut bindings, label, id))
                        {
                            return Err(());
                        }

                        bindings.bind_morphism(label.clone(), *id);
                    }
                    _ => return Err(()),
                },
            },
            Constraint::MorphismEq(f, g) => {
                if !bindings
                    .get_morphism(f)
                    .and_then(|f| {
                        bindings
                            .get_morphism(g)
                            .map(|g| equalities.contains(&(f, g)))
                    })
                    .unwrap_or(false)
                {
                    return Err(());
                }
            }
        }
    }

    Ok(bindings)
}
