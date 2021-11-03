use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
          .add_startup_system(spawn_camera.system())
          .add_startup_system(spawn_menu.system());
    }
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn_bundle(UiCameraBundle::default());
}

fn spawn_menu(
  mut commands: Commands,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  commands.spawn_bundle(NodeBundle {
    style: Style {
      size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
      ..Default::default()
    },
    material: materials.add(Color::rgb(0.65, 0.0, 0.0).into()),
    ..Default::default()
  });
}
