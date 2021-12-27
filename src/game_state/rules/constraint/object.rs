use super::*;

pub fn constraint_object<'a>(
    label: &'a Label,
    tags: &'a Vec<ObjectTag>,
    bindings: &'a Bindings,
    graph: &'a Graph,
) -> impl Iterator<Item = Bindings> + 'a {
    assert!(
        bindings.get_object(label).is_none(),
        "Objects must have unique names!"
    );

    graph.graph.vertices.iter().filter_map(|(&id, vertex)| {
        let mut binds = Bindings::new();
        if object_match(tags, vertex, bindings, &mut binds) {
            binds.bind_object(label.clone(), id);
            Some(binds)
        } else {
            None
        }
    })
}

fn object_match(
    tags: &Vec<ObjectTag>,
    vertex: &Vertex,
    bindings: &Bindings,
    binds: &mut Bindings,
) -> bool {
    tags.iter().all(|constraint| {
        vertex
            .vertex
            .tags
            .iter()
            .any(|tag| match (constraint, tag) {
                (
                    ObjectTag::Product(Some(constraint0), Some(constraint1)),
                    &ObjectTag::Product(Some(object0), Some(object1)),
                ) => {
                    match (
                        bindings.get_object(constraint0),
                        bindings.get_object(constraint1),
                    ) {
                        (Some(constraint0), Some(constraint1)) => {
                            constraint0 == object0 && constraint1 == object1
                                || constraint0 == object1 && constraint1 == object0
                        }
                        (Some(constraint0), None) => {
                            if constraint0 == object0 {
                                binds.bind_object(constraint1.clone(), object1);
                                true
                            } else if constraint0 == object1 {
                                binds.bind_object(constraint1.clone(), object0);
                                true
                            } else {
                                false
                            }
                        }
                        (None, Some(constraint1)) => {
                            if constraint1 == object0 {
                                binds.bind_object(constraint0.clone(), object1);
                                true
                            } else if constraint1 == object1 {
                                binds.bind_object(constraint0.clone(), object0);
                                true
                            } else {
                                false
                            }
                        }
                        (None, None) => {
                            binds.bind_object(constraint0.clone(), object0);
                            binds.bind_object(constraint1.clone(), object1);
                            true
                        }
                    }
                }
                _ => false,
            })
    })
}
