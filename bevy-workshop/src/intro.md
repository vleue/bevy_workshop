# Introduction to Bevy

We'll start our game with a splash screen.

## The Application

The most basic application. This will exit immediately when running, and do nothing.

```rust
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new().run();
}
```

## Default Bevy Plugins

Default plugins are added to handle windowing, rendering, input, audio, ... are added. This application opens a windows then does nothing.

```rust,no_run
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
```

Plugins can be configured, in this case to give a name to the window.

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
        .run();
}
```

## Systems and Schedules

Display a title in the open window.

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
        .add_systems(Startup, display_title)
        .run();
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
}
```

### Schedules

The `Startup` schedule is for things that must happens only once during application startup.

Other common schedules to use are `PreUpdate`, `Update` and `PostUpdate`. They have their fixed counter parts `FixedPreUpdate`, `FixedUpdate` and `FixedPostUpdate`.

Systems in the `Update` schedule are executed every frame. When using vsync, this is driven by your screen, most common case is 60fps, macs are often on 120fps. Systems in the `FixedUpdate` schedule are executed at a configurable and fixed frequency, by default 64Hz. Most of your game logic should happen in those schedules.

`Pre*` and `Post*` schedules are useful to do preparation and cleanup/propagation around game logic.

### Systems

Systems are any function that takes (up to 16) parameters that are [`SystemParam`](https://docs.rs/bevy/0.15.0-rc.1/bevy/ecs/system/trait.SystemParam.html). They are provided at runtime through dependency injection based on their type.

If you want more details on how this works, you can find them here: [Dependency Injection like Bevy Engine from Scratch](https://promethia-27.github.io/dependency_injection_like_bevy_from_scratch/introductions.html)

### Commands

Commands are the main way to change the game world, either by adding, mutating or removing entities and components.

### Side note: UI

The startup system in the example above spawn text. It first spawn a node entity that is pretty much the equivalent of a `<div>` HTML tag, used to center the text, then as a children the text itself.

The two layout types available in Bevy for the UI are Flexbox and CSS Grids.

## Updating the World

Remove the title after two seconds.

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
        .add_systems(Startup, display_title)
        .add_systems(Update, remove_title)
        .run();
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

fn remove_title(
    time: Res<Time>,
    mut timer: ResMut<SplashScreenTimer>,
    mut commands: Commands,
    nodes: Query<Entity, With<Node>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for entity in &nodes {
            commands.entity(entity).despawn();
        }
    }
}
```

### Resources

Used to store singleton in the world, based on their type. See [`Resource`](https://docs.rs/bevy/0.15.0-rc.1/bevy/ecs/prelude/trait.Resource.html).

### Queries

Used to access entities and their components in the world. Can be filtered. See [`Query`](https://docs.rs/bevy/0.15.0-rc.1/bevy/ecs/prelude/struct.Query.html).

### Mutable VS Immutable Access

The `remove_title` example access two resources, `Time` in a non mutable way, and `SplashScreenTimer` in a mutable way.

As the world continue to hold ownership of data, systems have access to references. Only one system accessing a given part of the world mutably can run at a single time. Systems that access different part mutable, or the same part immutably can run in parallel.

## States

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

### Schedules Based on State

Additional schedules are available when using state: `OnEnter`, `OnExit` and `OnTransition`.

### Changing State

Using a resource `NextState`.

### State Scopped Entities

By adding a component `StateScoped`, all entities and their hierarchy marked by this component will be despawned when exiting the state.

## Plugins

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
        .add_plugins(splash::SplashPlugin)
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

## What You've learned

* Application creation and adding Bevy default plugins
* Schedules and adding systems
* Basic use of commands and queries
* States, and running system only on a state or during state transition
* Code organization with plugins
