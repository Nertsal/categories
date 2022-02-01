use std::collections::HashMap;

use super::*;

#[derive(Debug, Clone)]
pub struct Bindings<L: Label> {
    objects: HashMap<L, ObjectId>,
    morphisms: HashMap<L, MorphismId>,
}

impl<L: Label> Bindings<L> {
    pub fn new() -> Self {
        Self {
            objects: Default::default(),
            morphisms: Default::default(),
        }
    }

    pub fn single_morphism(label: L, id: MorphismId) -> Self {
        let mut binds = Self::new();
        binds.bind_morphism(label, id);
        binds
    }

    pub fn extend(&mut self, bindings: Self) {
        self.objects.extend(bindings.objects.into_iter());
        self.morphisms.extend(bindings.morphisms.into_iter());
    }

    pub fn bind_object(&mut self, label: L, id: ObjectId) -> Option<ObjectId> {
        self.objects.insert(label, id)
    }

    pub fn bind_morphism(&mut self, label: L, id: MorphismId) -> Option<MorphismId> {
        self.morphisms.insert(label, id)
    }

    pub fn get_object(&self, label: &L) -> Option<ObjectId> {
        self.objects.get(label).copied()
    }

    pub fn get_morphism(&self, label: &L) -> Option<MorphismId> {
        self.morphisms.get(label).copied()
    }
}
