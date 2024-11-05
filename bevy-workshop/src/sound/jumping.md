# Jumping

## Load an Audio Asset

We'll create a new resource to hold the handles to audio assets, and load it in the `load_assets`  system.

```rust
# extern crate bevy;
# use bevy::prelude::*;
#[derive(Resource)]
struct AudioAssets {
    jump: Handle<AudioSource>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // ...
) {
    commands.insert_resource(AudioAssets {
        jump: asset_server.load("jump.wav"),
    });
    // ...
}

```

The build-in type for audio is [`AudioSource`](https://docs.rs/bevy/0.15.0-rc.3/bevy/audio/struct.AudioSource.html).

## Trigger an Event to Play Audio

We'll trigger an event when we want to play audio. For now, that is when the player is starting to jump. To avoid triggering to many events, we should make sure that the player was not already jumping.

We'll start by declaring an event type:

```rust
# extern crate bevy;
# use bevy::prelude::*;
#[derive(Event)]
enum AudioTrigger {
    Jump,
}
```

To send an event, we can use the [`EventWriter`](https://docs.rs/bevy/0.15.0-rc.3/bevy/ecs/event/struct.EventWriter.html) system parameter:

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Event)]
# enum AudioTrigger {Jump}
# #[derive(Component)]
# struct Velocity {jumping: f32}
# #[derive(Component)]
# struct IsOnGround(f32);
# #[derive(Component)]
# struct Player;
fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Velocity, &IsOnGround), With<Player>>,
    time: Res<Time>,
    mut audio_triggers: EventWriter<AudioTrigger>,
) {
    // ...
#     let mut velocity = Velocity { jumping: 0.0 };
#     let is_on_ground = IsOnGround(0.0);
    if time.elapsed_secs() - is_on_ground.0 < 0.5 && keyboard_input.pressed(KeyCode::Space) {
        if velocity.jumping == 0.0 {
            audio_triggers.send(AudioTrigger::Jump);
        }
        velocity.jumping = 15.0;
    }
}
```

## Play Audio when Receiving the Event

To receive an event, we must use the [`EventReader`](https://docs.rs/bevy/0.15.0-rc.3/bevy/ecs/event/struct.EventReader.html) system parameter, and by calling [`EventReader::read`](https://docs.rs/bevy/0.15.0-rc.3/bevy/ecs/event/struct.EventReader.html#method.read) we can iterate over events.

To play audio, we must spawn an entity with the [`AudioPlayer`](https://docs.rs/bevy/0.15.0-rc.3/bevy/audio/struct.AudioPlayer.html) component that will contain an [`Handle`](https://docs.rs/bevy/0.15.0-rc.3/bevy/asset/enum.Handle.html) to the [`AudioSource`](https://docs.rs/bevy/0.15.0-rc.3/bevy/audio/struct.AudioSource.html) asset.

By default, audio entities remain present once the audio is done playing. You can change this behaviour with the component [`PlaybackSettings::DESPAWN`](https://docs.rs/bevy/0.15.0-rc.3/bevy/audio/struct.PlaybackSettings.html#associatedconstant.DESPAWN).

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Event)]
# enum AudioTrigger {Jump}
# #[derive(Resource)]
# struct AudioAssets { jump: Handle<AudioSource> }
fn play_audio(
    mut commands: Commands,
    mut audio_triggers: EventReader<AudioTrigger>,
    sound_assets: Res<AudioAssets>,
) {
    for trigger in audio_triggers.read() {
        match trigger {
            AudioTrigger::Jump => {
                commands.spawn((
                    AudioPlayer::<AudioSource>(sound_assets.jump.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            }
        }
    }
}
```

We'll start a new plugin for all the audio related actions. Unlike events used with triggers and observers, events used with `EventWriter` and `EventReader` must be registered in the application with [`App::add_event`](https://docs.rs/bevy/0.15.0-rc.3/bevy/app/struct.App.html#method.add_event). The plugin will register the event and add the system.

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Event)]
# enum AudioTrigger {Jump}
# fn play_audio() {}
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AudioTrigger>()
            .add_systems(Update, play_audio);
    }
}
```
