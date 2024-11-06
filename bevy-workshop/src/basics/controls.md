# Controlling With Input

We'll control our player with the `A` and `D` keys on the keyboard. `A` changes the position of the player to the left, and `D` to the right.

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
    if keyboard_input.pressed(KeyCode::KeyA) {
        player_transform.translation.x -= 5.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        player_transform.translation.x += 5.0;
    }
}
```

<div class="warning">

Don't forget to add the new `control_player` system to the `game_plugin`, on `FixedUpdate` in the `GameState::Game` state.

</div>
