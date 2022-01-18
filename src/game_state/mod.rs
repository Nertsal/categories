use force_graph::{ForceBody, ForceEdge, ForceParameters, ForceVertex};
use geng::{Camera2d, PixelPerfectCamera};

use graphs::{EdgeId, VertexId};

use super::*;

mod action;
mod constants;
mod drag;
mod draw;
mod focus;
mod goal;
mod graph_builder;
mod graph_link;
mod graph_types;
mod graph_util;
mod handle_event;
mod init;
mod label;
mod renderable;
mod rules;
mod selection;
mod state;
mod update;

use action::*;
use constants::*;
use drag::*;
use focus::*;
use graph_builder::*;
use graph_link::*;
use graph_types::*;
use label::*;
use renderable::*;
use rules::*;
use state::*;

pub struct GameState {
    geng: Geng,
    ui_camera: PixelPerfectCamera,
    state: State,
    rules: Rules,
    main_graph: RenderableGraph,
    goal_graph: RenderableGraph,
    graph_link: GraphLink,
    focused_graph: FocusedGraph,
    dragging: Option<Dragging>,
    main_selection: Option<RuleSelection>,
    goal_selection: Option<RuleSelection>,
    action_history: Vec<GraphAction>,
}

impl GameState {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        let state = State::default();
        let main_graph = RenderableGraph::new(
            geng,
            assets,
            init::graph::main_graph(),
            GraphEqualities::new(),
            vec2(1, 1),
        );
        let goal_graph = RenderableGraph::new(
            geng,
            assets,
            init::graph::goal_graph(),
            GraphEqualities::new(),
            vec2(1, 1),
        );
        Self {
            geng: geng.clone(),
            dragging: None,
            main_selection: None,
            goal_selection: None,
            focused_graph: FocusedGraph::Main,
            action_history: vec![],
            ui_camera: PixelPerfectCamera,
            rules: init::rules::default_rules(geng, assets),
            graph_link: GraphLink::new(&main_graph.graph, &goal_graph.graph),
            main_graph,
            goal_graph,
            state,
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
