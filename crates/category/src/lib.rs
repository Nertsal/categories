#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Id(u64);

struct IdGenerator {
    next_id: Id,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self { next_id: Id(0) }
    }

    pub fn next(&mut self) -> Id {
        let mut next = Id(self.next_id.0 + 1);
        std::mem::swap(&mut self.next_id, &mut next);
        next
    }
}

pub struct Morphism {
    id: Id,
}

pub struct Object {
    id: Id,
}

pub struct Category {
    objects: HashMap<Id, Object>,
    morphisms: HashMap<Id, Morphism>,
}
