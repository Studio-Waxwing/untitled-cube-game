use bevy::{app::AppExit, prelude::*};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_startup_system(spawn_camera_system.system())
            .add_startup_system(spawn_menu_system.system())
            .add_system(button_system.system())
            .add_system(quit_on_escape_system.system());
    }
}

fn spawn_camera_system(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn spawn_menu_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let root_container = create_root_container(&mut materials);
    let buttons_container = create_buttons_container(&mut materials);
    let new_game_button = create_button(&mut materials);
    let new_game_button_text = create_button_text(&asset_server);
    let exit_game_button = create_button(&mut materials);
    let exit_game_button_text = create_button_text(&asset_server);

    commands
        .spawn_bundle(root_container)
        .with_children(|parent| {
            parent
                .spawn_bundle(buttons_container)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(new_game_button)
                        .with_children(|parent| {
                            parent.spawn_bundle(new_game_button_text);
                        });
                    parent
                        .spawn_bundle(exit_game_button)
                        .with_children(|parent| {
                            parent.spawn_bundle(exit_game_button_text);
                        });
                });
        });
}

fn create_root_container(materials: &mut ResMut<Assets<ColorMaterial>>) -> NodeBundle {
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

fn create_buttons_container(materials: &mut ResMut<Assets<ColorMaterial>>) -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        material: materials.add(Color::rgb(0.65, 0.65, 0.0).into()),
        ..Default::default()
    }
}

fn create_button(materials: &mut ResMut<Assets<ColorMaterial>>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            margin: Rect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: materials.add(Color::rgb(0.65, 0.0, 0.65).into()),
        ..Default::default()
    }
}

fn create_button_text(asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            "Button",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
            Default::default(),
        ),
        ..Default::default()
    }
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn quit_on_escape_system(keyboard_input: Res<Input<KeyCode>>, mut writer: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        writer.send(AppExit);
    }
}
