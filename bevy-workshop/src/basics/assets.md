# Using Assets

Let's improve on those blocks of colors! We'll start by loading a spritesheet for the player

## Loading Assets

Loading assets is asynchronous, and returns an [`Handle`](https://docs.rs/bevy/0.15.0-rc.3/bevy/asset/enum.Handle.html) to its data. By adding a system to our splash screen, we ensure it happens as early as possible.

```rust,no_run
# extern crate bevy;
# use bevy::prelude::*;
#[derive(Resource)]
struct GameAssets {
    player_image: Handle<Image>,
    player_layout: Handle<TextureAtlasLayout>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.insert_resource(GameAssets {
        player_image: asset_server.load("spritesheet_players.png"),
        player_layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(128, 256),
            7,
            8,
            None,
            None,
        )),
    });
}
```

We're loading the spritesheet as a texture atlas, and saying each sprite is 128px by 256px, with 7 columns and 8 rows, no padding, no offset.

<div class="warning">

Don't forget to add the new `load_assets` system to the `splash_plugin`, when entering the `GameState::Splash` state.

</div>

## Displaying Those Assets

Now that we have a texture atlas, we can use it to display a sprite for our player instead of a block of red.

```rust,no_run
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Resource)]
# struct GameAssets {
#     player_image: Handle<Image>,
#     player_layout: Handle<TextureAtlasLayout>,
# }
# #[derive(Component)]
# struct Player;
# #[derive(Component)]
# struct Ground;
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Game }
fn display_level(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Sprite::from_atlas_image(
            assets.player_image.clone(),
            TextureAtlas {
                layout: assets.player_layout.clone(),
                index: 0,
            },
        ),
        Transform::from_scale(Vec3::splat(0.5)),
        Player,
        StateScoped(GameState::Game),
    ));

    commands.spawn((
        Sprite::from_color(Color::linear_rgb(0.0, 1.0, 0.0), Vec2::new(1000.0, 80.0)),
        Transform::from_xyz(0.0, -100.0, 0.0),
        Ground,
        StateScoped(GameState::Game),
    ));
}
```
