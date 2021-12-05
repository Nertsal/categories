#[derive(Debug, Clone, PartialEq)]
pub enum ObjectTag<O> {
    Product(O, O),
}

impl<O> ObjectTag<O> {
    pub fn map<V, Fv: Fn(O) -> V>(self, fv: Fv) -> ObjectTag<V> {
        match self {
            Self::Product(a, b) => ObjectTag::Product(fv(a), fv(b)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MorphismTag<O, M> {
    Identity(O),
    Composition { first: M, second: M },
    Unique,
    Isomorphism(M, M),
}

impl<O, M> MorphismTag<O, M> {
    pub fn map<V, E, Fv: Fn(O) -> V, Fe: Fn(M) -> E>(self, fv: Fv, fe: Fe) -> MorphismTag<V, E> {
        match self {
            Self::Identity(v) => MorphismTag::Identity(fv(v)),
            Self::Composition { first, second } => MorphismTag::Composition {
                first: fe(first),
                second: fe(second),
            },
            Self::Unique => MorphismTag::Unique,
            Self::Isomorphism(f, g) => MorphismTag::Isomorphism(fe(f), fe(g)),
        }
    }

    pub fn map_borrowed<V, E, Fv: Fn(&O) -> V, Fe: Fn(&M) -> E>(
        &self,
        fv: Fv,
        fe: Fe,
    ) -> MorphismTag<V, E> {
        match self {
            Self::Identity(v) => MorphismTag::Identity(fv(v)),
            Self::Composition { first, second } => MorphismTag::Composition {
                first: fe(first),
                second: fe(second),
            },
            Self::Unique => MorphismTag::Unique,
            Self::Isomorphism(f, g) => MorphismTag::Isomorphism(fe(f), fe(g)),
        }
    }
}
