use super::*;

pub fn default_graph() -> Graph {
    let mut graph = Graph::new(ForceParameters::default());

    let mut objects = HashMap::new();
    let mut morphisms = HashMap::new();

    let mut rng = thread_rng();

    let mut object = |graph: &mut Graph,
                      objects: &mut HashMap<Label, VertexId>,
                      label: &str,
                      tags: Vec<ObjectTag<&str>>,
                      color: Color<f32>,
                      anchor: bool| {
        let new_object = graph.graph.new_vertex(ForceVertex {
            is_anchor: anchor,
            body: ForceBody {
                position: vec2(rng.gen(), rng.gen()),
                mass: POINT_MASS,
                velocity: Vec2::ZERO,
            },
            vertex: Point {
                label: label.to_owned(),
                radius: POINT_RADIUS,
                tags: tags
                    .into_iter()
                    .map(|tag| tag.map(|o| objects[o]))
                    .collect(),
                color,
            },
        });
        objects.insert(label.to_owned(), new_object);
    };

    let mut rng = thread_rng();
    let mut morphism = |graph: &mut Graph,
                        objects: &HashMap<Label, VertexId>,
                        morphisms: &mut HashMap<Label, EdgeId>,
                        label: &str,
                        from: &str,
                        to: &str,
                        tags: Vec<MorphismTag<&str, &str>>| {
        let color = draw::graph::morphism_color(&tags);
        let new_edge = graph.graph.new_edge(ForceEdge::new(
            vec2(rng.gen(), rng.gen()),
            vec2(rng.gen(), rng.gen()),
            ARROW_BODIES,
            ARROW_MASS,
            Arrow::new(
                label,
                objects[from],
                objects[to],
                tags.into_iter()
                    .map(|tag| tag.map(|o| objects[o], |m| morphisms[m]))
                    .collect(),
                color,
            ),
        ));
        morphisms.insert(label.to_owned(), new_edge.unwrap());
    };

    object(&mut graph, &mut objects, "A", vec![], Color::WHITE, false);
    object(&mut graph, &mut objects, "B", vec![], Color::WHITE, false);
    object(&mut graph, &mut objects, "C", vec![], Color::WHITE, false);
    object(
        &mut graph,
        &mut objects,
        "AxB",
        vec![ObjectTag::Product("A", "B")],
        Color::WHITE,
        false,
    );
    object(
        &mut graph,
        &mut objects,
        "BxC",
        vec![ObjectTag::Product("B", "C")],
        Color::WHITE,
        false,
    );
    object(
        &mut graph,
        &mut objects,
        "(AxB)xC",
        vec![ObjectTag::Product("AxB", "C")],
        Color::WHITE,
        false,
    );
    object(
        &mut graph,
        &mut objects,
        "Ax(BxC)",
        vec![ObjectTag::Product("A", "BxC")],
        Color::WHITE,
        false,
    );

    morphism(&mut graph, &objects, &mut morphisms, "", "AxB", "A", vec![]);
    morphism(&mut graph, &objects, &mut morphisms, "", "AxB", "B", vec![]);
    morphism(&mut graph, &objects, &mut morphisms, "", "BxC", "B", vec![]);
    morphism(&mut graph, &objects, &mut morphisms, "", "BxC", "C", vec![]);
    morphism(
        &mut graph,
        &objects,
        &mut morphisms,
        "",
        "(AxB)xC",
        "AxB",
        vec![],
    );
    morphism(
        &mut graph,
        &objects,
        &mut morphisms,
        "",
        "(AxB)xC",
        "C",
        vec![],
    );
    morphism(
        &mut graph,
        &objects,
        &mut morphisms,
        "",
        "Ax(BxC)",
        "A",
        vec![],
    );
    morphism(
        &mut graph,
        &objects,
        &mut morphisms,
        "",
        "Ax(BxC)",
        "BxC",
        vec![],
    );

    graph
}
