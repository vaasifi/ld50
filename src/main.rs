use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::text::{Text2dBounds, Text2dSize};
use game::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_folders)
        .add_startup_system(spawn_files)
        .add_startup_system(cursor_lock_settings)
        .add_system(move_grabbed_system)
        .add_system(grab_system)
        .add_system(folder_insert_system)
        .run();
}

#[derive(Component)]
struct FileTag;
const FILE_DIMENSION: f32 = 100.0;

#[derive(Component)]
struct FileName(String);

#[derive(Component)]
struct FolderTag;
const FOLDER_DIMENSION: f32 = 200.0;

#[derive(Bundle)]
struct FileBundle {
    
    #[bundle]
    sprite: SpriteBundle,
    #[bundle]
    text: Text2dBundle,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_files(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("LiberationSans-Regular.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("square.png"),
            ..default()
        })
        .insert(FileTag)
        .insert(Interaction::None)
        .insert(FileName(String::from("Duude")));

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("square.png"),
            transform: Transform {
                translation: Vec3::new(50.0, 50.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(FileTag)
        .insert(Interaction::None)
        .insert(Text::with_section(
            "Duude".to_string(),
            text_style.clone(),
            Default::default(),
        ))
        .insert(Text2dBounds { ..default() })
        .insert(Text2dSize { ..default() });
    /*commands.spawn_bundle(Text2dBundle {
        text: Text::with_section("Duuude", text_style.clone(), Default::default()),
        ..default()
    });*/
}

fn spawn_folders(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("folder.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 300.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(FolderTag)
        .insert(Interaction::None);
}

fn grab_system(
    mut query: Query<(&Transform, &mut Interaction), With<FileTag>>,
    windows: Res<Windows>,
    button: Res<Input<MouseButton>>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(absolute_cursor_position) = window.cursor_position() {
        let cursor_position: Vec2 =
            relative_cursor_position(absolute_cursor_position, window.width(), window.height());

        for (rect_transform, mut interaction) in query.iter_mut() {
            let rect_position: Vec2 = Vec3::truncate(rect_transform.translation);
            let rect_size: Vec2 = Vec2::splat(FILE_DIMENSION);
            if cursor_collision(cursor_position, rect_position, rect_size) {
                if button.just_pressed(MouseButton::Left) {
                    *interaction = Interaction::Clicked;
                    return;
                } else if !button.pressed(MouseButton::Left) {
                    *interaction = Interaction::Hovered;
                }
            } else if !button.pressed(MouseButton::Left) {
                *interaction = Interaction::None;
            }
        }
    }
}

fn move_grabbed_system(
    mut query: Query<(&mut Transform, &Interaction), With<FileTag>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(absolute_cursor_position) = window.cursor_position() {
        let cursor_position: Vec2 =
            relative_cursor_position(absolute_cursor_position, window.width(), window.height());
        for (mut rect_transform, interaction) in query.iter_mut() {
            if let Interaction::Clicked = interaction {
                println!("{:?}", rect_transform.translation);
                *rect_transform.translation = *Vec2::extend(cursor_position, 0.0);
                // can clamp here later...
            }
        }
    }
}

fn folder_insert_system(
    mut commands: Commands,
    file_query: Query<(&Transform, &Interaction, Entity), With<FileTag>>,
    folder_query: Query<&Transform, With<FolderTag>>,
) {
    for (file_transform, file_interaction, e) in file_query.iter() {
        if let Interaction::None | Interaction::Hovered = file_interaction {
            for folder_transform in folder_query.iter() {
                let collision = collide(
                    file_transform.translation,
                    Vec2::splat(FILE_DIMENSION),
                    folder_transform.translation,
                    Vec2::splat(FOLDER_DIMENSION),
                );
                if let Some(_) = collision {
                    //insert logic here
                    commands.entity(e).despawn_recursive();
                }
            }
        }
    }
}

fn cursor_lock_settings(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(false);
    window.set_cursor_visibility(true);
}
