# States

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
        .add_systems(OnEnter(GameState::Splash), display_title)
        .add_systems(Update, switch_to_menu.run_if(in_state(GameState::Splash)))
        .run();
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    #[default]
    Splash,
    Menu,
}

#[derive(Resource)]
struct SplashScreenTimer(Timer);

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

fn switch_to_menu(
    mut next: ResMut<NextState<GameState>>,
    mut timer: ResMut<SplashScreenTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        next.set(GameState::Menu);
    }
}
```

## Schedules Based on State

Additional schedules are available when using state: `OnEnter`, `OnExit` and `OnTransition`.

## Changing State

Using a resource `NextState`.

## State Scopped Entities

By adding a component `StateScoped`, all entities and their hierarchy marked by this component will be despawned when exiting the state.
