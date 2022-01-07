use super::*;

#[derive(Debug, Default, Clone)]
pub struct Bindings {
    objects: HashMap<String, VertexId>,
    morphisms: HashMap<String, EdgeId>,
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
        match label {
            Label::Name(label) => self.objects.insert(label, id),
            Label::Unknown => None,
        }
    }

    pub fn bind_morphism(&mut self, label: Label, id: EdgeId) -> Option<EdgeId> {
        match label {
            Label::Name(label) => self.morphisms.insert(label, id),
            Label::Unknown => None,
        }
    }

    pub fn get_object(&self, label: &Label) -> Option<VertexId> {
        match label {
            Label::Name(label) => self.objects.get(label).copied(),
            Label::Unknown => None,
        }
    }

    pub fn get_morphism(&self, label: &Label) -> Option<EdgeId> {
        match label {
            Label::Name(label) => self.morphisms.get(label).copied(),
            Label::Unknown => None,
        }
    }
}
