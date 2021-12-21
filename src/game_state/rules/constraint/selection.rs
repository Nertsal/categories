use super::*;

pub fn selection_constraints(
    selection: &Vec<GraphObject>,
    constraints: &Constraints,
    graph: &Graph,
) -> Result<Bindings, ()> {
    let mut selection = selection.iter();
    let mut bindings = Bindings::new();

    fn bind_object(bindings: &mut Bindings, label: &str, constraint: VertexId) -> bool {
        match bindings.get_object(label) {
            Some(object) => object == constraint,
            None => {
                bindings.bind_object(label.to_owned(), constraint);
                true
            }
        }
    }

    for constraint in constraints {
        match constraint {
            Constraint::RuleObject(label, object) => match object {
                RuleObject::Vertex { .. } => match selection.next() {
                    Some(GraphObject::Vertex { id }) => {
                        if bindings.bind_object(label.to_owned(), *id).is_some() {
                            return Err(());
                        }
                    }
                    _ => return Err(()),
                },
                RuleObject::Edge { constraint } => match selection.next() {
                    Some(GraphObject::Edge { id }) => {
                        let edge = graph.graph.edges.get(id).unwrap();
                        if !bind_object(&mut bindings, &constraint.from, edge.edge.from)
                            || !bind_object(&mut bindings, &constraint.to, edge.edge.to)
                        {
                            return Err(());
                        }

                        bindings.bind_morphism(label.to_owned(), *id);
                    }
                    _ => return Err(()),
                },
            },
            Constraint::MorphismEq(_, _) => todo!(),
        }
    }

    Ok(bindings)
}