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

    pub fn bind_object(&mut self, label: RuleLabel, id: VertexId) -> Option<VertexId> {
        match label {
            RuleLabel::Name(label) => self.objects.insert(label, id),
            RuleLabel::Any => None,
        }
    }

    pub fn bind_morphism(&mut self, label: RuleLabel, id: EdgeId) -> Option<EdgeId> {
        match label {
            RuleLabel::Name(label) => self.morphisms.insert(label, id),
            RuleLabel::Any => None,
        }
    }

    pub fn get_object(&self, label: &RuleLabel) -> Option<VertexId> {
        match label {
            RuleLabel::Name(label) => self.objects.get(label).copied(),
            RuleLabel::Any => None,
        }
    }

    pub fn get_morphism(&self, label: &RuleLabel) -> Option<EdgeId> {
        match label {
            RuleLabel::Name(label) => self.morphisms.get(label).copied(),
            RuleLabel::Any => None,
        }
    }
}
