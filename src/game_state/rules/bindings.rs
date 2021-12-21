use super::*;

#[derive(Debug, Default, Clone)]
pub struct Bindings {
    objects: HashMap<Label, VertexId>,
    morphisms: HashMap<Label, EdgeId>,
}

impl Bindings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extend(&mut self, bindings: Self) {
        self.objects.extend(bindings.objects.into_iter());
        self.morphisms.extend(bindings.morphisms.into_iter());
    }

    pub fn bind_object(&mut self, label: Label, id: VertexId) -> Option<VertexId> {
        self.objects.insert(label, id)
    }

    pub fn bind_morphism(&mut self, label: Label, id: EdgeId) -> Option<EdgeId> {
        self.morphisms.insert(label, id)
    }

    pub fn get_object(&self, label: &str) -> Option<VertexId> {
        self.objects.get(label).copied()
    }

    pub fn get_morphism(&self, label: &str) -> Option<EdgeId> {
        self.morphisms.get(label).copied()
    }
}