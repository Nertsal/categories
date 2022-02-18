use super::*;

impl GameState {
    pub fn apply_rule(&mut self, category: FocusedCategory, selection: RuleSelection) {
        let category = match category {
            FocusedCategory::Rule { .. } => unimplemented!(),
            FocusedCategory::Fact => &mut self.fact_category,
            FocusedCategory::Goal => &mut self.goal_category,
        };
        let rule = &self.rules[selection.rule()];
        let rule = match selection.inverse() {
            Some(inverse) => &rule.inverse[inverse],
            None => &rule.inner,
        };

        let (undo_actions, applied) = category.inner.apply_rule(
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
        );

        for action in &undo_actions {
            match action {
                category::Action::RemoveMorphisms(morphisms) => {
                    for morphism_id in morphisms {
                        if let Some(morphism) = category.inner.morphisms.get_mut(morphism_id) {
                            if morphism.inner.label.is_empty() {
                                morphism.inner.label = format!("{:?}", morphism_id.raw());
                            }
                        }
                    }
                }
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

        category.action_history.extend(undo_actions);

        if applied {
            if selection.inverse().is_some() {
                // TODO: smarter removal
                for morphism in selection.get_bindings().morphisms.values() {
                    category.inner.morphisms.remove(morphism);
                }
            }

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
