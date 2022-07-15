mod actions;
mod agent;
mod camera;
mod loading;
mod menu;
mod windows;
mod zone;

use crate::actions::ActionsPlugin;
use crate::agent::AgentPlugin;
use crate::camera::CameraPlugin;
use crate::menu::MenuPlugin;
use crate::windows::UiPlugin;
use crate::zone::ZonePlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::plugin::ShapePlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(UiPlugin)
            .add_plugin(loading::LoadingPlugin)
            .add_plugin(AgentPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(EguiPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(ShapePlugin)
            .add_plugin(ZonePlugin)
            .add_plugin(CameraPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
