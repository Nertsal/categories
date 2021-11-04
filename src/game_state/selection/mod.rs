use super::*;

mod geom;
mod select;

use geom::*;
pub use select::*;

pub struct Selection {
    pub vertices: HashSet<VertexId>,
    pub edges: HashSet<EdgeId>,
}

impl Selection {
    pub fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    pub fn clear_all(&mut self) {
        self.clear_vertices();
        self.clear_edges();
    }

    pub fn clear_vertices(&mut self) {
        self.vertices.clear();
    }

    pub fn clear_edges(&mut self) {
        self.edges.clear();
    }

    pub fn change_vertices(&mut self, vertices: impl Iterator<Item = VertexId>) {
        for vertex in vertices {
            if !self.vertices.insert(vertex) {
                self.vertices.remove(&vertex);
            }
        }
    }

    pub fn change_edges(&mut self, edges: impl Iterator<Item = EdgeId>) {
        for edge in edges {
            if !self.edges.insert(edge) {
                self.edges.remove(&edge);
            }
        }
    }

    pub fn select_vertices(&mut self, vertices: impl Iterator<Item = VertexId>) {
        self.vertices.extend(vertices);
    }

    pub fn select_edges(&mut self, edges: impl Iterator<Item = EdgeId>) {
        self.edges.extend(edges);
    }
}
