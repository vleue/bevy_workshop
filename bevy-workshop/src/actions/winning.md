# Winning the Game

We'll send an event when the player gets near the flag, then react to it to win the game.

We'll need a system to trigger the event and another to react to it.

## Declare an event

First we'll declare an event that we can trigger when the player is near the flag.

```rust
# extern crate bevy;
# use bevy::prelude::*;
#[derive(Event)]
struct ReachedFlag;
```

## Triggers

They can be global with [`Commands::trigger`](https://docs.rs/bevy/0.15.0-rc.3/bevy/ecs/prelude/struct.Commands.html#method.trigger) and [`App::add_observer`](https://docs.rs/bevy/0.15.0-rc.3/bevy/app/struct.App.html#method.add_observer), or specific to an entity with [`EntityCommands::trigger`](https://docs.rs/bevy/0.15.0-rc.3/bevy/ecs/prelude/struct.EntityCommands.html#method.trigger) and [`EntityCommands::observe`](https://docs.rs/bevy/0.15.0-rc.3/bevy/ecs/prelude/struct.EntityCommands.html#method.observe).

Here is the entity specific version to trigger the event:

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Event)]
# struct ReachedFlag;
# #[derive(Component)]
# struct Player;
# #[derive(Component)]
# struct Flag;
fn near_flag(
    mut commands: Commands,
    player_transform: Query<&Transform, With<Player>>,
    flags: Query<(Entity, &Transform), With<Flag>>,
) {
    let player_transform = player_transform.single();
    for (flag, flag_transform) in &flags {
        if player_transform
            .translation
            .distance(flag_transform.translation)
            < 50.0
        {
            commands.entity(flag).trigger(ReachedFlag);
        }
    }
}
```

The `near_flag` system is added to the `player_plugin`:

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Game }
# fn near_flag(){}
fn player_plugin(app: &mut App) {
    // ...
    app.add_systems(FixedUpdate, near_flag.run_if(in_state(GameState::Game)));
}
```

## Observers

To react to the trigger, we use a system that takes a [`Trigger`](https://docs.rs/bevy/0.15.0-rc.3/bevy/ecs/observer/struct.Trigger.html) as a system parameter, plus any other parameter needed.

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Event)]
# struct ReachedFlag;
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Menu }
fn reached_flag(_trigger: Trigger<ReachedFlag>, mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::Menu);
}
```

And the `reached_flag` observer is added to the `Flag` entity:

```rust
# extern crate bevy;
# use bevy::prelude::*;
# enum Tile { Flag }
# #[derive(Component)]
# struct Flag;
# #[derive(Event)]
# struct ReachedFlag;
# fn reached_flag(_trigger: Trigger<ReachedFlag>) {}
fn display_tile(/* ... */) {
    # let commands: Commands = unimplemented!();
    # let (x, y) = (0.0, 0.0);
    # let tile = Tile::Flag;
    match tile {
        // ...
        Tile::Flag => {
            commands
                .spawn((
                    // ...
                    Flag,
                ))
                .observe(reached_flag);
        }
    }
}
```
