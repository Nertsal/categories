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
    Inferred,
    Output,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleInput<L> {
    Object {
        label: L,
        id: ObjectId,
    },
    Morphism {
        label: L,
        id: MorphismId,
    },
    Equality {
        left: Vec<(L, MorphismId)>,
        right: Vec<(L, MorphismId)>,
    },
}

impl<O, M, E> Category<O, M, E> {
    pub fn from_rule<L: Label>(
        rule: &Rule<L>,
        object_constructor: impl Fn(RulePart, &L, &Vec<ObjectTag<L>>) -> O,
        morphism_constructor: impl Fn(RulePart, &L, &Vec<MorphismTag<L, L>>) -> M,
        equality_constructor: impl Fn(RulePart, &Equality<L>) -> E,
    ) -> (Self, Vec<RuleInput<L>>, Bindings<L>) {
        let statement = rule.get_statement();
        let statement_len = statement.len();
        let mut statement_iter = statement.iter();

        let mut category = Category::new();
        let mut bindings = Bindings::new();

        let input = statement_iter
            .next()
            .map(|construction| match construction {
                RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
                    add_constraints(
                        RulePart::Input,
                        constraints,
                        statement,
                        &mut bindings,
                        &mut category,
                        &object_constructor,
                        &morphism_constructor,
                        &equality_constructor,
                    )
                }
            })
            .unwrap_or_default();

        for _ in 1..statement_len - 1 {
            let construction = statement_iter
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
                        statement,
                        &mut bindings,
                        &mut category,
                        &object_constructor,
                        &morphism_constructor,
                        &equality_constructor,
                    );
                }
            }
        }

        if let Some(construction) = statement_iter.next() {
            match construction {
                RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
                    add_constraints(
                        RulePart::Output,
                        constraints,
                        statement,
                        &mut bindings,
                        &mut category,
                        &object_constructor,
                        &morphism_constructor,
                        &equality_constructor,
                    );
                }
            }
        }

        (category, input, bindings)
    }
}

fn add_constraints<'a, O, M, E, L: 'a + Label>(
    rule_part: RulePart,
    constraints: impl IntoIterator<Item = &'a Constraint<L>>,
    statement: &[RuleConstruction<L>],
    bindings: &mut Bindings<L>,
    category: &mut Category<O, M, E>,
    object_constructor: impl Fn(RulePart, &L, &Vec<ObjectTag<L>>) -> O,
    morphism_constructor: impl Fn(RulePart, &L, &Vec<MorphismTag<L, L>>) -> M,
    equality_constructor: impl Fn(RulePart, &Equality<L>) -> E,
) -> Vec<RuleInput<L>> {
    let object_constructor = &object_constructor;

    fn get_object<O, M, E, L: Label>(
        label: &L,
        rule_part: RulePart,
        objects: &mut HashMap<L, ObjectId>,
        category: &mut Category<O, M, E>,
        statement: &[RuleConstruction<L>],
        object_constructor: &impl Fn(RulePart, &L, &Vec<ObjectTag<L>>) -> O,
    ) -> ObjectId {
        objects.get(label).copied().unwrap_or_else(|| {
            let empty_tags = vec![];
            let tags = statement
                .iter()
                .flat_map(|construction| match construction {
                    RuleConstruction::Forall(c) | RuleConstruction::Exists(c) => c,
                })
                .find_map(|constraint| match constraint {
                    Constraint::Object { label: l, tags } if *l == *label => Some(tags),
                    _ => None,
                })
                .unwrap_or(&empty_tags);

            let inner = object_constructor(rule_part, label, tags);
            let tags = tags
                .iter()
                .map(|tag| {
                    tag.map_borrowed(|label| {
                        get_object(
                            label,
                            rule_part,
                            objects,
                            category,
                            statement,
                            object_constructor,
                        )
                    })
                })
                .collect();

            let id = category.new_object(Object { tags, inner });
            objects.insert(label.clone(), id);
            id
        })
    }

    let get_morphism = |label: &L, bindings: &mut Bindings<L>, category: &mut Category<O, M, E>| {
        bindings.morphisms.get(label).copied().unwrap_or_else(|| {
            let (connection, tags) = statement
                .iter()
                .flat_map(|construction| match construction {
                    RuleConstruction::Forall(c) | RuleConstruction::Exists(c) => c,
                })
                .find_map(|constraint| match constraint {
                    Constraint::Morphism {
                        label: l,
                        connection,
                        tags,
                    } if *l == *label => Some((connection, tags)),
                    _ => None,
                })
                .expect("Equality/Commute constraints expect morphism to be created explicitly");

            let connection = connection.map_borrowed(|label| {
                get_object(
                    label,
                    RulePart::Inferred,
                    &mut bindings.objects,
                    category,
                    statement,
                    object_constructor,
                )
            });
            let inner = morphism_constructor(rule_part, label, tags);

            let tags = tags
                .iter()
                .map(|tag| {
                    tag.map_borrowed(
                        |label| {
                            get_object(
                                label,
                                RulePart::Inferred,
                                &mut bindings.objects,
                                category,
                                statement,
                                object_constructor,
                            )
                        },
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
                .unwrap(); // objects should exist, because they have been binded
            bindings.bind_morphism(label.clone(), id);
            id
        })
    };

    constraints
        .into_iter()
        .filter_map(|constraint| match constraint {
            Constraint::Object { label, .. } => {
                let id = get_object(
                    label,
                    rule_part,
                    &mut bindings.objects,
                    category,
                    statement,
                    object_constructor,
                );
                bindings.bind_object(label.clone(), id);
                Some(RuleInput::Object {
                    label: label.clone(),
                    id,
                })
            }
            Constraint::Morphism { label, .. } => {
                let id = get_morphism(label, bindings, category);
                bindings.bind_morphism(label.clone(), id);
                Some(RuleInput::Morphism {
                    label: label.clone(),
                    id,
                })
            }
            Constraint::Equality(equality) => {
                let inner = equality_constructor(rule_part, equality);
                let (left_input, left_eq) = equality
                    .left()
                    .iter()
                    .map(|label| {
                        let id = get_morphism(label, bindings, category);
                        ((label.clone(), id), id)
                    })
                    .unzip();
                let (right_input, right_eq) = equality
                    .right()
                    .iter()
                    .map(|label| {
                        let id = get_morphism(label, bindings, category);
                        ((label.clone(), id), id)
                    })
                    .unzip();
                let equality =
                    Equality::new(left_eq, right_eq).expect("Failed to construct equality");

                category.equalities.new_equality(equality, inner);
                Some(RuleInput::Equality {
                    left: left_input,
                    right: right_input,
                })
            }
        })
        .collect()
}
