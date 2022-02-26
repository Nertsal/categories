use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CategoryThing {
    Object { id: ObjectId },
    Morphism { id: MorphismId },
}

pub struct Category<O, M, E> {
    pub objects: Objects<O>,
    pub morphisms: Morphisms<M>,
    pub equalities: Equalities<E>,
}

impl<O, M, E> Category<O, M, E> {
    pub fn new() -> Self {
        Self {
            objects: Objects::new(),
            morphisms: Morphisms::new(),
            equalities: Equalities::new(),
        }
    }

    pub fn new_object(&mut self, object: Object<O>) -> ObjectId {
        self.objects.new_object(object)
    }

    /// Adds a new morphism to the graph.
    /// Returns None if the graph does not contain any of the vertices.
    pub fn new_morphism(&mut self, morphism: Morphism<M>) -> Option<MorphismId> {
        let end_points = morphism.connection.end_points();
        if !self.objects.contains(end_points[0]) || !self.objects.contains(end_points[1]) {
            return None;
        }
        Some(self.morphisms.new_morphism(morphism))
    }

    pub fn insert_object(
        &mut self,
        object: Object<O>,
        object_id: ObjectId,
    ) -> Result<Option<Object<O>>, ()> {
        self.objects.insert(object, object_id)
    }

    pub fn insert_morphism(
        &mut self,
        morphism: Morphism<M>,
        morphism_id: MorphismId,
    ) -> Result<Option<Morphism<M>>, ()> {
        self.morphisms.insert(morphism, morphism_id)
    }

    /// Removes the object and connected morphisms from the graph.
    pub fn remove_object(
        &mut self,
        object_id: ObjectId,
    ) -> Option<(Object<O>, Vec<(MorphismId, Morphism<M>)>)> {
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
    pub fn remove_morphism(&mut self, morphism_id: MorphismId) -> Option<Morphism<M>> {
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
