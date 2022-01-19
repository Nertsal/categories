use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        // Update focus
        self.focus(self.geng.window().cursor_position());

        // Apply forces to objects/morphisms
        for category in vec![&mut self.fact_category, &mut self.goal_category]
            .into_iter()
            .chain(self.rules.iter_mut().map(|rule| rule.get_category_mut()))
        {
            update_category(category, delta_time);
        }

        // Mouse update
        self.drag_update();
    }
}

type BodiesCollection<'a> = HashMap<BodyId, PhysicsBody<'a>>;
type Connections = HashMap<BodyId, Vec<BodyId>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BodyId {
    Object { id: ObjectId },
    MorphismPart { id: MorphismId, part: usize },
}

struct PhysicsBody<'a> {
    is_vertex: bool,
    mass: f32,
    position: &'a mut Vec2<f32>,
    velocity: &'a mut Vec2<f32>,
}

impl<'a> force_graph::PhysicsBody for PhysicsBody<'a> {
    fn is_vertex(&self) -> bool {
        self.is_vertex
    }
    fn get_mass(&self) -> f32 {
        self.mass
    }
    fn get_position(&self) -> Vec2<f32> {
        *self.position
    }
    fn set_position(&mut self, position: Vec2<f32>) {
        *self.position = position
    }
    fn get_velocity(&self) -> Vec2<f32> {
        *self.velocity
    }
    fn set_velocity(&mut self, velocity: Vec2<f32>) {
        *self.velocity = velocity
    }
}

fn bodies_collection<'a>(category: &'a mut Category) -> BodiesCollection<'a> {
    let mut bodies = BodiesCollection::new();
    for (&id, object) in category.objects.iter_mut() {
        bodies.insert(
            BodyId::Object { id },
            PhysicsBody {
                is_vertex: true,
                mass: POINT_MASS,
                position: &mut object.position,
                velocity: &mut object.velocity,
            },
        );
    }
    for (&id, morphism) in category.morphisms.iter_mut() {
        bodies.extend(
            morphism
                .inner
                .positions
                .iter_mut()
                .zip(morphism.inner.velocities.iter_mut())
                .enumerate()
                .map(|(index, (position, velocity))| {
                    (
                        BodyId::MorphismPart { id, part: index },
                        PhysicsBody {
                            is_vertex: false,
                            mass: ARROW_MASS,
                            position,
                            velocity,
                        },
                    )
                }),
        );
    }
    bodies
}

fn connections(category: &Category) -> Connections {
    let mut connections = Connections::new();

    for (&id, _) in category.objects.iter() {
        let neighbours = category
            .neighbours(id)
            .map(|id| BodyId::Object { id })
            .collect();
        connections.insert(BodyId::Object { id }, neighbours);
    }

    for (&id, morphism) in category.morphisms.iter() {
        let parts = morphism
            .inner
            .positions
            .len()
            .min(morphism.inner.velocities.len());

        let (from, to) = match morphism.connection {
            MorphismConnection::Regular { from, to } => (from, to),
            MorphismConnection::Isomorphism(a, b) => (a, b),
        };

        if parts > 0 {
            let mut neighbours = vec![BodyId::Object { id: from }];
            if parts > 1 {
                neighbours.push(BodyId::MorphismPart { id, part: 1 });
            } else {
                neighbours.push(BodyId::Object { id: to });
            }
            connections.insert(BodyId::MorphismPart { id, part: 0 }, neighbours);
        }
        for i in 1..parts - 1 {
            let neighbours = vec![
                BodyId::MorphismPart { id, part: i - 1 },
                BodyId::MorphismPart { id, part: i + 1 },
            ];
            connections.insert(BodyId::MorphismPart { id, part: i }, neighbours);
        }
        if parts > 1 {
            let neighbours = vec![
                BodyId::MorphismPart {
                    id,
                    part: parts - 2,
                },
                BodyId::Object { id: to },
            ];
            connections.insert(
                BodyId::MorphismPart {
                    id,
                    part: parts - 1,
                },
                neighbours,
            );
        }
    }

    connections
}

fn update_category(category: &mut RenderableCategory, delta_time: f32) {
    let connections = connections(&category.inner);
    let mut bodies = bodies_collection(&mut category.inner);
    force_graph::apply_forces(&default(), delta_time, &mut bodies, &connections)
}
