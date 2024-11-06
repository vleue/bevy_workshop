# Basic Physics

It wouldn't be a platformer if you can't fall. Let's add some "gravity" to the game.

We'll compute an Axis Aligned Bounding Box (or AABB) for the player and the ground, and check for intersections. If there's one, then the player is on the ground. Otherwise, the player is in the air, then move its position down to simulate falling.

```rust,no_run
# extern crate bevy;
# #[derive(Component)]
# struct Player;
# #[derive(Component)]
# struct Ground;
# use bevy::{
#     math::bounding::{Aabb2d, IntersectsVolume},
#     prelude::*,
# };
fn gravity(
    mut player: Query<&mut Transform, With<Player>>,
    ground: Query<&Transform, (Without<Player>, With<Ground>)>,
) {
    let mut is_on_ground = false;
    let mut player_transform = player.single_mut();

    let player_aabb = Aabb2d::new(
        Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y,
        ),
        Vec2::new(
            128.0 * player_transform.scale.x,
            256.0 * player_transform.scale.y,
        ) / 2.0,
    );

    for ground_transform in &ground {
        let ground_aabb = Aabb2d::new(
            Vec2::new(
                ground_transform.translation.x,
                ground_transform.translation.y,
            ),
            Vec2::new(
                128.0 * ground_transform.scale.x,
                128.0 * ground_transform.scale.y,
            ) / 2.0,
        );

        if ground_aabb.intersects(&player_aabb) {
            is_on_ground = true;
        }
    }
    if !is_on_ground {
        player_transform.translation.y -= 10.0;
    }
}
```

<div class="warning">

Don't forget to add the new `gravity` system to the `game_plugin`, on `FixedUpdate` in the `GameState::Game` state.

</div>

## Disjoint Queries

Accessing the `Transform` component both mutably and immutably in the same system is not possible, unless we can prove the two queries are disjoint. The filters do this.
