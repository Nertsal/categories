use super::*;

pub fn constraint_object<'a>(
    label: &'a Label,
    tag: &'a Option<ObjectTag>,
    bindings: &'a Bindings,
    graph: &'a Graph,
) -> impl Iterator<Item = Bindings> + 'a {
    assert!(
        bindings.get_object(label).is_none(),
        "Objects must have unique names!"
    );

    graph.graph.vertices.iter().filter_map(|(&id, vertex)| {
        let mut binds = Bindings::new();
        if object_match(tag, vertex, bindings, &mut binds) {
            binds.bind_object(label.clone(), id);
            Some(binds)
        } else {
            None
        }
    })
}

fn object_match(
    constraint_tag: &Option<ObjectTag>,
    vertex: &Vertex,
    bindings: &Bindings,
    binds: &mut Bindings,
) -> bool {
    match (constraint_tag, &vertex.vertex.tag) {
        (None, _) => true,
        (_, None) => false,
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
    }
}

fn check<T: Eq>(value: T, constraint: Option<T>) -> bool {
    match constraint {
        None => true,
        Some(constraint) => value == constraint,
    }
}
