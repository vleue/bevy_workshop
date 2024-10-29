# Systems and Schedules

It wouldn't be a splash screen if it didn't display anything. Let's show a title in that open window.

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
        // adding a system executed once on startup
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

The `Startup` schedule is for things that must happens only once during application startup.

Other common schedules to use are `PreUpdate`, `Update` and `PostUpdate`. They have their fixed counter parts `FixedPreUpdate`, `FixedUpdate` and `FixedPostUpdate`.

Systems in the `Update` schedule are executed every frame. When using vsync, this is driven by your screen, most common case is 60fps, macs are often on 120fps. Systems in the `FixedUpdate` schedule are executed at a configurable and fixed frequency, by default 64Hz. Most of your game logic should happen in those schedules.

`Pre*` and `Post*` schedules are useful to do preparation and cleanup/propagation around game logic.

## Systems

Systems are any function that takes (up to 16) parameters that are [`SystemParam`](https://docs.rs/bevy/0.15.0-rc.2/bevy/ecs/system/trait.SystemParam.html). They are provided at runtime through dependency injection based on their type.

If you want more details on how this works, you can find them here: [Dependency Injection like Bevy Engine from Scratch](https://promethia-27.github.io/dependency_injection_like_bevy_from_scratch/introductions.html)

## Commands

Commands are the main way to change the game world, either by adding, mutating or removing entities and components.

## Side note: UI

The startup system in the example above spawn text. It first spawn a node entity that is pretty much the equivalent of a `<div>` HTML tag, used to center the text, then as a children the text itself.

The two layout types available in Bevy for the UI are Flexbox and CSS Grids.
