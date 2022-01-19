use std::collections::HashMap;

pub struct Objects<T> {
    objects: HashMap<ObjectId, T>,
    next_id: ObjectId,
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
pub struct ObjectId(u64);

impl<T> Objects<T> {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            next_id: ObjectId(0),
        }
    }

    pub(crate) fn new_object(&mut self, object: T) -> ObjectId {
        let id = self.next_id;
        self.next_id.0 += 1;
        assert!(
            self.objects.insert(id, object).is_none(),
            "Failed to generate new object"
        );
        id
    }

    pub(crate) fn insert(&mut self, object: T, object_id: ObjectId) -> Result<Option<T>, ()> {
        if object_id.0 >= self.next_id.0 {
            return Err(());
        }

        Ok(self.objects.insert(object_id, object))
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ObjectId, &T)> {
        self.objects.iter()
    }

    pub fn get(&self, id: &ObjectId) -> Option<&T> {
        self.objects.get(id)
    }

    pub fn get_mut(&mut self, id: &ObjectId) -> Option<&mut T> {
        self.objects.get_mut(id)
    }

    pub(crate) fn remove(&mut self, id: &ObjectId) -> Option<T> {
        self.objects.remove(id)
    }

    pub fn contains(&self, id: &ObjectId) -> bool {
        self.objects.contains_key(id)
    }
}
