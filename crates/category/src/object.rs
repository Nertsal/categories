use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Object<T> {
    pub tags: Vec<ObjectTag>,
    pub inner: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectTag<O = ObjectId> {
    Initial,
    Terminal,
    Product(O, O),
}

pub struct Objects<O> {
    objects: HashMap<ObjectId, Object<O>>,
    next_id: ObjectId,
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
pub struct ObjectId(u64);

impl ObjectId {
    pub fn raw(&self) -> u64 {
        self.0
    }
}

impl<T> Objects<T> {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            next_id: ObjectId(0),
        }
    }

    pub(crate) fn new_object(&mut self, object: Object<T>) -> ObjectId {
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
        object: Object<T>,
        object_id: ObjectId,
    ) -> Result<Option<Object<T>>, ()> {
        if object_id.0 >= self.next_id.0 {
            return Err(());
        }

        Ok(self.objects.insert(object_id, object))
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ObjectId, &Object<T>)> {
        self.objects.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&ObjectId, &mut Object<T>)> {
        self.objects.iter_mut()
    }

    pub fn get(&self, id: &ObjectId) -> Option<&Object<T>> {
        self.objects.get(id)
    }

    pub fn get_mut(&mut self, id: &ObjectId) -> Option<&mut Object<T>> {
        self.objects.get_mut(id)
    }

    pub(crate) fn remove(&mut self, id: &ObjectId) -> Option<Object<T>> {
        self.objects.remove(id)
    }

    pub fn contains(&self, id: &ObjectId) -> bool {
        self.objects.contains_key(id)
    }
}
