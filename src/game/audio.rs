use bevy::prelude::*;

use super::AudioTrigger;

pub fn audio_plugin(app: &mut App) {
    app.add_event::<AudioTrigger>()
        .add_systems(Update, play_audio);
}

fn play_audio(
    mut commands: Commands,
    mut audio_triggers: EventReader<AudioTrigger>,
    sound_assets: Res<crate::AudioAssets>,
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
