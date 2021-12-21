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
        label: &str,
        tags: Vec<ObjectTag<&str>>,
        color: Color<f32>,
        anchor: bool,
    ) -> Self {
        let new_object = self.graph.graph.new_vertex(ForceVertex {
            is_anchor: anchor,
            body: ForceBody {
                position: util::random_shift(),
                mass: POINT_MASS,
                velocity: Vec2::ZERO,
            },
            vertex: Point {
                label: label.to_owned(),
                radius: POINT_RADIUS,
                tags: tags
                    .into_iter()
                    .map(|tag| tag.map(|o| self.objects[o]))
                    .collect(),
                color,
            },
        });
        self.objects.insert(label.to_owned(), new_object);
        self
    }

    pub fn morphism(
        mut self,
        label: &str,
        from: &str,
        to: &str,
        tags: Vec<MorphismTag<&str, &str>>,
    ) -> Self {
        let color = draw::graph::morphism_color(&tags);
        let new_edge = self.graph.graph.new_edge(ForceEdge::new(
            util::random_shift(),
            util::random_shift(),
            ARROW_BODIES,
            ARROW_MASS,
            Arrow::new(
                label,
                self.objects[from],
                self.objects[to],
                tags.into_iter()
                    .map(|tag| tag.map(|o| self.objects[o], |m| self.morphisms[m]))
                    .collect(),
                color,
            ),
        ));
        self.morphisms.insert(label.to_owned(), new_edge.unwrap());
        self
    }
}
