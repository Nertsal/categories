use std::collections::HashMap;

use super::*;

#[derive(Debug, Clone)]
pub enum RuleConstructionError {}

impl<L: Label> Rule<L> {
    pub fn new(statement: RuleStatement<L>) -> Result<Self, RuleConstructionError> {
        // TODO: check that the statement is valid
        Ok(Self { statement })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RulePart {
    Input,
    Forall,
    Exists,
    Output,
}

impl<O, M> Category<O, M> {
    pub fn from_rule<L: Label>(
        rule: &Rule<L>,
        object_constructor: impl Fn(RulePart, &L, &Vec<ObjectTag<L>>) -> O,
        morphism_constructor: impl Fn(RulePart, &L, &Vec<MorphismTag<L, L>>) -> M,
    ) -> (Self, Vec<(L, CategoryThing)>) {
        let statement = rule.get_statement();
        let statement_len = statement.len();
        let mut statement = statement.iter();

        let mut category = Category::new();
        let mut bindings = Bindings::new();

        let input = statement
            .next()
            .map(|construction| match construction {
                RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
                    add_constraints(
                        RulePart::Input,
                        constraints,
                        &mut bindings,
                        &mut category,
                        &object_constructor,
                        &morphism_constructor,
                    )
                }
            })
            .unwrap_or_default();

        for _ in 1..statement_len - 1 {
            let construction = statement
                .next()
                .expect("statement_len is the number of entries");
            let rule_part = match construction {
                RuleConstruction::Forall(_) => RulePart::Forall,
                RuleConstruction::Exists(_) => RulePart::Exists,
            };
            match construction {
                RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
                    add_constraints(
                        rule_part,
                        constraints,
                        &mut bindings,
                        &mut category,
                        &object_constructor,
                        &morphism_constructor,
                    );
                }
            }
        }

        if let Some(construction) = statement.next() {
            match construction {
                RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
                    add_constraints(
                        RulePart::Output,
                        constraints,
                        &mut bindings,
                        &mut category,
                        &object_constructor,
                        &morphism_constructor,
                    );
                }
            }
        }

        (category, input)
    }
}

fn add_constraints<'a, O, M, L: 'a + Label>(
    rule_part: RulePart,
    constraints: impl IntoIterator<Item = &'a Constraint<L>>,
    bindings: &mut Bindings<L>,
    category: &mut Category<O, M>,
    object_constructor: impl Fn(RulePart, &L, &Vec<ObjectTag<L>>) -> O,
    morphism_constructor: impl Fn(RulePart, &L, &Vec<MorphismTag<L, L>>) -> M,
) -> Vec<(L, CategoryThing)> {
    let get_object =
        |label: &L, objects: &mut HashMap<L, ObjectId>, category: &mut Category<O, M>| {
            *objects.entry(label.clone()).or_insert_with(|| {
                category.new_object(Object {
                    tags: vec![],
                    inner: object_constructor(rule_part, label, &vec![]),
                })
            })
        };

    constraints
        .into_iter()
        .filter_map(|constraint| match constraint {
            Constraint::Object { label, tags } => {
                let inner = object_constructor(rule_part, label, tags);
                let tags = tags
                    .iter()
                    .map(|tag| tag.map_borrowed(|label| bindings.get_object(&label).unwrap())) // TODO: better error handling
                    .collect();
                let id = category.new_object(Object { tags, inner });
                bindings.bind_object(label.clone(), id);
                Some((label.clone(), CategoryThing::Object { id }))
            }
            Constraint::Morphism {
                label,
                connection,
                tags,
            } => {
                let connection = match connection {
                    MorphismConnection::Regular { from, to } => MorphismConnection::Regular {
                        from: get_object(from, &mut bindings.objects, category),
                        to: get_object(to, &mut bindings.objects, category),
                    },
                    MorphismConnection::Isomorphism(f, g) => MorphismConnection::Isomorphism(
                        get_object(f, &mut bindings.objects, category),
                        get_object(g, &mut bindings.objects, category),
                    ),
                };
                let inner = morphism_constructor(rule_part, label, tags);

                let tags = tags
                    .iter()
                    .map(|tag| {
                        tag.map_borrowed(
                            |label| get_object(label, &mut bindings.objects, category),
                            |label| *bindings.morphisms.get(&label).unwrap(), // TODO: better error handling
                        )
                    })
                    .collect();
                let id = category
                    .new_morphism(Morphism {
                        connection,
                        tags,
                        inner,
                    })
                    .expect("objects should exist, because they have been binded");
                bindings.bind_morphism(label.clone(), id);
                Some((label.clone(), CategoryThing::Morphism { id }))
            }
            Constraint::Equality(f, g) => {
                let [f, g] = [f, g].map(|label| {
                    bindings
                        .get_morphism(label)
                        .expect("Morphisms in equality constraint must be created explicitly")
                });
                category.equalities.new_equality(f, g);
                None // TODO: allow equality input
            }
            Constraint::Commute { f, g, h } => {
                let [f, g, h] = [f, g, h].map(|label| {
                    bindings
                        .get_morphism(label)
                        .expect("Morphisms in equality constraint must be created explicitly")
                });
                category.equalities.new_commute(f, g, h);
                None // TODO: allow commute input
            }
        })
        .collect()
}
