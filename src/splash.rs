use bevy::prelude::*;

use crate::{level_loader::LoadedLevel, GameAssets, GameState};

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), (display_title, load_assets))
        .add_systems(Update, switch_to_menu.run_if(in_state(GameState::Splash)));
}

fn display_title(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            StateScoped(GameState::Splash),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Bevy\nWorkshop"),
                TextFont {
                    font_size: 130.0,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });

    commands.insert_resource(SplashScreenTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

#[derive(Resource)]
struct SplashScreenTimer(Timer);

fn switch_to_menu(
    mut next: ResMut<NextState<GameState>>,
    mut timer: ResMut<SplashScreenTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        next.set(GameState::Menu);
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.insert_resource(LoadedLevel {
        level: asset_server.load("level.bw"),
    });
    commands.insert_resource(GameAssets {
        player_image: asset_server.load("spritesheet_players.png"),
        player_layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(128, 256),
            7,
            8,
            None,
            None,
        )),
        ground_image: asset_server.load("spritesheet_ground.png"),
        ground_layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(128, 128),
            7,
            8,
            None,
            None,
        )),
    });
}
