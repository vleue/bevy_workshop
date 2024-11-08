use bevy::prelude::*;

mod game;
mod level_loader;
mod menu;
mod splash;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Workshop".into(),
                canvas: Some("#game".into()),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_plugins((
            splash::splash_plugin,
            level_loader::level_loader_plugin,
            menu::menu_plugin,
            game::game_plugin,
        ))
        .run();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Resource)]
struct GameAssets {
    player_image: Handle<Image>,
    player_layout: Handle<TextureAtlasLayout>,
    ground_image: Handle<Image>,
    ground_layout: Handle<TextureAtlasLayout>,
}
