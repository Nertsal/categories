use super::*;

impl<O> ObjectTag<O> {
    pub fn map<V, Fv: FnMut(O) -> V>(self, mut fv: Fv) -> ObjectTag<V> {
        match self {
            Self::Initial => ObjectTag::Initial,
            Self::Terminal => ObjectTag::Terminal,
            Self::Product(a, b) => ObjectTag::Product(fv(a), fv(b)),
        }
    }

    pub fn map_borrowed<V, Fv: FnMut(&O) -> V>(&self, mut fv: Fv) -> ObjectTag<V> {
        match self {
            Self::Initial => ObjectTag::Initial,
            Self::Terminal => ObjectTag::Terminal,
            Self::Product(a, b) => ObjectTag::Product(fv(a), fv(b)),
        }
    }
}

impl<O, M> MorphismTag<O, M> {
    pub fn map<V, E, Fv: FnMut(O) -> V, Fe: FnMut(M) -> E>(
        self,
        mut fv: Fv,
        mut fe: Fe,
    ) -> MorphismTag<V, E> {
        match self {
            Self::Unique => MorphismTag::Unique,
            Self::ProductP1 => MorphismTag::ProductP1,
            Self::ProductP2 => MorphismTag::ProductP2,
            Self::Identity(v) => MorphismTag::Identity(fv(v)),
            Self::Composition { first, second } => MorphismTag::Composition {
                first: fe(first),
                second: fe(second),
            },
            Self::Isomorphism(f, g) => MorphismTag::Isomorphism(fe(f), fe(g)),
        }
    }

    pub fn map_borrowed<V, E, Fv: FnMut(&O) -> V, Fe: FnMut(&M) -> E>(
        &self,
        mut fv: Fv,
        mut fe: Fe,
    ) -> MorphismTag<V, E> {
        match self {
            Self::Unique => MorphismTag::Unique,
            Self::ProductP1 => MorphismTag::ProductP1,
            Self::ProductP2 => MorphismTag::ProductP2,
            Self::Identity(v) => MorphismTag::Identity(fv(v)),
            Self::Composition { first, second } => MorphismTag::Composition {
                first: fe(first),
                second: fe(second),
            },
            Self::Isomorphism(f, g) => MorphismTag::Isomorphism(fe(f), fe(g)),
        }
    }
}
