use super::*;

pub fn constraint_object(
    label: &Label,
    tag: &Option<ObjectTag>,
    bindings: &Bindings,
    category: &Category,
) -> Vec<Bindings> {
    match bindings.get_object(label) {
        Some(object_id) => {
            let mut binds = Bindings::new();
            let object = category.objects.get(&object_id).unwrap();
            if object_match(tag, object, bindings, &mut binds) {
                vec![binds]
            } else {
                vec![]
            }
        }
        None => category
            .objects
            .iter()
            .filter_map(|(&id, object)| {
                let mut binds = Bindings::new();
                if object_match(tag, object, bindings, &mut binds) {
                    binds.bind_object(label.clone(), id);
                    Some(binds)
                } else {
                    None
                }
            })
            .collect(),
    }
}

fn object_match(
    constraint_tag: &Option<ObjectTag>,
    object: &Object,
    bindings: &Bindings,
    binds: &mut Bindings,
) -> bool {
    match (constraint_tag, &object.tag) {
        (None, _) => true,
        (_, None) => false,
        (Some(ObjectTag::Initial), Some(ObjectTag::Initial)) => true,
        (Some(ObjectTag::Terminal), Some(ObjectTag::Terminal)) => true,
        (
            Some(ObjectTag::Product(constraint0, constraint1)),
            &Some(ObjectTag::Product(object0, object1)),
        ) => {
            let mut bind = |label, id| match (label, id) {
                (Some(label), Some(id)) => {
                    binds.bind_object(label, id);
                }
                _ => (),
            };

            match (
                constraint0
                    .as_ref()
                    .and_then(|label| bindings.get_object(label)),
                constraint1
                    .as_ref()
                    .and_then(|label| bindings.get_object(label)),
            ) {
                (Some(constraint0), Some(constraint1)) => {
                    check(constraint0, object0) && check(constraint1, object1)
                        || check(constraint0, object1) && check(constraint1, object0)
                }
                (Some(constraint0), None) => {
                    if check(constraint0, object0) {
                        bind(constraint1.clone(), object1);
                        true
                    } else if check(constraint0, object1) {
                        bind(constraint1.clone(), object0);
                        true
                    } else {
                        false
                    }
                }
                (None, Some(constraint1)) => {
                    if check(constraint1, object0) {
                        bind(constraint0.clone(), object1);
                        true
                    } else if check(constraint1, object1) {
                        bind(constraint0.clone(), object0);
                        true
                    } else {
                        false
                    }
                }
                (None, None) => {
                    bind(constraint0.clone(), object0);
                    bind(constraint1.clone(), object1);
                    true
                }
            }
        }
        _ => false,
    }
}

fn check<T: Eq>(value: T, constraint: Option<T>) -> bool {
    match constraint {
        None => true,
        Some(constraint) => value == constraint,
    }
}
