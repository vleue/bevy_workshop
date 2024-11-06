use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use flag::FlagMaterial;

use crate::{
    level_loader::{Level, LoadedLevel, Tile},
    GameAssets, GameState,
};

mod audio;
mod flag;
mod player;

const SCALE: f32 = 0.5;

pub fn game_plugin(app: &mut App) {
    app.add_plugins((
        player::player_plugin,
        audio::audio_plugin,
        flag::flag_plugin,
    ))
    .add_systems(OnEnter(GameState::Game), display_level)
    .add_systems(
        Update,
        animate_level.run_if(on_timer(Duration::from_secs_f32(0.25))),
    );
}

#[derive(Component)]
#[require(IsOnGround, Velocity, AgainstWall)]
struct Player;

#[derive(Component, Default)]
struct IsOnGround(f32);

#[derive(Component, Default)]
struct AgainstWall(bool, bool);

#[derive(Component, Default)]
struct Velocity {
    current: f32,
    target: f32,
    jumping: f32,
}

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Flag;

#[derive(Event)]
struct ReachedFlag;

fn ground_tile_index(line: &[Tile], i: usize) -> usize {
    match (
        i == 0 || !matches!(line.get(i - 1).unwrap_or(&Tile::Empty), Tile::Ground),
        !matches!(line.get(i + 1).unwrap_or(&Tile::Empty), Tile::Ground),
    ) {
        (true, true) => 8,
        (true, false) => 14,
        (false, true) => 0,
        (false, false) => 7,
    }
}

fn display_tile(
    commands: &mut Commands,
    tile: &Tile,
    i: usize,
    x: f32,
    y: f32,
    line: &[Tile],
    assets: &GameAssets,
    meshes: &mut Assets<Mesh>,
    flag_materials: &mut Assets<FlagMaterial>,
) {
    match tile {
        Tile::Ground => {
            let index = ground_tile_index(line, i);
            commands.spawn((
                Sprite::from_atlas_image(
                    assets.ground_image.clone(),
                    TextureAtlas {
                        layout: assets.ground_layout.clone(),
                        index,
                    },
                ),
                Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(SCALE)),
                Ground,
                StateScoped(GameState::Game),
            ));
        }
        Tile::Spawn => {
            commands.spawn((
                Sprite::from_atlas_image(
                    assets.player_image.clone(),
                    TextureAtlas {
                        layout: assets.player_layout.clone(),
                        index: 0,
                    },
                ),
                Transform::from_xyz(x, y + 256.0 / 4.0 * SCALE, 2.0).with_scale(Vec3::splat(SCALE)),
                StateScoped(GameState::Game),
                Player,
            ));
        }
        Tile::Flag => {
            commands
                .spawn((
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(flag_materials.add(FlagMaterial {
                        atlas: assets.items_image.clone(),
                        index: Vec4::new(0.0, 1.0, 0.0, 0.0),
                        distance: Vec4::ZERO,
                    })),
                    Transform::from_xyz(x, y, 1.0).with_scale(Vec3::splat(SCALE) * 128.0),
                    StateScoped(GameState::Game),
                    Flag,
                ))
                .observe(reached_flag);
        }
        Tile::Empty => {}
    }
}

fn display_level(
    mut commands: Commands,
    assets: Res<GameAssets>,
    level: Res<LoadedLevel>,
    levels: Res<Assets<Level>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut flag_materials: ResMut<Assets<FlagMaterial>>,
) {
    let level = levels.get(&level.level).unwrap();

    for (j, line) in level.tiles.iter().enumerate() {
        for (i, tile) in line.iter().enumerate() {
            let (x, y) = (
                (i as f32 - 9.0) * 128.0 * SCALE,
                -(j as f32 - 5.0) * 128.0 * SCALE,
            );
            display_tile(
                &mut commands,
                tile,
                i,
                x,
                y,
                line,
                &assets,
                meshes.as_mut(),
                flag_materials.as_mut(),
            );
        }
    }
}

fn animate_level(
    flags: Query<&MeshMaterial2d<FlagMaterial>, With<Flag>>,
    mut flag_materials: ResMut<Assets<FlagMaterial>>,
) {
    for flag in &flags {
        let material = flag_materials.get_mut(flag).unwrap();
        if material.index.y == 1.0 {
            material.index.y = 2.0;
        } else {
            material.index.y = 1.0;
        }
    }
}

fn reached_flag(_trigger: Trigger<ReachedFlag>, mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::Menu);
}

#[derive(Event)]
enum AudioTrigger {
    Jump,
}
