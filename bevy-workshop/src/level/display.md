# Displaying the Level

Loading an asset is an asynchronous process. As it involves file or network access, it doesn't happen immediately. This is why the asset server is returning an [`Handle`](https://docs.rs/bevy/0.15.0-rc.2/bevy/asset/enum.Handle.html) instead of the data.

Accessing the data from the [`Assets<T>`](https://docs.rs/bevy/0.15.0-rc.2/bevy/asset/struct.Assets.html) resource returns an `Option<T>` as the data may not be present yet. In our case, we're using the 2 second delay of the splash screen to be sure that assets are done loading, so we can `unwrap()` the `Option`.

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Asset, TypePath)]
# struct Level {pub tiles: Vec<Vec<Tile>>}
# enum Tile {Empty, Ground}
# #[derive(Resource)]
# struct GameAssets {
#     player_image: Handle<Image>,
#     player_layout: Handle<TextureAtlasLayout>,
#     ground_image: Handle<Image>,
#     ground_layout: Handle<TextureAtlasLayout>,
# }
# #[derive(Resource)]
# pub struct LoadedLevel { pub level: Handle<Level> }
# #[derive(Component)]
# struct Player;
# #[derive(Component)]
# struct Ground;
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Game }
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
                Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(0.5)),
                Ground,
                StateScoped(GameState::Game),
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
            let (x, y) = ((i as f32 - 9.0) * 64.0, -(j as f32 - 5.0) * 64.0);
            display_tile(&mut commands, tile, i, x, y, line, &assets);
        }
    }

    commands.spawn((
        Sprite::from_atlas_image(
            assets.player_image.clone(),
            TextureAtlas {
                layout: assets.player_layout.clone(),
                index: 0,
            },
        ),
        Transform::from_xyz(0.0, 200.0, 0.0).with_scale(Vec3::splat(0.5)),
        StateScoped(GameState::Game),
        Player,
    ));
}
```
