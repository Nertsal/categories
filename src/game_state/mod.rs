use category::prelude::*;
use geng::{Camera2d, PixelPerfectCamera};

use super::*;

mod constants;
mod drag;
mod draw;
mod focus;
mod goal;
mod graph_link;
mod graph_types;
mod graph_util;
mod handle_event;
mod init;
mod renderable;
mod selection;
mod state;
mod update;

use constants::*;
use drag::*;
use focus::*;
use graph_link::*;
use graph_types::*;
use renderable::*;
use selection::RuleSelection;
use state::*;

type Label = String;
type Rule = category::Rule<Label>;
type Bindings = category::Bindings<Label>;
type Constraints = category::Constraints<Label>;

pub struct GameState {
    geng: Geng,
    ui_camera: PixelPerfectCamera,
    state: State,
    rules: Vec<RenderableRule>,
    fact_category: RenderableCategory,
    goal_category: RenderableCategory,
    graph_link: GraphLink,
    focused_category: FocusedCategory,
    dragging: Option<Dragging>,
    main_selection: Option<RuleSelection>,
    goal_selection: Option<RuleSelection>,
    action_history: Vec<category::Action>, // TODO: move to [RenderableCategory]
}

impl GameState {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        let state = State::default();
        let fact_category = RenderableCategory::new(geng, assets, init::category::fact_category());
        let goal_category = RenderableCategory::new(geng, assets, init::category::goal_category());
        let rules = init::rules::default_rules(geng, assets)
            .into_iter()
            .map(|rule| RenderableRule {
                category: RenderableCategory::from_rule(geng, assets, &rule),
                inner: rule,
            })
            .collect();
        Self {
            geng: geng.clone(),
            dragging: None,
            main_selection: None,
            goal_selection: None,
            focused_category: FocusedCategory::Fact,
            action_history: vec![],
            ui_camera: PixelPerfectCamera,
            graph_link: GraphLink::new(&fact_category.inner, &goal_category.inner),
            fact_category,
            goal_category,
            rules,
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
