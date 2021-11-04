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
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let root_container = root_container(&mut materials);
    let buttons_container = buttons_container(&mut materials);
    let new_game_button = new_game_button(&mut materials);

    commands
        .spawn_bundle(root_container)
        .with_children(|parent| {
            parent
                .spawn_bundle(buttons_container)
                .with_children(|parent| {
                    parent.spawn_bundle(new_game_button);
                });
        });
}

fn root_container(materials: &mut ResMut<Assets<ColorMaterial>>) -> NodeBundle {
    NodeBundle {
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
    }
}

fn buttons_container(materials: &mut ResMut<Assets<ColorMaterial>>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(450.0), Val::Px(450.0)),
            ..Default::default()
        },
        material: materials.add(Color::rgb(0.65, 0.65, 0.0).into()),
        ..Default::default()
    }
}

fn new_game_button(materials: &mut ResMut<Assets<ColorMaterial>>) -> ButtonBundle {
    ButtonBundle {
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
    }
}
