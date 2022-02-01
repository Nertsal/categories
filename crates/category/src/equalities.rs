use std::collections::HashSet;

use super::*;

pub struct Equalities {
    equalities: HashSet<(MorphismId, MorphismId)>,
    commutes: HashSet<Commute>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
/// f . g = h
struct Commute {
    f: MorphismId,
    g: MorphismId,
    h: MorphismId,
}

impl Equalities {
    pub fn new() -> Self {
        Self {
            equalities: Default::default(),
            commutes: Default::default(),
        }
    }

    pub fn new_equality(&mut self, f: MorphismId, g: MorphismId) {
        self.equalities.insert((f, g));
    }

    pub fn new_commute(&mut self, f: MorphismId, g: MorphismId, h: MorphismId) {
        self.commutes.insert(Commute { f, g, h });
    }
}
