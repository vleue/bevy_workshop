# Custom Asset Format

## Level Format: The Quick and Dirty Way

Let's go with a basic format that you can manually edit with a good idea of how it should render: emojis to the rescue!

```level
⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜🙂⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜🟩🟩🟩🟩🟩🟩🟩🟩🟩⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
```

## Asset Type

To match the basic level format, we'll use a basic type that will just be a vec of vecs of tiles. The struct must derive the [`Asset`](https://docs.rs/bevy/0.15.0-rc.3/bevy/asset/trait.Asset.html) trait.

```rust
# extern crate bevy;
# use bevy::prelude::*;
#[derive(Asset, TypePath)]
struct Level {
    pub tiles: Vec<Vec<Tile>>,
}

enum Tile {
    Empty,
    Ground,
}
```

## Asset Loader

To load this format, we'll read the file character by character, then choose the right tile depending on the character. Bevy expects custom asset loader to implement the trait [`AssetLoader`](https://docs.rs/bevy/0.15.0-rc.3/bevy/asset/trait.AssetLoader.html).

```rust
# extern crate bevy;
# extern crate thiserror;
# use bevy::{asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext}, prelude::*};
# use thiserror::Error;
# #[derive(Asset, TypePath)]
# struct Level {pub tiles: Vec<Vec<Tile>>}
# enum Tile {Empty, Ground}
#[derive(Default)]
struct LevelLoader;

#[derive(Debug, Error)]
enum LevelLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unknown tile: {0}")]
    UnknownTile(char),
}

impl AssetLoader for LevelLoader {
    type Asset = Level;
    type Settings = ();
    type Error = LevelLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf).await?;

        let mut tiles = vec![];
        let mut line = vec![];
        for char in buf.chars() {
            match char {
                '⬜' => line.push(Tile::Empty),
                '🟩' => line.push(Tile::Ground),
                '🙂' => (),
                '\n' => {
                    tiles.push(line);
                    line = vec![];
                }
                char => Err(LevelLoaderError::UnknownTile(char))?,
            }
        }
        Ok(Level { tiles })
    }

    fn extensions(&self) -> &[&str] {
        &["bw"]
    }
}
```

## Loading the Level

Custom asset formats and loaders must be initiated in the application with [`App::init_asset`](https://docs.rs/bevy/0.15.0-rc.3/bevy/app/struct.App.html#method.init_asset) and [`App::init_asset_loader`](https://docs.rs/bevy/0.15.0-rc.3/bevy/asset/trait.AssetApp.html#tymethod.init_asset_loader). We can wrap that in a plugin.

```rust
# extern crate bevy;
# extern crate thiserror;
# use bevy::{asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext}, prelude::*};
# use thiserror::Error;
# #[derive(Asset, TypePath)]
# struct Level {pub tiles: Vec<Vec<Tile>>}
# enum Tile {Empty, Ground}
# #[derive(Default)]
# struct LevelLoader;
# #[derive(Debug, Error)]
# enum LevelLoaderError {}
# impl AssetLoader for LevelLoader {
#     type Asset = Level;
#     type Settings = ();
#     type Error = LevelLoaderError;
#     async fn load(&self, reader: &mut dyn Reader, _settings: &(), _load_context: &mut LoadContext<'_>) -> Result<Self::Asset, Self::Error> { unimplemented!() }
#     fn extensions(&self) -> &[&str] { &["bw"] }
# }
fn level_loader_plugin(app: &mut App) {
    app.init_asset::<Level>().init_asset_loader::<LevelLoader>();
}
```

<div class="warning">

Don't forget to add the new `level_loader_plugin` to the app in the `main.rs` file.

</div>

Now we can load the asset file like the sprites we're already using, and keeping the handle to the loaded level in a resource.

```rust
# extern crate bevy;
# use bevy::{asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext}, prelude::*};
# #[derive(Asset, TypePath)]
# struct Level {pub tiles: Vec<Vec<Tile>>}
# enum Tile {Empty, Ground}
#[derive(Resource)]
pub struct LoadedLevel {
    pub level: Handle<Level>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // ...
) {
    commands.insert_resource(LoadedLevel {
        level: asset_server.load("level.bw"),
    });
    // ...
}

```
