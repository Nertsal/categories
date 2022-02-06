use super::*;

impl GameState {
    /// Checks whether the goal has been reached
    pub fn check_goal(&mut self) {
        let bindings = self.graph_link.bindings();
        let constraints = self.goal_category.inner.to_constraints();

        if self
            .fact_category
            .inner
            .find_candidates(&constraints, bindings)
            .map(|mut candidates| candidates.next().is_some())
            .unwrap_or(false)
        {
            //  The goal has been reached
            println!("Hooray! Goal reached!"); // TODO: display on the screen
        }
    }
}
