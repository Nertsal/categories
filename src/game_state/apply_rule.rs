use super::*;

impl GameState {
    pub fn apply_rule(&mut self, category: FocusedCategory, selection: RuleSelection) {
        let category = match category {
            FocusedCategory::Rule { .. } => unimplemented!(),
            FocusedCategory::Fact => &mut self.fact_category,
            FocusedCategory::Goal => &mut self.goal_category,
        };
        let rule = &self.rules[selection.rule()];
        let (rule, rule_input) = match selection.inverse() {
            Some(inverse) => (&rule.inverse[inverse], &rule.inverse_input),
            None => (&rule.inner, &rule.input),
        };

        let (mut undo_actions, applied) = category.inner.apply_rule(
            rule,
            selection.get_bindings().clone(),
            |tags| {
                let label = tags
                    .into_iter()
                    .find_map(|tag| tag.map(|object| &object.inner.label).infer_name())
                    .unwrap_or_default();
                Point::new(label, Color::WHITE)
            },
            |connection, tags| {
                let color = match connection {
                    MorphismConnection::Isomorphism(_, _) => ARROW_ISOMORPHISM_COLOR,
                    MorphismConnection::Regular { .. } => tags
                        .iter()
                        .find_map(|tag| match tag {
                            MorphismTag::Unique => Some(ARROW_UNIQUE_COLOR),
                            _ => None,
                        })
                        .unwrap_or(ARROW_REGULAR_COLOR),
                };
                let label = tags
                    .into_iter()
                    .find_map(|tag| {
                        tag.map(
                            |object| &object.inner.label,
                            |morphism| &morphism.inner.label,
                        )
                        .infer_name()
                    })
                    .unwrap_or_default();
                Arrow::new(label, color, util::random_shift(), util::random_shift())
            },
            |_equality| Equality {
                color: constants::EQUALITY_FONT_COLOR,
            },
        );

        for action in &undo_actions {
            match action {
                // Morphisms were actually created
                category::Action::RemoveMorphisms(morphisms) => {
                    for morphism_id in morphisms {
                        if let Some(morphism) = category.inner.morphisms.get_mut(morphism_id) {
                            if morphism.inner.label.is_empty() {
                                morphism.inner.label = format!("{:?}", morphism_id.raw());
                            }
                        }
                    }
                }
                // Tags were actually extended
                category::Action::RemoveMorphismTags(extensions) => {
                    for (morphism_id, new_tags) in extensions {
                        if let Some(morphism) = category.inner.morphisms.get_mut(morphism_id) {
                            if new_tags
                                .iter()
                                .any(|tag| matches!(tag, MorphismTag::Unique))
                            {
                                morphism.inner.color = ARROW_UNIQUE_COLOR;
                            }
                        }
                    }
                }
                _ => (),
            }
        }

        if applied {
            if selection.inverse().is_some() {
                let bindings = selection.get_bindings();
                let remove_morphisms = rule_input
                    .iter()
                    .filter_map(|input| match input {
                        RuleInput::Morphism { label, .. } => bindings.get_morphism(label),
                        _ => None,
                    })
                    .filter(|&id| {
                        // Check that there are no equalities with that morphism
                        category
                            .inner
                            .equalities
                            .get_equalities_with(id)
                            .next()
                            .is_none()
                    })
                    .filter_map(|id| category.inner.morphisms.remove(&id))
                    .collect::<Vec<_>>();
                undo_actions.push(CategoryAction::NewMorphisms(remove_morphisms));
            }

            category.action_do(undo_actions);

            if self.check_goal() {
                println!("Hooray! Goal reached!");
                // TODO: display on screen
            }
        }
    }

    /// Checks whether the goal has been reached
    fn check_goal(&self) -> bool {
        let bindings = self.graph_link.bindings();
        let constraints = self.goal_category.inner.to_constraints();

        self.fact_category
            .inner
            .find_candidates(&constraints, bindings)
            .map(|mut candidates| candidates.next().is_some())
            .unwrap_or(false)
    }
}
