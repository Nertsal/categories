use category::prelude::*;
use category::RuleInput;
use geng::{Camera2d, PixelPerfectCamera};

use super::*;

mod apply_rule;
mod constants;
mod drag;
mod draw;
mod focus;
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

type Category = category::types::Category<Point, Arrow, Equality>;
type Morphism = category::types::Morphism<Arrow>;
type Object = category::types::Object<Point>;
type CategoryAction = category::Action<Point, Arrow, Equality>;
type Label = String;
type Rule = category::Rule<Label>;
type Bindings = category::Bindings<Label>;
type Constraints = category::Constraints<Label>;
type Equalities = category::Equalities<Equality>;

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
    fact_selection: Option<RuleSelection>,
    goal_selection: Option<RuleSelection>,
}

impl GameState {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        let state = State::default();
        let fact_category =
            RenderableCategory::new(geng, assets, init::category::fact_category(), true);
        let goal_category =
            RenderableCategory::new(geng, assets, init::category::goal_category(), true);
        let rules = init::rules::default_rules()
            .into_iter()
            .map(|rule| RenderableRule::from_rule(geng, assets, rule))
            .collect();
        Self {
            geng: geng.clone(),
            dragging: None,
            fact_selection: None,
            goal_selection: None,
            focused_category: FocusedCategory::Fact,
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
