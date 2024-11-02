# Systems and Schedules

A splash screen needs to display something, so let's show a title in the open window.

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
        // add a system that executes once at startup
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

## Schedules

The `Startup` schedule is used for tasks that need to occur only once during application startup.

Other common schedules include `PreUpdate`, `Update`, and `PostUpdate`, along with their fixed counterparts: `FixedPreUpdate`, `FixedUpdate`, and `FixedPostUpdate`.

Systems in the `Update` schedule execute every frame. With vsync enabled, this is typically driven by your screen's refresh rate, commonly 60fps, with some Macs running at 120fps. Systems in the `FixedUpdate` schedule execute at a configurable, fixed frequency, defaulting to 64Hz. Most game logic should occur within these schedules.

`Pre*` and `Post*` schedules are useful for preparation and cleanup/propagation tasks surrounding game logic.

## Systems

Systems are functions that accept up to 16 parameters, each of which must implement the [`SystemParam`](https://docs.rs/bevy/0.15.0-rc.2/bevy/ecs/system/trait.SystemParam.html) trait. These parameters are provided at runtime through dependency injection based on their type.

If you want more details on how this works, you can find them here: [Dependency Injection like Bevy Engine from Scratch](https://promethia-27.github.io/dependency_injection_like_bevy_from_scratch/introductions.html)

## Commands

Commands are the primary means of modifying the game world, allowing you to add, mutate, or remove entities and components. They are not executed straight away, but at sync points between systems.

## Side note: UI

The startup system in the example above spawns text. It first spawns a node entity, which functions similarly to a `<div>` HTML tag, used to center the text, and then spawns the text itself as a child.

Bevy offers two layout types for UI: Flexbox and CSS Grids.
