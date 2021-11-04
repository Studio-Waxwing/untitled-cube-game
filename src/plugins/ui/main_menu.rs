use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_camera.system())
            .add_startup_system(spawn_menu.system());
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn spawn_menu(
    commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    root_container(commands, materials, asset_server);
}

fn root_container(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_grow: 1.0,
                flex_shrink: 0.0,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.65, 0.0, 0.0).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            buttons_container(materials, asset_server, parent);
        });
}

fn buttons_container(
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    parent: &mut ChildBuilder,
) {
    parent.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(450.0), Val::Px(450.0)),
            ..Default::default()
        },
        material: materials.add(Color::rgb(0.65, 0.65, 0.0).into()),
        ..Default::default()
    }).with_children(|parent| {
        new_game_button(materials, asset_server, parent);
    });
}

fn new_game_button(
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    parent: &mut ChildBuilder,
) {
    parent.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            // center button
            margin: Rect::all(Val::Auto),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: materials.add(Color::rgb(0.65, 0.0, 0.65).into()),
        ..Default::default()
    });
}
