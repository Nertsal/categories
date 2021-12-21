use force_graph::{ForceBody, ForceEdge, ForceParameters, ForceVertex};
use geng::{prelude::rand::thread_rng, Camera2d};

use graphs::{EdgeId, GraphObject, VertexId};

use super::*;

mod action;
mod constants;
mod drag;
mod draw;
mod focus;
mod graph_types;
mod handle_event;
mod init;
mod rules;
mod selection;
mod update;
mod util;

use action::*;
use constants::*;
use drag::*;
use focus::*;
use graph_types::*;
use rules::*;
use util::*;

pub struct GameState {
    geng: Geng,
    assets: Rc<Assets>,
    camera: Camera2d,
    framebuffer_size: Vec2<f32>,
    main_graph: Graph,
    rules: Rules,
    focused_graph: FocusedGraph,
    dragging: Option<Dragging>,
    selection: Option<RuleSelection>,
    action_history: Vec<GraphAction>,
}

impl GameState {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            dragging: None,
            framebuffer_size: vec2(1.0, 1.0),
            selection: None,
            focused_graph: FocusedGraph::Main,
            action_history: vec![],
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 100.0,
            },
            rules: init::rules::default_rules(geng, assets),
            main_graph: init::graph::default_graph(),
        }
    }
}

impl geng::State for GameState {
    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;
        self.update_impl(delta_time);
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.draw_impl(framebuffer);
    }

    fn handle_event(&mut self, event: geng::Event) {
        self.handle_event_impl(event);
    }
}
