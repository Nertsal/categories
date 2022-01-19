use super::*;

impl GameState {
    /// Checks whether the goal has been reached
    pub fn check_goal(&mut self) {
        let bindings = self.graph_link.bindings();
        let constraints = category_to_constraints(&self.goal_category.inner);

        if find_candidates(
            &constraints,
            bindings,
            &self.fact_category.inner,
            &self.fact_category.equalities,
        )
        .map(|mut candidates| candidates.next().is_some())
        .unwrap_or(false)
        {
            //  The goal has been reached
            println!("Hooray! Goal reached!");
        }
    }
}

fn category_to_constraints(category: &Category) -> Constraints {
    let get_object_label = |id: ObjectId| {
        category
            .objects
            .get(&id)
            .map(|object| &object.label)
            .unwrap()
            .clone()
    };

    let get_morphism_label = |id: MorphismId| {
        category
            .morphisms
            .get(&id)
            .map(|morphism| &morphism.inner.label)
            .unwrap()
            .clone()
    };

    category
        .objects
        .iter()
        .map(|(_, object)| {
            Constraint::RuleObject(
                object.label.clone(),
                RuleObject::Object {
                    tag: object.tag.as_ref().map(|tag| {
                        tag.map_borrowed(|object| object.map(|object| get_object_label(object)))
                    }),
                },
            )
        })
        .chain(category.morphisms.iter().map(|(_, morphism)| {
            let connection = match morphism.connection {
                MorphismConnection::Regular { from, to } => MorphismConnection::Regular {
                    from: get_object_label(from),
                    to: get_object_label(to),
                },
                MorphismConnection::Isomorphism(a, b) => {
                    MorphismConnection::Isomorphism(get_object_label(a), get_object_label(b))
                }
            };

            Constraint::RuleObject(
                morphism.inner.label.clone(),
                RuleObject::Morphism {
                    constraint: ArrowConstraint {
                        connection,
                        tag: morphism.inner.tag.as_ref().map(|tag| {
                            tag.map_borrowed(
                                |object| object.map(|object| get_object_label(object)),
                                |edge| edge.map(|edge| get_morphism_label(edge)),
                            )
                        }),
                    },
                },
            )
        }))
        .collect()
}
