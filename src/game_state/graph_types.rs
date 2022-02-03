use super::*;

pub type Equalities = HashSet<(MorphismId, MorphismId)>;

pub struct CategoryWrapper {
    pub inner: category::types::Category,
    pub objects: HashMap<ObjectId, Point>,
    pub morphisms: HashMap<MorphismId, Arrow>,
}

impl CategoryWrapper {
    pub fn new(category: category::types::Category) -> Self {
        let (objects, morphisms) = category_to_things(&category);
        Self {
            inner: category,
            objects,
            morphisms,
        }
    }
}

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
    pub fn new(label: Label, position: Vec2<f32>, is_anchor: bool, color: Color<f32>) -> Self {
        Self {
            velocity: Vec2::ZERO,
            radius: POINT_RADIUS,
            label,
            is_anchor,
            position,
            color,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Arrow {
    pub label: Label,
    pub positions: Vec<Vec2<f32>>,
    pub velocities: Vec<Vec2<f32>>,
    pub color: Color<f32>,
}

impl Arrow {
    pub fn new(label: Label, color: Color<f32>, pos_a: Vec2<f32>, pos_b: Vec2<f32>) -> Self {
        Self {
            positions: (0..ARROW_BODIES)
                .map(|i| {
                    pos_a + (pos_b - pos_a) / ARROW_BODIES as f32 * i as f32 + util::random_shift()
                })
                .collect(),
            velocities: (0..ARROW_BODIES).map(|_| Vec2::ZERO).collect(),
            label,
            color,
        }
    }
}

fn category_to_things(
    category: &category::types::Category,
) -> (HashMap<ObjectId, Point>, HashMap<MorphismId, Arrow>) {
    todo!()
}
