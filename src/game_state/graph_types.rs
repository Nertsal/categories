use super::*;

pub type Category = category::Category<Point<ObjectId>, Arrow<ObjectId, MorphismId>>;
pub type Object = Point<ObjectId>;
pub type Morphism = category::Morphism<Arrow<ObjectId, MorphismId>>;
pub type Equalities = HashSet<(MorphismId, MorphismId)>;

#[derive(Debug, Clone)]
pub struct Point<O> {
    pub label: Label,
    pub is_anchor: bool,
    pub position: Vec2<f32>,
    pub velocity: Vec2<f32>,
    pub radius: f32,
    pub tag: Option<ObjectTag<Option<O>>>,
    pub color: Color<f32>,
}

impl<O> Point<O> {
    pub fn new(
        label: Label,
        tag: Option<ObjectTag<Option<O>>>,
        position: Vec2<f32>,
        is_anchor: bool,
        color: Color<f32>,
    ) -> Self {
        Self {
            velocity: Vec2::ZERO,
            radius: POINT_RADIUS,
            label,
            is_anchor,
            position,
            tag,
            color,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Arrow<O, M> {
    pub label: Label,
    pub positions: Vec<Vec2<f32>>,
    pub velocities: Vec<Vec2<f32>>,
    pub tag: Option<MorphismTag<Option<O>, Option<M>>>,
    pub color: Color<f32>,
}

impl<O, M> Arrow<O, M> {
    pub fn new(
        label: Label,
        tag: Option<MorphismTag<Option<O>, Option<M>>>,
        color: Color<f32>,
        pos_a: Vec2<f32>,
        pos_b: Vec2<f32>,
    ) -> Self {
        Self {
            positions: (0..ARROW_BODIES)
                .map(|i| {
                    pos_a + (pos_b - pos_a) / ARROW_BODIES as f32 * i as f32 + util::random_shift()
                })
                .collect(),
            velocities: (0..ARROW_BODIES).map(|_| Vec2::ZERO).collect(),
            label,
            tag,
            color,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CategoryThing {
    Object { id: ObjectId },
    Morphism { id: MorphismId },
}
