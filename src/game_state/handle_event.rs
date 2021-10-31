use super::*;

impl GameState {
    pub fn handle_event_impl(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseDown { position, button } => {
                self.dragging = Some(Dragging {
                    mouse_start_pos: position,
                    world_start_pos: self
                        .camera
                        .screen_to_world(self.framebuffer_size, position.map(|x| x as f32)),
                    mouse_button: button,
                })
            }
            geng::Event::MouseUp { position, button } => {
                self.drag_stop(position, button);
            }
            _ => (),
        }
    }

    fn drag_stop(&mut self, mouse_position: Vec2<f64>, mouse_button: geng::MouseButton) {
        if let Some(dragging) = self.dragging.take() {
            if dragging.mouse_button != mouse_button {
                return;
            }

            match mouse_button {
                geng::MouseButton::Left => {
                    let dragged_delta = mouse_position - dragging.mouse_start_pos;
                    if dragged_delta.len().approx_eq(&0.0) {
                        // Click
                        self.select_point(dragging.world_start_pos, SelectionOptions::New);
                    } else {
                        // Drag
                        let world_pos = self.camera.screen_to_world(
                            self.framebuffer_size,
                            mouse_position.map(|x| x as f32),
                        );
                        self.select_area(
                            AABB::from_corners(dragging.world_start_pos, world_pos),
                            SelectionOptions::New,
                        );
                    }
                }
                _ => (),
            }
        }
    }

    fn select_point(&mut self, position: Vec2<f32>, options: SelectionOptions) {
        // Vertices
        let selected_vertices = self
            .force_graph
            .graph
            .vertices
            .iter()
            .filter(|(_, vertex)| (vertex.body.position - position).len() <= vertex.vertex.radius)
            .map(|(&id, _)| id)
            .collect();

        // Edges
        let selected_edges = self
            .force_graph
            .graph
            .edges
            .iter()
            .filter(|(_, edge)| {
                self.force_graph
                    .graph
                    .vertices
                    .get(&edge.edge.from)
                    .map(|vertex| vertex.body.position)
                    .and_then(|arrow_start| {
                        self.force_graph
                            .graph
                            .vertices
                            .get(&edge.edge.from)
                            .map(|vertex| (arrow_start, vertex.body.position))
                    })
                    .map(|(arrow_start, arrow_end)| {
                        distance_point_segment(position, arrow_start, arrow_end) <= edge.edge.width
                    })
                    .unwrap_or(false)
            })
            .map(|(&id, _)| id)
            .collect();

        // Add to selection
        self.select(selected_vertices, selected_edges, options);
    }

    fn select_area(&mut self, area: AABB<f32>, options: SelectionOptions) {
        // Vertices
        let selected_vertices = self
            .force_graph
            .graph
            .vertices
            .iter()
            .filter(|(_, vertex)| {
                vertex.vertex.distance_to_aabb(vertex.body.position, &area) <= 0.0
            })
            .map(|(&id, _)| id)
            .collect();

        // Edges
        let selected_edges = self
            .force_graph
            .graph
            .edges
            .iter()
            .filter(|(_, edge)| {
                self.force_graph
                    .graph
                    .vertices
                    .get(&edge.edge.from)
                    .map(|vertex| vertex.body.position)
                    .and_then(|arrow_start| {
                        self.force_graph
                            .graph
                            .vertices
                            .get(&edge.edge.to)
                            .map(|vertex| (arrow_start, vertex.body.position))
                    })
                    .map(|(arrow_start, arrow_end)| {
                        overlap_aabb_segment(&area, arrow_start, arrow_end)
                    })
                    .unwrap_or(false)
            })
            .map(|(&id, _)| id)
            .collect();

        // Add to selection
        self.select(selected_vertices, selected_edges, options);
    }

    fn select(&mut self, vertices: Vec<VertexId>, edges: Vec<EdgeId>, options: SelectionOptions) {
        match options {
            SelectionOptions::New => {
                self.selection.clear_all();
                self.selection.select_vertices(vertices.into_iter());
                self.selection.select_edges(edges.into_iter());
            }
            SelectionOptions::Add => {
                self.selection.select_vertices(vertices.into_iter());
                self.selection.select_edges(edges.into_iter());
            }
            SelectionOptions::Change => {
                self.selection.change_vertices(vertices.into_iter());
                self.selection.change_edges(edges.into_iter());
            }
        }

        self.apply_rule(0);
    }
}

enum SelectionOptions {
    /// Clear previous selection
    New,
    /// Add the items in the area to the selection
    Add,
    /// Change the selection of the items in the area
    Change,
}

/// Calculate the distance from a point to a line segment
fn distance_point_segment(
    point: Vec2<f32>,
    segment_start: Vec2<f32>,
    segment_end: Vec2<f32>,
) -> f32 {
    // Project on the segment
    let delta = point - segment_start;
    let direction = segment_end - segment_start;
    let segment_len = direction.len();
    if segment_len < 1e-5 {
        // Segment is so small it resembles a point
        // Done to avoid division by 0
        return delta.len();
    }

    let direction_norm = direction / segment_len;
    let projection = Vec2::dot(delta, direction_norm);
    if projection < 0.0 {
        // The projection is outside of the line, closer to the start
        return delta.len();
    }
    if projection > segment_len {
        // The projection is outside of the line, closer to the end
        return (point - segment_end).len();
    }

    // Project on the normal
    let normal = direction_norm.rotate_90();
    let projection = Vec2::dot(delta, normal).abs();
    projection
}

/// Calculate whether the aabb and the segment overlap
fn overlap_aabb_segment(
    aabb: &AABB<f32>,
    segment_start: Vec2<f32>,
    segment_end: Vec2<f32>,
) -> bool {
    // Either one of segment points is inside
    // Or the segment intersects one of the edges
    if aabb.contains(segment_start) || aabb.contains(segment_end) {
        return true;
    }

    let top_left = aabb.top_left();
    let top_right = aabb.top_right();
    let bottom_left = aabb.bottom_left();
    let bottom_right = aabb.bottom_right();

    intersect_segment_segment(segment_start, segment_end, top_left, top_right).is_some()
        || intersect_segment_segment(segment_start, segment_end, bottom_left, bottom_right)
            .is_some()
        || intersect_segment_segment(segment_start, segment_end, bottom_right, top_right).is_some()
        || intersect_segment_segment(segment_start, segment_end, bottom_left, top_left).is_some()
}

/// Calculate the intersection point of two lines
fn intersect_segment_segment(
    segment_start: Vec2<f32>,
    segment_end: Vec2<f32>,
    other_start: Vec2<f32>,
    other_end: Vec2<f32>,
) -> Option<Vec2<f32>> {
    let segment_dir = segment_end - segment_start;
    let other_dir = other_end - other_start;

    fn cross(a: Vec2<f32>, b: Vec2<f32>) -> f32 {
        a.x * b.y - a.y * b.x
    }

    let start_delta = other_start - segment_start;
    let rxs = cross(segment_dir, other_dir);

    if rxs == 0.0 {
        // Collinear
        return None;
    }

    let tx = cross(start_delta, other_dir);
    let t = tx / rxs;
    let ux = cross(start_delta, segment_dir);
    let u = ux / rxs;

    if t < 0.0 || t > 1.0 || u < 0.0 || u > 1.0 {
        // No intersection
        return None;
    }

    // Intersection
    Some(segment_start + segment_dir * t)
}
