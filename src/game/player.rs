use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::GameState;

use super::{AgainstWall, Ground, IsOnGround, Player, Velocity};

pub fn player_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            control_player,
            on_ground,
            moving,
            player_animation,
            death_by_fall,
            gravity.after(on_ground),
        )
            .run_if(in_state(GameState::Game)),
    );
}

fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Velocity, &IsOnGround), With<Player>>,
    time: Res<Time>,
) {
    let (mut velocity, is_on_ground) = player.single_mut();
    if time.elapsed_secs() - is_on_ground.0 < 2.0 || velocity.jumping > 0.0 {
        if keyboard_input.pressed(KeyCode::KeyA) {
            velocity.target = -5.0;
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            velocity.target = 5.0;
        } else {
            velocity.target = 0.0;
        }
    }
    if time.elapsed_secs() - is_on_ground.0 < 0.5 && keyboard_input.pressed(KeyCode::Space) {
        velocity.jumping = 15.0;
    }
}

fn on_ground(
    mut player: Query<(&Transform, &mut IsOnGround, &mut AgainstWall), With<Player>>,
    ground: Query<&Transform, (Without<Player>, With<Ground>)>,
    time: Res<Time>,
) {
    let mut is_on_ground = false;
    let mut is_against_wall = (false, false);
    let (player_transform, mut player_on_ground, mut player_against_wall) = player.single_mut();

    let player_aabb = Aabb2d::new(
        Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y - 128.0 / 4.0,
        ),
        Vec2::new(
            128.0 * player_transform.scale.x,
            (256.0 * 5.0 / 8.0) * player_transform.scale.y,
        ) / 2.0
            * 0.8,
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
            if ground_transform.translation.y
                > player_transform.translation.y - 256.0 / 4.0 * player_transform.scale.y - 2.0
            {
                if ground_transform.translation.x < player_transform.translation.x {
                    is_against_wall.0 = true;
                } else {
                    is_against_wall.1 = true;
                }
            } else {
                is_on_ground = true;
            }
        }
    }
    if is_on_ground {
        player_on_ground.0 = time.elapsed_secs();
    }
    if is_against_wall.0 != player_against_wall.0 {
        player_against_wall.0 = is_against_wall.0;
    }
    if is_against_wall.1 != player_against_wall.1 {
        player_against_wall.1 = is_against_wall.1;
    }
}

fn gravity(mut player: Query<(&mut Transform, &IsOnGround), With<Player>>, time: Res<Time>) {
    let (mut player_transform, player_on_ground) = player.single_mut();

    if time.elapsed_secs() - player_on_ground.0 > 0.1 {
        player_transform.translation.y -= 10.0;
    }
}

fn moving(mut player: Query<(&mut Transform, &mut Velocity, &AgainstWall), With<Player>>) {
    let (mut player_transform, mut velocity, against_wall) = player.single_mut();

    if velocity.jumping > 0.0 {
        player_transform.translation.y += velocity.jumping;
        velocity.jumping -= 0.5;
    }

    if velocity.current != 0.0 {
        if against_wall.0 && velocity.current < 0.0 {
            velocity.current = 0.0;
        }
        if against_wall.1 && velocity.current > 0.0 {
            velocity.current = 0.0;
        }
        player_transform.translation.x += velocity.current;
    }
    if velocity.current != velocity.target {
        velocity.current += (velocity.target - velocity.current) / 10.0;
        if velocity.current.abs() < 0.1 {
            velocity.current = 0.0;
        }
    }
}

fn player_animation(
    mut player: Query<(&mut Sprite, &Velocity), Changed<Transform>>,
    mut steps: Local<u32>,
) {
    if let Ok((mut sprite, velocity)) = player.get_single_mut() {
        if velocity.jumping > 0.0 {
            sprite.texture_atlas.as_mut().unwrap().index = 35;
        } else {
            *steps += 1;
            if *steps % 10 == 0 {
                sprite.texture_atlas.as_mut().unwrap().index =
                    if sprite.texture_atlas.as_ref().unwrap().index == 0 {
                        7
                    } else {
                        0
                    };
            }
        }
        if velocity.current < 0.0 {
            sprite.flip_x = true;
        } else if velocity.current > 0.0 {
            sprite.flip_x = false;
        }
    }
}

fn death_by_fall(
    mut next: ResMut<NextState<GameState>>,
    player_transform: Query<&Transform, With<Player>>,
) {
    let player_transform = player_transform.single();
    if player_transform.translation.y < -400.0 {
        next.set(GameState::Menu);
    }
}
