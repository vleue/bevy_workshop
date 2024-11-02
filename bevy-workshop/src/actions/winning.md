# Winning the Game

We'll send an event when the player gets near the flag, then react to it to win the game.

There are two ways to work with events, we'll explore both and continue with the second:
* General Events
* Triggers and Observers

In both case, we'll need a system to trigger the event and another to react to it.

## Declare an event

First we'll declare an event that we can trigger when the player is near the flag.

```rust
# extern crate bevy;
# use bevy::prelude::*;
#[derive(Event)]
struct ReachedFlag;
```

## General Events

General events need to be registered on the application with [`App::add_event`](https://docs.rs/bevy/0.15.0-rc.2/bevy/app/struct.App.html#method.add_event).

Add to the `GamePlugin`:

```rust
# extern crate bevy;
# use bevy::prelude::*;
# let mut app = App::new();
# #[derive(Event)]
# struct ReachedFlag;
app.add_event::<ReachedFlag>();
```

To trigger the event, we can use the [`EventWriter`](https://docs.rs/bevy/0.15.0-rc.2/bevy/ecs/event/struct.EventWriter.html) system parameter:

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
    player_transform: Query<&Transform, With<Player>>,
    flags: Query<&Transform, With<Flag>>,
    mut reached_flag_events: EventWriter<ReachedFlag>,
) {
    let player_transform = player_transform.single();
    for flag_transform in &flags {
        if player_transform
            .translation
            .distance(flag_transform.translation)
            < 50.0
        {
            reached_flag_events.send(ReachedFlag);
        }
    }
}
```

And then use [`EventReader`](https://docs.rs/bevy/0.15.0-rc.2/bevy/ecs/event/struct.EventReader.html) to check for new events:

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Event)]
# struct ReachedFlag;
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Menu }
fn reached_flag(mut events: EventReader<ReachedFlag>, mut next: ResMut<NextState<GameState>>) {
    for _ in events.read() {
        next.set(GameState::Menu);
    }
}
```

Finally, add those systems to the app in the plugins:

```rust
# extern crate bevy;
# use bevy::prelude::*;
# let mut app = App::new();
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Game }
# #[derive(Event)]
# struct ReachedFlag;
# fn near_flag(){}
# fn reached_flag(){}
# struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // ...
        app.add_systems(FixedUpdate, near_flag.run_if(in_state(GameState::Game)));
    }
}

# struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // ...
        app.add_systems(Update, reached_flag);
    }
}
```

It's possible to run the system only if an event has been triggered:

```rust
# extern crate bevy;
# use bevy::prelude::*;
# let mut app = App::new();
# #[derive(Event)]
# struct ReachedFlag;
# fn reached_flag(){}
# struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // ...
        app.add_systems(Update, reached_flag.run_if(on_event::<ReachedFlag>));
    }
}
```

## Triggers and Observers

Triggered events don't need to be registered.

They can be global with [`Commands::trigger`](https://docs.rs/bevy/0.15.0-rc.2/bevy/ecs/prelude/struct.Commands.html#method.trigger) and [`App::add_observer`](https://docs.rs/bevy/0.15.0-rc.2/bevy/app/struct.App.html#method.add_observer), or specific to an entity with [`EntityCommands::trigger`](https://docs.rs/bevy/0.15.0-rc.2/bevy/ecs/prelude/struct.EntityCommands.html#method.trigger) and [`EntityCommands::observe`](https://docs.rs/bevy/0.15.0-rc.2/bevy/ecs/prelude/struct.EntityCommands.html#method.observe).

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

And to react to the trigger:

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

The `near_flag` system is added to the `PlayerPlugin`:

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Game }
# fn near_flag(){}
# struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // ...
        app.add_systems(FixedUpdate, near_flag.run_if(in_state(GameState::Game)));
    }
}
```

And the `reached_flag` observer is added to the `Flag` entity:

```rust
# extern crate bevy;
# use bevy::prelude::*;
# enum Tile { End }
# #[derive(Component)]
# struct Flag;
# #[derive(Event)]
# struct ReachedFlag;
# fn reached_flag(_trigger: Trigger<ReachedFlag>) {}
fn display_tile(/* ... */) {
    # let commands: Commands = unimplemented!();
    # let (x, y) = (0.0, 0.0);
    # let tile = Tile::End;
    match tile {
        // ...
        Tile::End => {
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
