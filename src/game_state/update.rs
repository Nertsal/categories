use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        // Apply forces to objects/morphisms
        for category in [&mut self.fact_category.inner, &mut self.goal_category.inner]
            .into_iter()
            .chain(self.rules.iter_mut().map(|rule| &mut rule.category.inner))
        {
            update_category(category, delta_time);
        }

        self.update_cameras_bounds();

        // Mouse update
        self.drag_update();
    }

    fn update_cameras_bounds(&mut self) {
        for (category, camera, framebuffer_size) in [
            (
                &self.fact_category.inner,
                &mut self.fact_category.camera,
                self.fact_category.texture_size,
            ),
            (
                &self.goal_category.inner,
                &mut self.goal_category.camera,
                self.goal_category.texture_size,
            ),
        ]
        .into_iter()
        .chain(self.rules.iter_mut().map(|rule| {
            (
                &rule.category.inner,
                &mut rule.category.camera,
                rule.category.texture_size,
            )
        })) {
            let mut positions = category
                .objects
                .iter()
                .map(|(_, object)| object.inner.position)
                .chain(
                    category
                        .morphisms
                        .iter()
                        .flat_map(|(_, morphism)| morphism.inner.positions.iter().copied()),
                );
            if let Some(pos) = positions.next() {
                let bounds = AABB::points_bounding_box(std::iter::once(pos).chain(positions));
                camera.update_bounds(bounds, framebuffer_size.map(|x| x as f32));
            }
        }
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
                position: &mut object.inner.position,
                velocity: &mut object.inner.velocity,
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

    for (&id, _) in category
        .objects
        .iter()
        .filter(|(_, object)| !object.inner.is_anchor)
    {
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

        // TODO: better error handling?
        let (from, to) = match category.morphisms.get(&id).unwrap().connection {
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

fn update_category(category: &mut Category, delta_time: f32) {
    let connections = connections(&category);
    let mut bodies = bodies_collection(category);
    force_graph::apply_forces(&default(), delta_time, &mut bodies, &connections)
}
