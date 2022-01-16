use super::*;

pub fn constraint_equality(
    morphism_f: &Label,
    morphism_g: &Label,
    bindings: &Bindings,
    graph: &Graph,
    equalities: &GraphEqualities,
) -> Vec<Bindings> {
    match (
        bindings.get_morphism(morphism_f),
        bindings.get_morphism(morphism_g),
    ) {
        (Some(morphism_f), Some(morphism_g)) => {
            if morphism_f == morphism_g
                || equalities.contains(&(morphism_f, morphism_g))
                || equalities.contains(&(morphism_g, morphism_f))
            {
                vec![Bindings::new()]
            } else {
                vec![]
            }
        }
        (Some(morphism_f), None) => equalities
            .iter()
            .filter_map(|&(f, g)| {
                if f == morphism_f {
                    Some(Bindings::single_morphism(morphism_g.clone(), g))
                } else if g == morphism_f {
                    Some(Bindings::single_morphism(morphism_g.clone(), f))
                } else {
                    None
                }
            })
            .chain(std::iter::once(Bindings::single_morphism(
                morphism_g.clone(),
                morphism_f,
            )))
            .collect(),
        (None, Some(morphism_g)) => equalities
            .iter()
            .filter_map(|&(f, g)| {
                if f == morphism_g {
                    Some(Bindings::single_morphism(morphism_f.clone(), g))
                } else if g == morphism_g {
                    Some(Bindings::single_morphism(morphism_f.clone(), f))
                } else {
                    None
                }
            })
            .chain(std::iter::once(Bindings::single_morphism(
                morphism_f.clone(),
                morphism_g,
            )))
            .collect(),
        (None, None) => equalities
            .iter()
            .map(|&(f, g)| {
                let mut binds = Bindings::new();
                binds.bind_morphism(morphism_f.clone(), f);
                binds.bind_morphism(morphism_g.clone(), g);
                binds
            })
            .chain(graph.graph.edges.iter().map(|(&edge, _)| {
                let mut binds = Bindings::new();
                binds.bind_morphism(morphism_f.clone(), edge);
                binds.bind_morphism(morphism_g.clone(), edge);
                binds
            }))
            .collect(),
    }
}
