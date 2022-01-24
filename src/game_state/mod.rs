use category::{MorphismConnection, MorphismId, ObjectId};
use geng::{Camera2d, PixelPerfectCamera};

use super::*;

mod action;
mod constants;
mod drag;
mod draw;
mod focus;
mod goal;
mod category_builder;
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

use action::GraphAction;
use constants::*;
use drag::*;
use focus::*;
use category_builder::*;
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
    fact_category: RenderableCategory,
    goal_category: RenderableCategory,
    graph_link: GraphLink,
    focused_category: FocusedCategory,
    dragging: Option<Dragging>,
    main_selection: Option<RuleSelection>,
    goal_selection: Option<RuleSelection>,
    action_history: Vec<GraphAction>,
}

impl GameState {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        let state = State::default();
        let fact_category = RenderableCategory::new(
            geng,
            assets,
            init::category::fact_category(),
            Equalities::new(),
            vec2(1, 1),
        );
        let goal_category = RenderableCategory::new(
            geng,
            assets,
            init::category::goal_category(),
            Equalities::new(),
            vec2(1, 1),
        );
        Self {
            geng: geng.clone(),
            dragging: None,
            main_selection: None,
            goal_selection: None,
            focused_category: FocusedCategory::Fact,
            action_history: vec![],
            ui_camera: PixelPerfectCamera,
            rules: init::rules::default_rules(geng, assets),
            graph_link: GraphLink::new(&fact_category.inner, &goal_category.inner),
            fact_category,
            goal_category,
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
