use super::*;

pub struct RenderableRule {
    pub inner: Rule,
    pub inverse: Vec<Rule>,
    pub category: RenderableCategory,
    pub input: Vec<RuleInput<Label>>,
    pub inverse_input: Vec<RuleInput<Label>>,
    pub bindings: Bindings,
}

impl RenderableRule {
    pub fn from_rule(geng: &Geng, assets: &Rc<Assets>, rule: Rule) -> Self {
        fn part_color(part: category::RulePart) -> Color<f32> {
            match part {
                category::RulePart::Input => RULE_INPUT_COLOR,
                category::RulePart::Forall => RULE_FORALL_COLOR,
                category::RulePart::Exists => RULE_EXISTS_COLOR,
                category::RulePart::Inferred => RULE_INFER_COLOR,
                category::RulePart::Output => RULE_OUTPUT_COLOR,
            }
        }

        fn object_constructor(
            part: category::RulePart,
            label: &Label,
            _tags: &Vec<ObjectTag<Label>>,
        ) -> Point {
            Point::new(label, part_color(part))
        }

        fn morphism_constructor(
            part: category::RulePart,
            label: &Label,
            _tags: &Vec<MorphismTag<Label, Label>>,
        ) -> Arrow {
            Arrow::new(
                Some(label),
                part_color(part),
                util::random_shift(),
                util::random_shift(),
            )
        }

        fn equality_constructor(
            part: category::RulePart,
            _equality: &category::Equality<Label>,
        ) -> Equality {
            Equality {
                color: part_color(part),
            }
        }

        let (category, input, bindings) = Category::from_rule(
            &rule,
            object_constructor,
            morphism_constructor,
            equality_constructor,
        );

        let inverse = rule.invert();

        let inverse_input = inverse
            .last()
            .map(|rule| {
                Category::from_rule(
                    rule,
                    object_constructor,
                    morphism_constructor,
                    equality_constructor,
                )
                .1
            })
            .unwrap_or_default();

        Self {
            category: RenderableCategory::new(geng, assets, category, false),
            inner: rule,
            inverse,
            input,
            inverse_input,
            bindings,
        }
    }
}
