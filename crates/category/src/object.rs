use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Object {
    pub tags: Vec<ObjectTag>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectTag<O = ObjectId> {
    Initial,
    Terminal,
    Product(O, O),
}

pub struct Objects {
    objects: HashMap<ObjectId, Object>,
    next_id: ObjectId,
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
pub struct ObjectId(u64);

impl Objects {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            next_id: ObjectId(0),
        }
    }

    pub(crate) fn new_object(&mut self, object: Object) -> ObjectId {
        let id = self.next_id;
        self.next_id.0 += 1;
        assert!(
            self.objects.insert(id, object).is_none(),
            "Failed to generate new object"
        );
        id
    }

    pub(crate) fn insert(
        &mut self,
        object: Object,
        object_id: ObjectId,
    ) -> Result<Option<Object>, ()> {
        if object_id.0 >= self.next_id.0 {
            return Err(());
        }

        Ok(self.objects.insert(object_id, object))
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ObjectId, &Object)> {
        self.objects.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&ObjectId, &mut Object)> {
        self.objects.iter_mut()
    }

    pub fn get(&self, id: &ObjectId) -> Option<&Object> {
        self.objects.get(id)
    }

    pub fn get_mut(&mut self, id: &ObjectId) -> Option<&mut Object> {
        self.objects.get_mut(id)
    }

    pub(crate) fn remove(&mut self, id: &ObjectId) -> Option<Object> {
        self.objects.remove(id)
    }

    pub fn contains(&self, id: &ObjectId) -> bool {
        self.objects.contains_key(id)
    }
}
