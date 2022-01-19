use super::*;

#[derive(Debug, Clone)]
pub enum GraphAction {
    NewObjects(Vec<(Label, Option<ObjectTag<Option<ObjectId>>>)>),
    NewMorphisms(Vec<(Label, ArrowConstraint<ObjectId, MorphismId>)>),
    RemoveObjects(Vec<ObjectId>),
    RemoveMorphisms(Vec<MorphismId>),
    NewEqualities(Vec<(MorphismId, MorphismId)>),
    RemoveEqualities(Vec<(MorphismId, MorphismId)>),
}

/// Perform the action and returns the inverse action
pub fn action_do(
    category: &mut Category,
    equalities: &mut Equalities,
    action_do: GraphAction,
) -> Vec<GraphAction> {
    match action_do {
        GraphAction::NewObjects(objects) => {
            let objects = objects
                .into_iter()
                .map(|(label, tag)| {
                    let id = category.new_object(Point::new(
                        label,
                        tag,
                        util::random_shift(),
                        false,
                        Color::WHITE,
                    ));
                    id
                })
                .collect();
            vec![GraphAction::RemoveObjects(objects)]
        }
        GraphAction::NewMorphisms(morphisms) => {
            let morphisms = morphisms
                .into_iter()
                .map(|(label, constraint)| {
                    let (pos_a, pos_b) = match &constraint.connection {
                        MorphismConnection::Regular { from, to } => {
                            let from = category.objects.get(from).expect(todo!());
                            let to = category.objects.get(to).expect(todo!());
                            (from.position, to.position)
                        }
                        MorphismConnection::Isomorphism(a, b) => {
                            let a = category.objects.get(a).expect(todo!());
                            let b = category.objects.get(b).expect(todo!());
                            (a.position, b.position)
                        }
                    };
                    let pos_a = pos_a + util::random_shift();
                    let pos_b = pos_b + util::random_shift();

                    let connection = constraint.connection;
                    let tag = constraint.tag;
                    let color = draw::category::morphism_color(&tag);
                    let id = category
                        .new_morphism(Morphism {
                            connection,
                            inner: Arrow::new(label, tag, color, pos_a, pos_b),
                        })
                        .expect("Clearly impossible because the positions of the objects have been received at this point");
                    id
                })
                .collect();
            vec![GraphAction::RemoveMorphisms(morphisms)]
        }
        GraphAction::RemoveObjects(objects) => {
            let (objects, morphisms) = objects
                .into_iter()
                .filter_map(|id| category.remove_object(id))
                .map(|(object, morphisms)| {
                    let object = (object.label, object.tag);
                    let morphisms: Vec<_> = morphisms
                        .into_iter()
                        .map(|(_, morphism)| {
                            (
                                morphism.inner.label,
                                ArrowConstraint {
                                    connection: morphism.connection,
                                    tag: morphism.inner.tag,
                                },
                            )
                        })
                        .collect();
                    (object, morphisms)
                })
                .fold(
                    (Vec::new(), Vec::new()),
                    |(mut acc_objects, mut acc_morphisms), (object, morphisms)| {
                        acc_objects.push(object);
                        acc_morphisms.extend(morphisms);
                        (acc_objects, acc_morphisms)
                    },
                );
            vec![
                GraphAction::NewMorphisms(morphisms),
                GraphAction::NewObjects(objects),
            ]
        }
        GraphAction::RemoveMorphisms(morphisms) => {
            let equalities: Vec<_> = morphisms
                .iter()
                .flat_map(|&morphism| {
                    let equals: Vec<_> = equalities
                        .iter()
                        .filter(move |&&(f, g)| f == morphism || g == morphism)
                        .copied()
                        .collect();
                    equals.iter().for_each(|equality| {
                        equalities.remove(equality);
                    });
                    equals
                })
                .collect();
            let morphisms: Vec<_> = morphisms
                .into_iter()
                .filter_map(|id| category.remove_morphism(id))
                .map(|morphism| {
                    (
                        morphism.inner.label,
                        ArrowConstraint {
                            connection: morphism.connection,
                            tag: morphism.inner.tag,
                        },
                    )
                })
                .collect();
            vec![
                GraphAction::NewEqualities(equalities),
                GraphAction::NewMorphisms(morphisms),
            ]
        }
        GraphAction::NewEqualities(equals) => {
            equals
                .iter()
                .copied()
                .filter(|&(f, g)| f != g)
                .for_each(|(f, g)| {
                    equalities.insert((f, g));
                });
            vec![GraphAction::RemoveEqualities(equals)]
        }
        GraphAction::RemoveEqualities(equals) => {
            equals.iter().copied().for_each(|(f, g)| {
                equalities.remove(&(f, g));
            });
            vec![GraphAction::NewEqualities(equals)]
        }
    }
}

impl GameState {
    pub fn action_undo(&mut self) {
        if let Some(action) = self.action_history.pop() {
            action_do(
                &mut self.fact_category.inner,
                &mut self.fact_category.equalities,
                action,
            );
        }
    }
}
