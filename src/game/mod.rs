use bevy::prelude::*;

use crate::{GameAssets, GameState};

mod player;

pub fn game_plugin(app: &mut App) {
    app.add_plugins(player::player_plugin)
        .add_systems(OnEnter(GameState::Game), display_level);
}

#[derive(Component)]
struct Player;

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
        StateScoped(GameState::Game),
        Player,
    ));

    commands.spawn((
        Sprite::from_color(Color::linear_rgb(0.0, 1.0, 0.0), Vec2::new(1000.0, 80.0)),
        Transform::from_xyz(0.0, -100.0, 0.0),
        Ground,
        StateScoped(GameState::Game),
    ));
}
