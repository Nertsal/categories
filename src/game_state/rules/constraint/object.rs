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
        if tags.iter().all(|constraint| {
            vertex
                .vertex
                .tags
                .iter()
                .any(|tag| match (constraint, tag) {
                    (
                        ObjectTag::Product(constraint0, constraint1),
                        &ObjectTag::Product(object0, object1),
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
                                    binds.bind_object(constraint1.to_owned(), object1);
                                    true
                                } else if constraint0 == object1 {
                                    binds.bind_object(constraint1.to_owned(), object0);
                                    true
                                } else {
                                    false
                                }
                            }
                            (None, Some(constraint1)) => {
                                if constraint1 == object0 {
                                    binds.bind_object(constraint0.to_owned(), object1);
                                    true
                                } else if constraint1 == object1 {
                                    binds.bind_object(constraint0.to_owned(), object0);
                                    true
                                } else {
                                    false
                                }
                            }
                            (None, None) => {
                                binds.bind_object(constraint0.to_owned(), object0);
                                binds.bind_object(constraint1.to_owned(), object1);
                                true
                            }
                        }
                    }
                })
        }) {
            binds.bind_object(label.to_owned(), id);
            Some(binds)
        } else {
            None
        }
    })
}