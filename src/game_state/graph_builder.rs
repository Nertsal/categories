use super::*;

pub struct GraphBuilder {
    graph: Graph,
    objects: HashMap<Label, VertexId>,
    morphisms: HashMap<Label, EdgeId>,
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
        label: impl Into<RuleLabel>,
        tags: Vec<ObjectTag>,
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
                tags: tags
                    .into_iter()
                    .map(|tag| {
                        tag.map(|label| {
                            label.and_then(|label| match label {
                                RuleLabel::Name(label) => Some(self.objects[&label]),
                                RuleLabel::Any => None,
                            })
                        })
                    })
                    .collect(),
                color,
            },
        });

        match label {
            RuleLabel::Name(label) => {
                self.objects.insert(label, new_object);
            }
            RuleLabel::Any => (),
        }

        self
    }

    pub fn morphism(
        mut self,
        label: impl Into<RuleLabel>,
        from: &str,
        to: &str,
        tags: Vec<MorphismTag>,
    ) -> Self {
        let label = label.into();
        let color = draw::graph::morphism_color(&tags);
        let new_edge = self.graph.graph.new_edge(ForceEdge::new(
            util::random_shift(),
            util::random_shift(),
            ARROW_BODIES,
            ARROW_MASS,
            Arrow {
                label: label.clone(),
                from: self.objects[from],
                to: self.objects[to],
                tags: tags
                    .into_iter()
                    .map(|tag| {
                        tag.map(
                            |label| {
                                label.and_then(|label| match label {
                                    RuleLabel::Name(label) => Some(self.objects[&label]),
                                    RuleLabel::Any => None,
                                })
                            },
                            |label| {
                                label.and_then(|label| match label {
                                    RuleLabel::Name(label) => Some(self.morphisms[&label]),
                                    RuleLabel::Any => None,
                                })
                            },
                        )
                    })
                    .collect(),
                color,
            },
        ));

        match label {
            RuleLabel::Name(label) => {
                self.morphisms.insert(label.to_owned(), new_edge.unwrap());
            }
            RuleLabel::Any => (),
        }

        self
    }
}
