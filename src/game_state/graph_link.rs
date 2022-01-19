use super::*;

pub struct GraphLink {
    bindings: Bindings,
}

impl GraphLink {
    pub fn new(fact: &Category, goal: &Category) -> Self {
        Self {
            bindings: {
                let mut bindings = Bindings::new();
                // Vertices
                for (_, object) in goal.objects.iter() {
                    let label = &object.label;
                    if let Label::Name(_) = label {
                        if let Some(id) = fact
                            .objects
                            .iter()
                            .find(|(_, object)| label.matches(&object.label))
                            .map(|(&id, _)| id)
                        {
                            bindings.bind_object(label.clone(), id);
                        }
                    }
                }

                // Edges
                for (_, morphism) in goal.morphisms.iter() {
                    let label = &morphism.inner.label;
                    if let Label::Name(_) = label {
                        if let Some(id) = fact
                            .morphisms
                            .iter()
                            .find(|(_, morphism)| label.matches(&morphism.inner.label))
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
