mod a_b_tests;
mod color_picker;
mod draw_neighborhoods;
mod map_edits;
mod road_editor;
mod scenarios;
mod stop_sign_editor;
mod traffic_signal_editor;

use crate::objects::{Ctx, ID};
use crate::plugins::{Plugin, PluginCtx};
use ezgui::{Color, GfxCtx};
use map_model::IntersectionID;

pub struct EditMode {
    active_plugin: Option<Box<Plugin>>,
}

impl EditMode {
    pub fn new() -> EditMode {
        EditMode {
            active_plugin: None,
        }
    }

    pub fn show_turn_icons(&self, id: IntersectionID) -> bool {
        if let Some(p) = self
            .active_plugin
            .as_ref()
            .and_then(|p| p.downcast_ref::<stop_sign_editor::StopSignEditor>().ok())
        {
            return p.show_turn_icons(id);
        }
        if let Some(p) = self.active_plugin.as_ref().and_then(|p| {
            p.downcast_ref::<traffic_signal_editor::TrafficSignalEditor>()
                .ok()
        }) {
            return p.show_turn_icons(id);
        }
        false
    }
}

impl Plugin for EditMode {
    fn blocking_event(&mut self, ctx: &mut PluginCtx) -> bool {
        if self.active_plugin.is_some() {
            if self.active_plugin.as_mut().unwrap().blocking_event(ctx) {
                return true;
            } else {
                self.active_plugin = None;
                return false;
            }
        }

        // TODO Something higher-level should not even invoke EditMode while we're in A/B test
        // mode.
        if ctx.secondary.is_some() {
            return false;
        }

        if let Some(p) = a_b_tests::ABTestManager::new(ctx) {
            self.active_plugin = Some(Box::new(p));
        } else if let Some(p) = color_picker::ColorPicker::new(ctx) {
            self.active_plugin = Some(Box::new(p));
        } else if let Some(p) = draw_neighborhoods::DrawNeighborhoodState::new(ctx) {
            self.active_plugin = Some(Box::new(p));
        } else if let Some(p) = map_edits::EditsManager::new(ctx) {
            self.active_plugin = Some(Box::new(p));
        } else if let Some(p) = road_editor::RoadEditor::new(ctx) {
            self.active_plugin = Some(Box::new(p));
        } else if let Some(p) = scenarios::ScenarioManager::new(ctx) {
            self.active_plugin = Some(Box::new(p));
        } else if let Some(p) = stop_sign_editor::StopSignEditor::new(ctx) {
            self.active_plugin = Some(Box::new(p));
        } else if let Some(p) = traffic_signal_editor::TrafficSignalEditor::new(ctx) {
            self.active_plugin = Some(Box::new(p));
        }

        self.active_plugin.is_some()
    }

    fn draw(&self, g: &mut GfxCtx, ctx: &mut Ctx) {
        if let Some(ref plugin) = self.active_plugin {
            plugin.draw(g, ctx);
        }
    }

    fn color_for(&self, obj: ID, ctx: &mut Ctx) -> Option<Color> {
        if let Some(ref plugin) = self.active_plugin {
            return plugin.color_for(obj, ctx);
        }
        None
    }
}