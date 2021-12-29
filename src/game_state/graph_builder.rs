use super::*;

pub struct GraphBuilder {
    graph: Graph,
    objects: HashMap<String, VertexId>,
    morphisms: HashMap<String, EdgeId>,
}

impl GraphBuilder {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(ForceParameters::default()),
            objects: HashMap::new(),
            morphisms: HashMap::new(),
        }
    }

    pub fn build(self) -> Graph {
        self.graph
    }

    pub fn object(
        mut self,
        label: impl Into<Label>,
        tag: Option<ObjectTag<Label>>,
        color: Color<f32>,
        anchor: bool,
    ) -> Self {
        let label = label.into();
        let new_object = self.graph.graph.new_vertex(ForceVertex {
            is_anchor: anchor,
            body: ForceBody {
                position: util::random_shift(),
                mass: POINT_MASS,
                velocity: Vec2::ZERO,
            },
            vertex: Point {
                label: label.clone(),
                radius: POINT_RADIUS,
                tag: tag.map(|tag| {
                    tag.map(|label| match label {
                        Label::Name(label) => Some(self.objects[&label]),
                        Label::Any => None,
                    })
                }),
                color,
            },
        });

        match label {
            Label::Name(label) => {
                self.objects.insert(label, new_object);
            }
            Label::Any => (),
        }

        self
    }

    pub fn morphism(
        mut self,
        label: impl Into<Label>,
        from: &str,
        to: &str,
        tag: Option<MorphismTag<Label, Label>>,
    ) -> Self {
        let label = label.into();
        let color = draw::graph::morphism_color(&tag);
        let new_edge = self.graph.graph.new_edge(ForceEdge::new(
            util::random_shift(),
            util::random_shift(),
            ARROW_BODIES,
            ARROW_MASS,
            Arrow {
                label: label.clone(),
                from: self.objects[from],
                to: self.objects[to],
                tag: tag.map(|tag| {
                    tag.map(
                        |label| match label {
                            Label::Name(label) => Some(self.objects[&label]),
                            Label::Any => None,
                        },
                        |label| match label {
                            Label::Name(label) => Some(self.morphisms[&label]),
                            Label::Any => None,
                        },
                    )
                }),
                color,
            },
        ));

        match label {
            Label::Name(label) => {
                self.morphisms.insert(label.to_owned(), new_edge.unwrap());
            }
            Label::Any => (),
        }

        self
    }
}
