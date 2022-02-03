use super::*;

pub struct Category {
    pub objects: Objects,
    pub morphisms: Morphisms,
    pub equalities: Equalities,
}

impl Category {
    pub fn new() -> Self {
        Self {
            objects: Objects::new(),
            morphisms: Morphisms::new(),
            equalities: Equalities::new(),
        }
    }

    pub fn new_object(&mut self, object: Object) -> ObjectId {
        self.objects.new_object(object)
    }

    /// Adds a new morphism to the graph.
    /// Returns None if the graph does not contain any of the vertices.
    pub fn new_morphism(&mut self, morphism: Morphism) -> Option<MorphismId> {
        let end_points = morphism.connection.end_points();
        if !self.objects.contains(end_points[0]) || !self.objects.contains(end_points[1]) {
            return None;
        }
        Some(self.morphisms.new_morphism(morphism))
    }

    pub fn insert_object(
        &mut self,
        object: Object,
        object_id: ObjectId,
    ) -> Result<Option<Object>, ()> {
        self.objects.insert(object, object_id)
    }

    pub fn insert_morphism(
        &mut self,
        morphism: Morphism,
        morphism_id: MorphismId,
    ) -> Result<Option<Morphism>, ()> {
        self.morphisms.insert(morphism, morphism_id)
    }

    /// Removes the object and connected morphisms from the graph.
    pub fn remove_object(
        &mut self,
        object_id: ObjectId,
    ) -> Option<(Object, Vec<(MorphismId, Morphism)>)> {
        self.objects.remove(&object_id).map(|object| {
            let removes: Vec<_> = self
                .morphisms
                .iter()
                .filter(|(_, morphism)| morphism.connection.is_object_connected(object_id))
                .map(|(&id, _)| id)
                .collect();
            let mut morphisms = Vec::new();
            for remove in removes {
                morphisms.push((remove, self.morphisms.remove(&remove).unwrap()));
            }
            (object, morphisms)
        })
    }

    /// Removes the morphism from the graph.
    pub fn remove_morphism(&mut self, morphism_id: MorphismId) -> Option<Morphism> {
        self.morphisms.remove(&morphism_id)
    }

    pub fn neighbours<'a>(&'a self, object: ObjectId) -> impl Iterator<Item = ObjectId> + 'a {
        self.morphisms.iter().filter_map(move |(_, morphism)| {
            let endpoints = morphism.connection.end_points();
            if *endpoints[0] == object {
                Some(*endpoints[1])
            } else if *endpoints[1] == object {
                Some(*endpoints[0])
            } else {
                None
            }
        })
    }
}
