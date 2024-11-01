use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
};
use thiserror::Error;

pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Level>().init_asset_loader::<LevelLoader>();
    }
}

#[derive(Resource)]
pub struct LoadedLevel {
    pub level: Handle<Level>,
}

#[derive(Asset, TypePath, Debug)]
pub struct Level {
    pub tiles: Vec<Vec<Tile>>,
}

#[derive(Debug)]
pub enum Tile {
    Empty,
    Ground,
    Start,
    End,
}

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
                '🏁' => line.push(Tile::End),
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
