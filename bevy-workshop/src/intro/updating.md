# Updating the World

Another characteristic of splash screen is that they don't stay forever. Let's remove the title after two seconds.

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

#[derive(Resource)]
struct SplashScreenTimer(Timer);

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

## Resources

Used to store singleton in the world, based on their type. See [`Resource`](https://docs.rs/bevy/0.15.0-rc.2/bevy/ecs/prelude/trait.Resource.html).

## Queries

Used to access entities and their components in the world. Can be filtered. See [`Query`](https://docs.rs/bevy/0.15.0-rc.2/bevy/ecs/prelude/struct.Query.html).

## Mutable VS Immutable Access

The `remove_title` example access two resources, `Time` in a non mutable way, and `SplashScreenTimer` in a mutable way.

As the world continue to hold ownership of data, systems have access to references. Only one system accessing a given part of the world mutably can run at a single time. Systems that access different part mutable, or the same part immutably can run in parallel.
