# Displaying Something

We'll just display block of colors for now, as a placeholder. Red is the player, green is the ground.

```rust,no_run
# extern crate bevy;
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Game }
use bevy::prelude::*;

fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), display_level);
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ground;

fn display_level(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(Color::linear_rgb(1.0, 0.0, 0.0), Vec2::new(50.0, 80.0)),
        Player,
        StateScoped(GameState::Game),
    ));

    commands.spawn((
        Sprite::from_color(Color::linear_rgb(0.0, 1.0, 0.0), Vec2::new(1000.0, 80.0)),
        Transform::from_xyz(0.0, -100.0, 0.0),
        Ground,
        StateScoped(GameState::Game),
    ));
}
```

<div class="warning">

Don't forget to add the new `game_plugin` to the app in the `main.rs` file.

</div>

## First Custom Component

A component is a Rust type, a struct or an enum, that implement the `Component` trait. It can be derived.

## Tag Components

Tag components, or markers, are Zero Sized Types (ZST) used to mark an entity for easier query. Zero Sized Types are types that have only one value possible, and offers optimisations in Rust.

To differentiate between the ground and the player entities, we could use an enum:

```rust
# extern crate bevy;
# use bevy::prelude::*;
#[derive(Component)]
enum Kind {
    Player,
    Ground
}
```

And query that component. That would mean the same query would return both the ground and the player entities, and we would have to filter based on the value of the component.

By using tag components, the query will return only the entity for the player or the ground but not both.

Which is better will depend on your specific use case, the number of entities, how often you need to iterate over, ...

## Required Components

We've spawned two entities with the `Sprite` component, to display a block of color, but only one with the `Transform` component, to position it on screen.

Even though it's not specified, the player entity will also have a `Transform` component, which will be added with the default value.

This is because `Transform` is a [required component](https://docs.rs/bevy/0.15.0-rc.3/bevy/ecs/component/trait.Component.html#required-components) of `Sprite`.

Required components are specified by an attribute when deriving `Component`, and should implement `Default` (or specify a constructor in the attribute).

```rust
# extern crate bevy;
# use bevy::prelude::*;
#[derive(Component)]
#[require(Transform)]
pub struct Sprite {
    /// The sprite's color tint
    pub color: Color,
    // ...
}

#[derive(Component, Default)]
pub struct Transform {
    /// Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.
    pub translation: Vec3,
    // ...
}

```
