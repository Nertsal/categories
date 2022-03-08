use super::*;

pub fn constraint_object<'a, O, M, E, L: Label>(
    label: &'a L,
    tags: &'a [ObjectTag<L>],
    bindings: &'a Bindings<L>,
    category: &'a Category<O, M, E>,
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

    for tag_check in tags
        .iter()
        .map(|constraint| tag_matches(constraint, &object.tags, bindings))
    {
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
    tags: &[ObjectTag],
    bindings: &Bindings<L>,
) -> Option<Bindings<L>> {
    match constraint {
        ObjectTag::Initial => tags.iter().find_map(|tag| {
            if let ObjectTag::Initial = tag {
                Some(Bindings::new())
            } else {
                None
            }
        }),
        ObjectTag::Terminal => tags.iter().find_map(|tag| {
            if let ObjectTag::Terminal = tag {
                Some(Bindings::new())
            } else {
                None
            }
        }),
        ObjectTag::Product(constraint_a, constraint_b) => tags.iter().find_map(|tag| {
            if let &ObjectTag::Product(object_a, object_b) = tag {
                constraint_ordered(
                    vec![constraint_a, constraint_b]
                        .into_iter()
                        .map(|label| (label.clone(), bindings.get_object(label))),
                    vec![object_a, object_b],
                )
                .map(|binds| Bindings::from_objects(binds))
            } else {
                None
            }
        }),
    }
}
