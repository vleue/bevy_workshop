use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{
    level_loader::{Level, LoadedLevel, Tile},
    GameAssets, GameState,
};

mod player;

const SCALE: f32 = 0.5;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::PlayerPlugin)
            .add_systems(OnEnter(GameState::Game), display_level)
            .add_systems(
                Update,
                animate_level.run_if(on_timer(Duration::from_secs_f32(0.25))),
            );
    }
}

#[derive(Component)]
#[require(IsOnGround, Velocity, AgainstWall)]
struct Player;

#[derive(Component, Default)]
struct IsOnGround(bool);

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
        Tile::Start => {
            commands.spawn((
                Sprite::from_atlas_image(
                    assets.player_image.clone(),
                    TextureAtlas {
                        layout: assets.player_layout.clone(),
                        index: 0,
                    },
                ),
                Transform::from_xyz(x, y + 256.0 / 4.0 * SCALE, 0.0).with_scale(Vec3::splat(SCALE)),
                StateScoped(GameState::Game),
                Player,
            ));
        }
        Tile::End => {
            commands.spawn((
                Sprite::from_atlas_image(
                    assets.items_image.clone(),
                    TextureAtlas {
                        layout: assets.items_layout.clone(),
                        index: 6,
                    },
                ),
                Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(SCALE)),
                StateScoped(GameState::Game),
                Flag,
            ));
        }
        Tile::Empty => {}
    }
}

fn display_level(
    mut commands: Commands,
    assets: Res<GameAssets>,
    level: Res<LoadedLevel>,
    levels: Res<Assets<Level>>,
) {
    let level = levels.get(&level.level).unwrap();

    for (j, line) in level.tiles.iter().enumerate() {
        for (i, tile) in line.iter().enumerate() {
            let (x, y) = (
                (i as f32 - 9.0) * 128.0 * SCALE,
                -(j as f32 - 5.0) * 128.0 * SCALE,
            );
            display_tile(&mut commands, tile, i, x, y, line, &assets);
        }
    }
}

fn animate_level(mut flags: Query<&mut Sprite, With<Flag>>) {
    for mut flag in &mut flags {
        let atlas = flag.texture_atlas.as_mut().unwrap();
        if atlas.index == 6 {
            atlas.index = 12;
        } else {
            atlas.index = 6;
        }
    }
}
