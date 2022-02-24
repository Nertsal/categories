use super::*;

impl<L: Label> Rule<L> {
    pub fn invert(&self) -> Vec<Self> {
        invert_statement(&self.statement)
            .into_iter()
            .map(|statement| Self { statement })
            .collect()
    }
}

fn invert_statement<L: Label>(statement: &RuleStatement<L>) -> Vec<RuleStatement<L>> {
    let mut prelude_forall = Vec::new();
    let mut prelude_exists = Vec::new();
    let mut statements = Vec::new();

    let mut last_forall = None;

    for construction in statement {
        match construction {
            RuleConstruction::Forall(constraints) => {
                if let Some(forall) = last_forall.replace(constraints.clone()) {
                    prelude_exists.extend(forall);
                }
            }
            RuleConstruction::Exists(constraints) => {
                if let Some(forall) = last_forall.take() {
                    // Construct an inverse rule
                    let inv_forall = invert_constraints(constraints);
                    let mut exist = prelude_exists.clone();
                    exist.extend(forall.clone());

                    let mut statement = Vec::new();
                    statement.push(RuleConstruction::Forall(inv_forall));
                    if !prelude_forall.is_empty() {
                        statement.push(RuleConstruction::Forall(prelude_forall.clone()));
                    }
                    statement.push(RuleConstruction::Exists(exist));

                    statements.push(statement);
                    prelude_forall.extend(forall);
                }
                prelude_forall.extend(constraints.clone());
            }
        };
    }

    statements
}

fn invert_constraints<L: Label>(constraints: &Constraints<L>) -> Constraints<L> {
    constraints
        .iter()
        .map(|constraint| match constraint {
            Constraint::Object { .. } => constraint.clone(),
            Constraint::Morphism {
                label,
                connection,
                tags,
            } => Constraint::Morphism {
                label: label.clone(),
                connection: connection.clone(),
                tags: tags
                    .iter()
                    .filter_map(|tag| match tag {
                        MorphismTag::Identity(_) => Some(tag.clone()),
                        _ => None,
                    })
                    .collect(),
            },
            Constraint::Equality(_) => constraint.clone(),
        })
        .collect()
}
