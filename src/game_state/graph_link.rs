use super::*;

pub struct GraphLink {
    bindings: Bindings,
}

impl GraphLink {
    pub fn new(main: &Graph, goal: &Graph) -> Self {
        Self {
            bindings: {
                let mut bindings = Bindings::new();
                // Vertices
                for (_, goal_vertex) in goal.graph.vertices.iter() {
                    let label = &goal_vertex.vertex.label;
                    if let Label::Name(_) = label {
                        if let Some(id) = main
                            .graph
                            .vertices
                            .iter()
                            .find(|(_, vertex)| label.matches(&vertex.vertex.label))
                            .map(|(&id, _)| id)
                        {
                            bindings.bind_object(label.clone(), id);
                        }
                    }
                }

                // Edges
                for (_, goal_edge) in goal.graph.edges.iter() {
                    let label = &goal_edge.edge.label;
                    if let Label::Name(_) = label {
                        if let Some(id) = main
                            .graph
                            .edges
                            .iter()
                            .find(|(_, edge)| label.matches(&edge.edge.label))
                            .map(|(&id, _)| id)
                        {
                            bindings.bind_morphism(label.clone(), id);
                        }
                    }
                }

                bindings
            },
        }
    }

    pub fn bindings(&self) -> &Bindings {
        &self.bindings
    }
}
