use super::*;

pub fn constraint_morphism<'a, O, M, L: Label>(
    label: &'a L,
    connection: &'a MorphismConnection<L>,
    tags: &'a [MorphismTag<L>],
    bindings: &'a Bindings<L>,
    category: &'a Category<O, M>,
) -> Box<dyn Iterator<Item = Bindings<L>> + 'a> {
    match bindings.get_morphism(label) {
        Some(morphism) => {
            let morphism = category
                .morphisms
                .get(&morphism)
                .expect("Invalid bindings: unknown object id"); // TODO: return an error
            morphism_matches(tags, morphism, bindings)
                .map_or(Box::new(vec![].into_iter()), |binds| {
                    Box::new(std::iter::once(binds))
                })
        }
        None => Box::new(category.morphisms.iter().filter_map(|(&id, morphism)| {
            morphism_matches(tags, morphism, bindings).map(|mut binds| {
                binds.bind_morphism(label.clone(), id);
                binds
            })
        })),
    }
}

fn morphism_matches<T, L: Label>(
    tags: &[MorphismTag<L>],
    morphism: &Morphism<T>,
    bindings: &Bindings<L>,
) -> Option<Bindings<L>> {
    todo!()
}
