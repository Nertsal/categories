use super::*;

#[derive(Debug, Clone)]
pub enum Action {
    NewObjects(Vec<Object>),
    NewMorphisms(Vec<Morphism>),
    RemoveObjects(Vec<ObjectId>),
    RemoveMorphisms(Vec<MorphismId>),
    NewEqualities(Vec<(MorphismId, MorphismId)>),
    RemoveEqualities(Vec<(MorphismId, MorphismId)>),
}

/// Perform the action and returns the inverse action that can be used to undo the action.
pub fn action_do(category: &mut Category, action_do: Action) -> Vec<Action> {
    match action_do {
        Action::NewObjects(objects) => {
            let objects = objects
                .into_iter()
                .map(|object| category.new_object(object))
                .collect();
            vec![Action::RemoveObjects(objects)]
        }
        Action::NewMorphisms(morphisms) => {
            let morphisms = morphisms
                .into_iter()
                .map(|morphism| {
                    category
                        .new_morphism(morphism)
                        .expect("Objects are expected to exist") // TODO: do proper handling
                })
                .collect();
            vec![Action::RemoveMorphisms(morphisms)]
        }
        Action::RemoveObjects(objects) => {
            let (objects, morphisms) = objects
                .into_iter()
                .filter_map(|id| category.remove_object(id))
                .map(|(object, morphisms)| {
                    let morphisms: Vec<_> = morphisms
                        .into_iter()
                        .map(|(_, morphism)| morphism)
                        .collect();
                    (object, morphisms)
                }) // TODO: use `unzip`
                .fold(
                    (Vec::new(), Vec::new()),
                    |(mut acc_objects, mut acc_morphisms), (object, morphisms)| {
                        acc_objects.push(object);
                        acc_morphisms.extend(morphisms);
                        (acc_objects, acc_morphisms)
                    },
                );
            vec![Action::NewMorphisms(morphisms), Action::NewObjects(objects)]
        }
        Action::RemoveMorphisms(morphisms) => {
            let equalities: Vec<_> = morphisms
                .iter()
                .flat_map(|&morphism| {
                    let equals: Vec<_> = category
                        .equalities
                        .get_equalities(morphism)
                        .map(|id| (morphism, id))
                        .collect();
                    equals.iter().for_each(|&(f, g)| {
                        category.equalities.remove_equality(f, g);
                    });
                    equals
                })
                .collect();
            let morphisms: Vec<_> = morphisms
                .into_iter()
                .filter_map(|id| category.remove_morphism(id))
                .collect();
            vec![
                Action::NewEqualities(equalities),
                Action::NewMorphisms(morphisms),
            ]
        }
        Action::NewEqualities(equals) => {
            equals
                .iter()
                .copied()
                .filter(|&(f, g)| f != g)
                .for_each(|(f, g)| {
                    category.equalities.new_equality(f, g);
                });
            vec![Action::RemoveEqualities(equals)]
        }
        Action::RemoveEqualities(equals) => {
            equals.iter().copied().for_each(|(f, g)| {
                category.equalities.remove_equality(f, g);
            });
            vec![Action::NewEqualities(equals)]
        }
    }
}
