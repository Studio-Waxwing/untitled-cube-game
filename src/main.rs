pub mod plugins;

use bevy::{prelude::App, DefaultPlugins};
use plugins::ui::{ui_example_1::UiExample1Plugin, ui_example_2::UiExample2Plugin};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(UiExample1Plugin)
        .run();
}
