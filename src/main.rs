use bevy::prelude::{App, IntoSystem};

fn main() {
    App::build()
        .add_system(hello_world.system())
        .run();
}

fn hello_world() {
    println!("Hello, world!");
}
