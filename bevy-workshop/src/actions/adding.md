# Adding a Winning Zone

## Add a Winning Zone to the Level

By adding a new emoji to our level, we can add something new. Let's add a winning zone with a 🏁 emoji.

First we'll add a new variant to the `Tile` enum: `Flag`:

```rust
pub enum Tile {
    // ...
    Flag,
}
```

Then parse it in our `LevelLoader`:

```rust,edition2021
# extern crate bevy;
# use bevy::{asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext}, prelude::*};
# #[derive(Asset, TypePath)]
# struct Level {pub tiles: Vec<Vec<Tile>>}
# enum LevelLoaderError {UnknownTile(char)}
# enum Tile {Flag}
# struct LevelLoader;
# trait ShortLoader {
#     type Error;
#     type Asset;
#     async fn load() -> Result<Self::Asset, Self::Error>;
# }
# impl ShortLoader for LevelLoader {
# type Error = LevelLoaderError;
# type Asset = Level;
async fn load(/* ... */) -> Result<Self::Asset, Self::Error> {
#     let buf = String::new();
#     let mut tiles = vec![];
#     let mut line = vec![];
    // ...
    for char in buf.chars() {
        match char {
            // ...
            '🏁' => line.push(Tile::Flag),
            char => Err(LevelLoaderError::UnknownTile(char))?,
        }
    }
    Ok(Level { tiles })
}
# }
```

## Displaying the Zone

We'll use a new spritesheet, `spritesheet_items.png`, to have items to display.

First we'll add new fields to the `GameAssets` resource to hold the new handles:

```rust
# extern crate bevy;
# use bevy::prelude::*;
#[derive(Resource)]
struct GameAssets {
    // ...
    items_image: Handle<Image>,
    items_layout: Handle<TextureAtlasLayout>,
}
```

Then load the new spritesheet during the splash screen:

```rust,no_run
# extern crate bevy;
# use bevy::prelude::*;
# let commands: Commands = unimplemented!();
# let texture_atlas_layouts: Assets<TextureAtlasLayout> = unimplemented!();
# let asset_server: AssetServer = unimplemented!();
# #[derive(Resource)]
# struct GameAssets {
#     items_image: Handle<Image>,
#     items_layout: Handle<TextureAtlasLayout>,
# }
commands.insert_resource(GameAssets {
    // ...
    items_image: asset_server.load("spritesheet_items.png"),
    items_layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(128, 128),
        6,
        4,
        None,
        None,
    )),
});
```

And finally we'll display the flag in `display_tile`:

```rust
# extern crate bevy;
# use bevy::prelude::*;
# struct GameAssets {
#     items_image: Handle<Image>,
#     items_layout: Handle<TextureAtlasLayout>,
# }
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Game }
# enum Tile { Flag }
#[derive(Component)]
struct Flag;

fn display_tile(/* ... */) {
    # let assets: GameAssets = unimplemented!();
    # let commands: Commands = unimplemented!();
    # let (x, y) = (0.0, 0.0);
    # let tile = Tile::Flag;
    match tile {
        // ...
        Tile::Flag => {
            commands.spawn((
                Sprite::from_atlas_image(
                    assets.items_image.clone(),
                    TextureAtlas {
                        layout: assets.items_layout.clone(),
                        index: 6,
                    },
                ),
                Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(0.5)),
                StateScoped(GameState::Game),
                Flag,
            ));
        }
    }
}
```

## Z-Index

If you play a few times, you may notice that the order of sprites varies: sometimes the alien is in front of the flag, sometimes behind. This can be controlled with the `Z` index.

Everything we've displayed up till now, we've used `O.O` for the `z` value when calling [`Transform::from_xyz`](https://docs.rs/bevy/0.15.0-rc.2/bevy/transform/components/struct.Transform.html#method.from_xyz). By using the same value for every sprite, we're not telling the engine the order they should be displayed. In Bevy, higher values are displayed in front.
