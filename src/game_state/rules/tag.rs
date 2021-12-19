use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectTag<O = Label> {
    Product(O, O),
}

impl<O: AsRef<str>> ObjectTag<O> {
    pub fn infer_name(&self) -> Option<Label> {
        match self {
            ObjectTag::Product(a, b) => {
                if a.as_ref().is_empty() || b.as_ref().is_empty() {
                    return None;
                }
                Some(label_operation(a.as_ref(), b.as_ref(), "x"))
            }
        }
    }
}

impl<O> ObjectTag<O> {
    pub fn map<V, Fv: Fn(O) -> V>(self, fv: Fv) -> ObjectTag<V> {
        match self {
            Self::Product(a, b) => ObjectTag::Product(fv(a), fv(b)),
        }
    }

    pub fn map_borrowed<V, Fv: Fn(&O) -> V>(&self, fv: Fv) -> ObjectTag<V> {
        match self {
            Self::Product(a, b) => ObjectTag::Product(fv(a), fv(b)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MorphismTag<O = Label, M = Label> {
    Identity(O),
    Composition { first: M, second: M },
    Unique,
    Isomorphism(M, M),
}

impl<O: AsRef<str>, M: AsRef<str>> MorphismTag<O, M> {
    pub fn infer_name(&self) -> Option<Label> {
        match self {
            MorphismTag::Identity(_) => Some(format!("id")),
            MorphismTag::Composition { first, second } => {
                if first.as_ref().is_empty() || second.as_ref().is_empty() {
                    return None;
                }
                Some(label_operation(first.as_ref(), second.as_ref(), "."))
            }
            MorphismTag::Unique => None,
            MorphismTag::Isomorphism(_, _) => None,
        }
    }
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

fn label_operation(label_a: &str, label_b: &str, operation: &str) -> String {
    let first = if label_a.contains(operation) {
        format!("({})", label_a)
    } else {
        format!("{}", label_a)
    };
    let second = if label_b.contains(operation) {
        format!("({})", label_b)
    } else {
        format!("{}", label_b)
    };
    format!("{}{}{}", first, operation, second)
}
