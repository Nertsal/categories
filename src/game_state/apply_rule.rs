use super::*;

impl GameState {
    pub fn apply_rule(&mut self, category: FocusedCategory, selection: RuleSelection) {
        let category = match category {
            FocusedCategory::Rule { .. } => unimplemented!(),
            FocusedCategory::Fact => &mut self.fact_category,
            FocusedCategory::Goal => &mut self.goal_category,
        };
        let rule = &self.rules[selection.rule()].inner;

        let (actions, applied) = category.inner.apply_rule(
            rule,
            selection.into_bindings(),
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

        category.action_history.extend(actions);

        if applied && self.check_goal() {
            println!("Hooray! Goal reached!");
            // TODO: display on screen
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
