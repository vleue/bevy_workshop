use bevy::prelude::*;

use crate::{GameAssets, GameState};

mod player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::PlayerPlugin)
            .add_systems(OnEnter(GameState::Game), display_level);
    }
}

#[derive(Component)]
#[require(IsOnGround, Velocity)]
struct Player;

#[derive(Component, Default)]
struct IsOnGround(bool);

#[derive(Component, Default)]
struct Velocity {
    current: f32,
    target: f32,
    jumping: f32,
}

#[derive(Component)]
struct Ground;

fn display_level(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Sprite::from_atlas_image(
            assets.player_image.clone(),
            TextureAtlas {
                layout: assets.player_layout.clone(),
                index: 0,
            },
        ),
        Transform::from_xyz(0.0, -32.0, 0.0).with_scale(Vec3::splat(0.5)),
        StateScoped(GameState::Game),
        Player,
    ));

    let half_width = 4;
    commands.spawn((
        Sprite::from_atlas_image(
            assets.ground_image.clone(),
            TextureAtlas {
                layout: assets.ground_layout.clone(),
                index: 14,
            },
        ),
        Transform::from_xyz(-half_width as f32 * 64.0, -128.0, 0.0).with_scale(Vec3::splat(0.5)),
        StateScoped(GameState::Game),
        Ground,
    ));
    for i in (-half_width + 1)..half_width {
        commands.spawn((
            Sprite::from_atlas_image(
                assets.ground_image.clone(),
                TextureAtlas {
                    layout: assets.ground_layout.clone(),
                    index: 7,
                },
            ),
            Transform::from_xyz(i as f32 * 64.0, -128.0, 0.0).with_scale(Vec3::splat(0.5)),
            Ground,
            StateScoped(GameState::Game),
        ));
    }
    commands.spawn((
        Sprite::from_atlas_image(
            assets.ground_image.clone(),
            TextureAtlas {
                layout: assets.ground_layout.clone(),
                index: 0,
            },
        ),
        Transform::from_xyz(half_width as f32 * 64.0, -128.0, 0.0).with_scale(Vec3::splat(0.5)),
        Ground,
        StateScoped(GameState::Game),
    ));
}
