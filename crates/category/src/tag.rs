use std::fmt::Display;

use super::*;

impl<L: Label + Display> ObjectTag<L> {
    pub fn infer_name(&self) -> Option<String> {
        match &self.map_borrowed(|label| format!("{label}")) {
            ObjectTag::Product(a, b) => label_operation(a, b, "x"),
            _ => None,
        }
    }
}

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

impl<L: Label + Display> MorphismTag<L, L> {
    pub fn infer_name(&self) -> Option<String> {
        match &self.map_borrowed(|label| format!("{label}"), |label| format!("{label}")) {
            MorphismTag::Identity(_) => Some(format!("id")),
            MorphismTag::Composition { first, second } => label_operation(second, first, " o "), // TODO: use ∘
            _ => None,
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
            Self::Identity(v) => MorphismTag::Identity(fv(v)),
            Self::Composition { first, second } => MorphismTag::Composition {
                first: fe(first),
                second: fe(second),
            },
            Self::Isomorphism(f, g) => MorphismTag::Isomorphism(fe(f), fe(g)),
        }
    }
}

fn label_operation(label_a: &str, label_b: &str, operation: &str) -> Option<String> {
    if label_a.is_empty() || label_b.is_empty() {
        return None;
    }

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

    Some(format!("{}{}{}", first, operation, second))
}
