use super::*;

pub fn constraint_object<'a, O, M, L: Label>(
    label: &'a L,
    tags: &'a [ObjectTag<L>],
    bindings: &'a Bindings<L>,
    category: &'a Category<O, M>,
) -> Box<dyn Iterator<Item = Bindings<L>> + 'a> {
    match bindings.get_object(label) {
        Some(object) => {
            let object = category
                .objects
                .get(&object)
                .expect("Invalid bindings: unknown object id"); // TODO: return an error
            object_matches(tags, object, bindings).map_or(Box::new(vec![].into_iter()), |binds| {
                Box::new(std::iter::once(binds))
            })
        }
        None => Box::new(category.objects.iter().filter_map(|(&id, object)| {
            object_matches(tags, object, bindings).map(|mut binds| {
                binds.bind_object(label.clone(), id);
                binds
            })
        })),
    }
}

fn object_matches<T, L: Label>(
    tags: &[ObjectTag<L>],
    object: &Object<T>,
    bindings: &Bindings<L>,
) -> Option<Bindings<L>> {
    todo!()
}
