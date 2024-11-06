# Plugins

Plugins are used for code organization, often in their own files.

```rust,no_run
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Workshop".into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_plugins(splash::SplashPlugin)           // adding our new plugin
        .run();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    #[default]
    Splash,
    Menu,
}

mod splash {
    use bevy::prelude::*;

    use crate::GameState;

    pub struct SplashPlugin;

    impl Plugin for SplashPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::Splash), display_title)
                .add_systems(Update, switch_to_menu.run_if(in_state(GameState::Splash)));
        }
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
}
```

For plugins that don't use any configuration, it's possible to expose the build function directly, and use it as a plugin:

```rust,no_run
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState {
#     #[default]
#     Splash,
# }
fn main() {
    App::new()
        // ...
        .add_plugins(splash::splash_plugin)           // adding our new plugin
        .run();
}

mod splash {
    # use bevy::prelude::*;
    # use crate::GameState;
    # fn display_title() {}
    # fn load_assets() {}
    # fn switch_to_menu() {}
    pub fn splash_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), (display_title, load_assets))
            .add_systems(Update, switch_to_menu.run_if(in_state(GameState::Splash)));
    }
}
```