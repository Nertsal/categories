use super::*;

#[derive(Debug, Clone)]
pub enum RuleConstructionError {}

impl<L: Label> Rule<L> {
    pub fn new(statement: RuleStatement<L>) -> Result<Self, RuleConstructionError> {
        // TODO: check that the statement is valid
        Ok(Self { statement })
    }
}

impl<O, M> Category<O, M> {
    pub fn from_rule<L: Label>(
        rule: &Rule<L>,
        object_constructor: impl Fn(&L, &Vec<ObjectTag<L>>) -> O,
        morphism_constructor: impl Fn(&L, &Vec<MorphismTag<L, L>>) -> M,
    ) -> (Self, Vec<(L, CategoryThing)>) {
        let mut statement = rule.get_statement().iter();

        let mut category = Category::new();
        let mut bindings = Bindings::new();

        let input = statement
            .next()
            .map(|construction| match construction {
                RuleConstruction::Forall(constraints) => add_constraints(
                    constraints,
                    &mut bindings,
                    &mut category,
                    object_constructor,
                    morphism_constructor,
                ),
                RuleConstruction::Exists(constraints) => add_constraints(
                    constraints,
                    &mut bindings,
                    &mut category,
                    object_constructor,
                    morphism_constructor,
                ),
            })
            .unwrap_or_default();

        (category, input)
    }
}

fn add_constraints<'a, O, M, L: 'a + Label>(
    constraints: impl IntoIterator<Item = &'a Constraint<L>>,
    bindings: &mut Bindings<&'a L>,
    category: &mut Category<O, M>,
    object_constructor: impl Fn(&L, &Vec<ObjectTag<L>>) -> O,
    morphism_constructor: impl Fn(&L, &Vec<MorphismTag<L, L>>) -> M,
) -> Vec<(L, CategoryThing)> {
    let get_object = |label, bindings: &mut Bindings<&L>, category: &mut Category<O, M>| {
        bindings.get_object(&label).unwrap_or_else(|| {
            category.new_object(Object {
                tags: vec![],
                inner: object_constructor(label, &vec![]),
            })
        })
    };

    constraints
        .into_iter()
        .filter_map(|constraint| match constraint {
            Constraint::Object { label, tags } => {
                let inner = object_constructor(label, tags);
                let tags = tags
                    .iter()
                    .map(|tag| tag.map_borrowed(|label| bindings.get_object(&label).unwrap())) // TODO: better error handling
                    .collect();
                let id = category.new_object(Object { tags, inner });
                bindings.bind_object(label, id);
                Some((label.clone(), CategoryThing::Object { id }))
            }
            Constraint::Morphism {
                label,
                connection,
                tags,
            } => {
                let connection = match connection {
                    MorphismConnection::Regular { from, to } => MorphismConnection::Regular {
                        from: get_object(from, bindings, category),
                        to: get_object(to, bindings, category),
                    },
                    MorphismConnection::Isomorphism(f, g) => MorphismConnection::Isomorphism(
                        get_object(f, bindings, category),
                        get_object(g, bindings, category),
                    ),
                };
                let id = category
                    .new_morphism({
                        let inner = morphism_constructor(label, tags);
                        let tags = tags
                            .iter()
                            .map(|tag| {
                                tag.map_borrowed(
                                    |label| bindings.get_object(&label).unwrap(),
                                    |label| bindings.get_morphism(&label).unwrap(),
                                )
                            }) // TODO: better error handling
                            .collect();
                        Morphism {
                            connection,
                            tags,
                            inner,
                        }
                    })
                    .expect("objects exist, because they have been binded");
                bindings.bind_morphism(label, id);
                Some((label.clone(), CategoryThing::Morphism { id }))
            }
            Constraint::Equality(_, _) => None, // TODO: allow equality input
            Constraint::Commute { .. } => None, // TODO: allow commutativity input
        })
        .collect()
}
