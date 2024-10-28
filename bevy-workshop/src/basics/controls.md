# Controlling With Input

```rust,no_run
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Component)]
# struct Player;
fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = player.single_mut();
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        player_transform.translation.x -= 5.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        player_transform.translation.x += 5.0;
    }
}
```
