use super::*;

pub fn constraint_equality(
    morphism_f: &Label,
    morphism_g: &Label,
    bindings: &Bindings,
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
                    let mut binds = Bindings::new();
                    binds.bind_morphism(morphism_g.clone(), g);
                    Some(binds)
                } else if g == morphism_f {
                    let mut binds = Bindings::new();
                    binds.bind_morphism(morphism_g.clone(), f);
                    Some(binds)
                } else {
                    None
                }
            })
            .collect(),
        (None, Some(morphism_g)) => equalities
            .iter()
            .filter_map(|&(f, g)| {
                if f == morphism_g {
                    let mut binds = Bindings::new();
                    binds.bind_morphism(morphism_f.clone(), g);
                    Some(binds)
                } else if g == morphism_g {
                    let mut binds = Bindings::new();
                    binds.bind_morphism(morphism_f.clone(), f);
                    Some(binds)
                } else {
                    None
                }
            })
            .collect(),
        (None, None) => equalities
            .iter()
            .map(|&(f, g)| {
                let mut binds = Bindings::new();
                binds.bind_morphism(morphism_f.clone(), f);
                binds.bind_morphism(morphism_g.clone(), g);
                binds
            })
            .collect(),
    }
}
