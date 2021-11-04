use bevy::{app::AppExit, prelude::*};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MenuMaterials>()
            .add_startup_system(spawn_camera_system.system())
            .add_startup_system(spawn_menu_system.system())
            .add_system(button_system.system())
            .add_system(exit_game_button_system.system())
            .add_system(quit_on_escape_system.system());
    }
}

fn spawn_camera_system(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn spawn_menu_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_materials: Res<MenuMaterials>,
) {
    let root_container = create_root_container(&menu_materials);
    let buttons_container = create_buttons_container();
    let new_game_button = create_first_button(&menu_materials);
    let new_game_button_text = create_button_text(&asset_server, "New Game");
    let options_button = create_button(&menu_materials);
    let options_button_text = create_button_text(&asset_server, "Options");
    let exit_game_button = create_button(&menu_materials);
    let exit_game_button_text = create_button_text(&asset_server, "Exit Game");

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
                    parent.spawn_bundle(options_button).with_children(|parent| {
                        parent.spawn_bundle(options_button_text);
                    });
                    parent
                        .spawn_bundle(exit_game_button)
                        .insert(ExitGameButton)
                        .with_children(|parent| {
                            parent.spawn_bundle(exit_game_button_text);
                        });
                });
        });
}

fn create_root_container(menu_materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_grow: 1.0,
            flex_shrink: 0.0,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        material: menu_materials.background.clone(),
        ..Default::default()
    }
}

fn create_buttons_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            min_size: Size::new(Val::Px(250.0), Val::Auto),
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn create_first_button(menu_materials: &Res<MenuMaterials>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            flex_grow: 1.0,
            padding: Rect::all(Val::Px(5.0)),
            margin: Rect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: menu_materials.button_normal.clone(),
        ..Default::default()
    }
}

fn create_button(menu_materials: &Res<MenuMaterials>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            padding: Rect::all(Val::Px(5.0)),
            margin: Rect {
                left: Val::Px(5.0),
                right: Val::Px(5.0),
                top: Val::Px(0.0),
                bottom: Val::Px(5.0),
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: menu_materials.button_normal.clone(),
        ..Default::default()
    }
}

fn create_button_text(asset_server: &Res<AssetServer>, text: &str) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            text,
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

struct NewGameButton;
struct OptionsButton;
struct ExitGameButton;

struct MenuMaterials {
    background: Handle<ColorMaterial>,
    button_normal: Handle<ColorMaterial>,
    button_hovered: Handle<ColorMaterial>,
    button_pressed: Handle<ColorMaterial>,
}

impl FromWorld for MenuMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

        MenuMaterials {
            background: materials.add(Color::hex("082032").unwrap().into()),
            button_normal: materials.add(Color::hex("2C394B").unwrap().into()),
            button_hovered: materials.add(Color::hex("334756").unwrap().into()),
            button_pressed: materials.add(Color::hex("FF4C29").unwrap().into()),
        }
    }
}

fn button_system(
    menu_materials: Res<MenuMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        *material = match *interaction {
            Interaction::Clicked => menu_materials.button_pressed.clone(),
            Interaction::Hovered => menu_materials.button_hovered.clone(),
            Interaction::None => menu_materials.button_normal.clone(),
        }
    }
}

fn exit_game_button_system(
    mut interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<Button>,
            With<ExitGameButton>,
        ),
    >,
    mut writer: EventWriter<AppExit>,
) {
    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Clicked {
            writer.send(AppExit);
        }
    }
}

fn quit_on_escape_system(keyboard_input: Res<Input<KeyCode>>, mut writer: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        writer.send(AppExit);
    }
}
