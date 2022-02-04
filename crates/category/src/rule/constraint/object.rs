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

fn object_matches<O, L: Label>(
    tags: &[ObjectTag<L>],
    object: &Object<O>,
    bindings: &Bindings<L>,
) -> Option<Bindings<L>> {
    let mut new_bindings = Bindings::new();

    for tag_check in tags.iter().map(|constraint| {
        object
            .tags
            .iter()
            .find_map(|tag| tag_matches(constraint, tag, bindings))
    }) {
        let binds = match tag_check {
            Some(binds) => binds,
            None => return None,
        };
        new_bindings.extend(binds);
    }

    Some(new_bindings)
}

fn tag_matches<L: Label>(
    constraint: &ObjectTag<L>,
    tag: &ObjectTag,
    bindings: &Bindings<L>,
) -> Option<Bindings<L>> {
    match (constraint, tag) {
        (ObjectTag::Initial, ObjectTag::Initial) => Some(Bindings::new()),
        (ObjectTag::Initial, _) => None,
        (ObjectTag::Terminal, ObjectTag::Terminal) => Some(Bindings::new()),
        (ObjectTag::Terminal, _) => None,
        (
            ObjectTag::Product(constraint_a, constraint_b),
            &ObjectTag::Product(object_a, object_b),
        ) => constraint_unordered(
            vec![constraint_a, constraint_b]
                .into_iter()
                .map(|label| (label.clone(), bindings.get_object(label))),
            vec![object_a, object_b],
        )
        .map(|binds| Bindings::from_objects(binds)),
        (ObjectTag::Product(_, _), _) => None,
    }
}
