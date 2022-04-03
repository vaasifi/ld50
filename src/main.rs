use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use game::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
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
struct FileFolderId(usize);

#[derive(Component)]
struct FileId(usize);

#[derive(Component)]
struct FolderId(usize);

#[derive(Component)]
struct FolderTag;
const FOLDER_DIMENSION: f32 = 100.0;

#[derive(Component)]
struct FolderFiles {
    indices: Vec<usize>,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_files(mut commands: Commands, asset_server: Res<AssetServer>) {
    let (id, files) = (0, vec!["FirstFile", "SecondFile", "ThirdFile"]);
    for (i, &file) in files.iter().enumerate() {
        let sprite = commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("square.png"),
                ..default()
            })
            .insert(FileTag)
            .insert(FileId(i))
            .insert(FileFolderId(id))
            .insert(Interaction::None)
            .id();

        let font = asset_server.load("LiberationSans-Regular.ttf");
        let text_style = TextStyle {
            font,
            font_size: 30.0,
            color: Color::WHITE,
        };

        let text_alignment = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        };

        let text = commands
            .spawn_bundle(Text2dBundle {
                transform: Transform {
                    translation: Vec3::Y * (-FILE_DIMENSION / 2.0),
                    ..default()
                },
                text: Text::with_section(file, text_style.clone(), text_alignment.clone()),
                ..default()
            })
            .id();

        commands.entity(sprite).push_children(&[text]);
    }

    let indices = (0..files.len()).collect();

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("folder.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 200.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(FolderTag)
        .insert(FolderFiles { indices })
        .insert(FolderId(id))
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
                *rect_transform.translation = *Vec2::extend(cursor_position, 0.0);
                // can clamp here later...
            }
        }
    }
}

fn folder_insert_system(
    mut commands: Commands,
    mut file_query: Query<
        (
            &mut Transform,
            &Interaction,
            &FileId,
            &FileFolderId,
            &FileTag,
            Entity,
        ),
        Without<FolderTag>,
    >,
    mut folder_query: Query<(&Transform, &mut FolderFiles, &FolderId), With<FolderTag>>,
) {
    for (
        mut file_transform,
        file_interaction,
        FileId(file_id),
        FileFolderId(file_folder_id),
        _,
        e,
    ) in file_query.iter_mut()
    {
        if let Interaction::None | Interaction::Hovered = file_interaction {
            for (folder_transform, mut folder_files, FolderId(folder_id)) in folder_query.iter_mut()
            {
                let collision = collide(
                    file_transform.translation,
                    Vec2::splat(FILE_DIMENSION),
                    folder_transform.translation,
                    Vec2::splat(FOLDER_DIMENSION),
                );

                if let Some(_) = collision {
                    if file_folder_id == folder_id {
                        if let Some(next_file_id) = folder_files.indices.first() {
                            if next_file_id == file_id {
                                folder_files.indices.remove(0);
                                commands.entity(e).despawn_recursive();
                            } else {
                                *file_transform.translation = *Vec3::ZERO;
                                println!("FUCK YOU WRONG ORDER");
                            }
                        }
                    } else {
                        *file_transform.translation = *Vec3::ZERO;
                        println!("FUCK YOU WRONG FOLDER");
                    }
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
