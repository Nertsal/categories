use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectTag<O = Option<Label>> {
    Product(O, O),
}

impl ObjectTag<Option<&Label>> {
    pub fn infer_name(&self) -> Option<String> {
        match self {
            ObjectTag::Product(Some(Label::Name(a)), Some(Label::Name(b))) => tag_name(a, b, "x"),
            _ => None,
        }
    }
}

fn tag_name(label_a: &str, label_b: &str, operation: &str) -> Option<String> {
    if label_a.is_empty() || label_b.is_empty() {
        None
    } else {
        Some(label_operation(label_a, label_b, operation))
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
pub enum MorphismTag<O = Option<Label>, M = Option<Label>> {
    Identity(O),
    Composition { first: M, second: M },
    Unique,
    Isomorphism(M, M),
}

impl MorphismTag<Option<&Label>, Option<&Label>> {
    pub fn infer_name(&self) -> Option<String> {
        match self {
            MorphismTag::Identity(_) => Some(format!("id")),
            MorphismTag::Composition {
                first: Some(Label::Name(first)),
                second: Some(Label::Name(second)),
            } => tag_name(first, second, "."),
            _ => None,
        }
    }
}

impl<O, M> MorphismTag<O, M> {
    pub fn objects(&self) -> Vec<&O> {
        match self {
            MorphismTag::Identity(a) => vec![a],
            MorphismTag::Composition { .. }
            | MorphismTag::Unique
            | MorphismTag::Isomorphism(_, _) => vec![],
        }
    }

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
