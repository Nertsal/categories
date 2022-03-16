use super::*;

#[derive(Debug, Clone)]
pub struct Point {
    pub label: Label,
    pub is_anchor: bool,
    pub position: Vec2<f32>,
    pub velocity: Vec2<f32>,
    pub radius: f32,
    pub color: Color<f32>,
}

impl Point {
    pub fn new<L: Into<Label>>(label: L, color: Color<f32>) -> Self {
        Self {
            velocity: Vec2::ZERO,
            radius: POINT_RADIUS,
            is_anchor: false,
            position: util::random_shift(),
            label: label.into(),
            color,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Arrow {
    pub label: Option<Label>,
    pub positions: Vec<Vec2<f32>>,
    pub velocities: Vec<Vec2<f32>>,
    pub color: Color<f32>,
}

impl Arrow {
    pub fn new<L: Into<Label>>(
        label: Option<L>,
        color: Color<f32>,
        pos_a: Vec2<f32>,
        pos_b: Vec2<f32>,
    ) -> Self {
        Self {
            label: label.map(|label| label.into()),
            positions: (0..ARROW_BODIES)
                .map(|i| {
                    pos_a + (pos_b - pos_a) / ARROW_BODIES as f32 * i as f32 + util::random_shift()
                })
                .collect(),
            velocities: (0..ARROW_BODIES).map(|_| Vec2::ZERO).collect(),
            color,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Equality {
    pub color: Color<f32>,
}

pub fn object_name_from_tag_label(tag: &ObjectTag<&str>) -> Option<String> {
    match &tag {
        ObjectTag::Product(a, b) => label_operation(a, b, "x"),
        _ => None,
    }
}

pub fn infer_morphism_name(morphism: &Morphism, category: &Category) -> Option<String> {
    morphism.inner.label.clone().or_else(|| {
        morphism
            .tags
            .iter()
            .find_map(|tag| morphism_name_from_tag(tag, category))
    })
}

pub fn morphism_name_from_tag(tag: &MorphismTag, category: &Category) -> Option<String> {
    match tag {
        MorphismTag::Identity(_) => Some(format!("id")),
        MorphismTag::Isomorphism(_, _) => Some(format!("")),
        MorphismTag::Composition { first, second } => {
            let mut decomposed = category::util::decompose_morphism(*first, category);
            let second = category::util::decompose_morphism(*second, category);
            decomposed.extend(second);
            let mut result = String::new();
            let to_label = |id| {
                category
                    .morphisms
                    .get(&id)
                    .unwrap()
                    .inner
                    .label
                    .clone()
                    .unwrap_or_else(|| format!("{}", id.raw()))
            };

            let mut decomposed = decomposed.into_iter().rev();
            result += &to_label(decomposed.next().unwrap());
            for id in decomposed {
                result += " o "; // TODO: use better symbol
                result += &to_label(id);
            }

            Some(result)
        }
        _ => None,
    }
}

fn label_operation(label_a: &str, label_b: &str, operation: &str) -> Option<String> {
    if label_a.is_empty() || label_b.is_empty() {
        return None;
    }

    let first = if label_a.contains(operation) {
        format!("({})", label_a)
    } else {
        format!("{}", label_a)
    };

    let second = if label_b.contains(operation) {
        format!("({})", label_b)
    } else {
        format!("{}", label_b)
    };

    Some(format!("{}{}{}", first, operation, second))
}
