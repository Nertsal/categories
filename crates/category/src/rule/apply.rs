use super::*;

impl<O, M, E> Category<O, M, E> {
    pub fn apply_rule<L: Label>(
        &mut self,
        rule: &Rule<L>,
        bindings: Bindings<L>,
        object_constructor: impl Fn(Vec<ObjectTag<&Object<O>>>) -> O,
        morphism_constructor: impl Fn(
            MorphismConnection<&Object<O>>,
            Vec<MorphismTag<&Object<O>, &Morphism<M>>>,
        ) -> M,
        equality_constructor: impl Fn(&Equality) -> E,
    ) -> (Vec<Action<O, M, E>>, bool) {
        self.apply_impl(
            rule.get_statement(),
            bindings,
            &object_constructor,
            &morphism_constructor,
            &equality_constructor,
        )
    }

    fn apply_impl<L: Label>(
        &mut self,
        statement: &[RuleConstruction<L>],
        bindings: Bindings<L>,
        object_constructor: &impl Fn(Vec<ObjectTag<&Object<O>>>) -> O,
        morphism_constructor: &impl Fn(
            MorphismConnection<&Object<O>>,
            Vec<MorphismTag<&Object<O>, &Morphism<M>>>,
        ) -> M,
        equality_constructor: &impl Fn(&Equality) -> E,
    ) -> (Vec<Action<O, M, E>>, bool) {
        let construction = match statement.first() {
            Some(construction) => construction,
            None => return (Vec::new(), false),
        };

        let statement = &statement[1..];
        match construction {
            RuleConstruction::Forall(constraints) => self
                .find_candidates(constraints, &bindings)
                .map(|candidates| candidates.collect::<Vec<_>>())
                .unwrap_or_else(|| vec![Bindings::new()])
                .into_iter()
                .map(|mut binds| {
                    binds.extend(bindings.clone());
                    self.apply_impl(
                        statement,
                        binds,
                        object_constructor,
                        morphism_constructor,
                        equality_constructor,
                    )
                })
                .fold(
                    (Vec::new(), false),
                    |(mut acc_actions, acc_apply), (action, apply)| {
                        acc_actions.extend(action);
                        (acc_actions, acc_apply || apply)
                    },
                ),
            RuleConstruction::Exists(constraints) => {
                let candidates = self
                    .find_candidates(constraints, &bindings)
                    .map(|candidates| candidates.collect::<Vec<_>>())
                    .unwrap_or_else(|| vec![Bindings::new()]);

                if candidates.is_empty() {
                    let (mut actions, new_binds) = self.apply_constraints(
                        constraints,
                        &bindings,
                        object_constructor,
                        morphism_constructor,
                        equality_constructor,
                    );
                    actions.extend(
                        self.apply_impl(
                            statement,
                            new_binds,
                            object_constructor,
                            morphism_constructor,
                            equality_constructor,
                        )
                        .0,
                    );
                    (actions, true)
                } else {
                    candidates
                        .into_iter()
                        .map(|mut binds| {
                            binds.extend(bindings.clone());
                            // Keep object and morphism constraints to add extra tags
                            let constraints =
                                constraints.iter().filter(|constraint| match constraint {
                                    Constraint::Object { .. } | Constraint::Morphism { .. } => true,
                                    Constraint::Equality(_) => false,
                                });
                            let (mut actions, binds) = self.apply_constraints(
                                constraints,
                                &binds,
                                object_constructor,
                                morphism_constructor,
                                equality_constructor,
                            );
                            let (new_actions, _) = self.apply_impl(
                                statement,
                                binds,
                                object_constructor,
                                morphism_constructor,
                                equality_constructor,
                            );
                            actions.extend(new_actions);
                            (actions, true)
                        })
                        .fold(
                            (vec![], false),
                            |(mut acc_actions, acc_apply), (action, apply)| {
                                acc_actions.extend(action);
                                (acc_actions, acc_apply || apply)
                            },
                        )
                }
            }
        }
    }
}
