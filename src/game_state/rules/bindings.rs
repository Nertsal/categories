use super::*;

#[derive(Debug, Default, Clone)]
pub struct Bindings {
    objects: HashMap<String, ObjectId>,
    morphisms: HashMap<String, MorphismId>,
}

impl Bindings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn single_morphism(label: Label, id: MorphismId) -> Self {
        let mut binds = Self::default();
        binds.bind_morphism(label, id);
        binds
    }

    pub fn extend(&mut self, bindings: Self) {
        self.objects.extend(bindings.objects.into_iter());
        self.morphisms.extend(bindings.morphisms.into_iter());
    }

    pub fn bind_object(&mut self, label: Label, id: ObjectId) -> Option<ObjectId> {
        match label {
            Label::Name(label) => self.objects.insert(label, id),
            Label::Unknown => None,
        }
    }

    pub fn bind_morphism(&mut self, label: Label, id: MorphismId) -> Option<MorphismId> {
        match label {
            Label::Name(label) => self.morphisms.insert(label, id),
            Label::Unknown => None,
        }
    }

    pub fn get_object(&self, label: &Label) -> Option<ObjectId> {
        match label {
            Label::Name(label) => self.objects.get(label).copied(),
            Label::Unknown => None,
        }
    }

    pub fn get_morphism(&self, label: &Label) -> Option<MorphismId> {
        match label {
            Label::Name(label) => self.morphisms.get(label).copied(),
            Label::Unknown => None,
        }
    }
}
