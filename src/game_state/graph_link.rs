use super::*;

pub struct GraphLink {
    bindings: category::Bindings<CategoryThing>,
}

impl GraphLink {
    pub fn new(fact: &Category, goal: &Category) -> Self {
        Self {
            bindings: {
                let mut bindings = category::Bindings::new();
                // Vertices
                for (&id, object) in goal.objects.iter() {
                    let label = &object.inner.label;
                    if let Some(fact_id) = fact
                        .objects
                        .iter()
                        .find(|(_, object)| label.eq(&object.inner.label))
                        .map(|(&id, _)| id)
                    {
                        bindings.bind_object(CategoryThing::Object { id }, fact_id);
                    }
                }

                // Edges
                for (&id, morphism) in goal.morphisms.iter() {
                    let label = &morphism.inner.label;
                    if let Some(fact_id) = fact
                        .morphisms
                        .iter()
                        .find(|(_, morphism)| label.eq(&morphism.inner.label))
                        .map(|(&id, _)| id)
                    {
                        bindings.bind_morphism(CategoryThing::Morphism { id }, fact_id);
                    }
                }

                bindings
            },
        }
    }

    pub fn bindings(&self) -> &category::Bindings<CategoryThing> {
        &self.bindings
    }
}
