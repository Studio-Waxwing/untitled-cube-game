pub mod plugins;

use bevy::{prelude::App, DefaultPlugins};
use plugins::ui::main_menu::MainMenuPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .run();
}
