# Custom Asset Format

## Asset Format

### Use an Existing Level Editor

For a real project, I strongly recommend [LDtk](https://ldtk.io) (Level Designer toolkit).

### Build Your Own Level Editor

If you have some custom needs, or want complete control, you can build your own level editor that would output the level is some parsable format, for example:

```json
{
    "platforms": [
        {
            "start": 3,
            "end": 7,
            "height": 2
        }
    ],
    "start": [4, 3],
}
```

### The Quick and Dirty Way

LDtk support is not built in, and we won't have time to build a custom editor. Let's go with a basic format that you can manually edit with a good idea of how it should render: emojis to the rescue!

```level





⬜⬜⬜⬜⬜⬜🙂
⬜⬜⬜⬜⬜🟩🟩🟩🟩🟩🟩🟩🟩🟩
```

## Asset Type

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
    Start,
}
```

## Asset Loader

```rust,edition2021
# extern crate bevy;
# extern crate thiserror;
# use bevy::{asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext}, prelude::*};
# use thiserror::Error;
# #[derive(Asset, TypePath)]
# struct Level {pub tiles: Vec<Vec<Tile>>}
# enum Tile {Empty, Ground, Start}
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
                '🙂' => line.push(Tile::Start),
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
