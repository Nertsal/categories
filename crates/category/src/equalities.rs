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

    pub fn check_equality(&self, f: MorphismId, g: MorphismId) -> bool {
        self.equalities.contains(&(f, g)) || self.equalities.contains(&(g, f))
    }

    pub fn check_commutativity(&self, f: MorphismId, g: MorphismId, h: MorphismId) -> bool {
        self.commutes.contains(&Commute { f, g, h })
    }

    pub fn remove_equality(&mut self, f: MorphismId, g: MorphismId) -> bool {
        self.equalities.remove(&(f, g)) || self.equalities.remove(&(g, f))
    }

    pub fn remove_commute(&mut self, f: MorphismId, g: MorphismId, h: MorphismId) -> bool {
        self.commutes.remove(&Commute { f, g, h })
    }

    pub fn all_equalities<'a>(&'a self) -> impl Iterator<Item = (MorphismId, MorphismId)> + 'a {
        self.equalities.iter().copied()
    }

    pub fn get_equalities<'a>(
        &'a self,
        morphism: MorphismId,
    ) -> impl Iterator<Item = MorphismId> + 'a {
        self.equalities.iter().filter_map(move |&(f, g)| {
            if f == morphism {
                Some(g)
            } else if g == morphism {
                Some(f)
            } else {
                None
            }
        })
    }
}
