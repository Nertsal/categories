pub enum ObjectTag<V> {
    Product(V, V),
}

pub enum MorphismTag<V, E> {
    Identity(V),
    Composition { first: E, second: E },
    Unique,
    Isomorphism(E, E),
}
