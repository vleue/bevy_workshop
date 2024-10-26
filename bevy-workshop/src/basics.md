# Basic Game

At the end of this section, you'll be able to move around the player, and lose the game.

## Displaying Something

```rust,no_run
# extern crate bevy;
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Game }
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), display_level);
    }
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

Don't forget to add the new `GamePlugin` to the app in the `main.rs` file.

### Tag Components

Helper component to query for specific entities.

## Controlling With Input

```rust,no_run
# extern crate bevy;
# #[derive(Component)]
# struct Player;
# use bevy::prelude::*;
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

## Loading Assets

```rust,no_run
# extern crate bevy;
#[derive(Resource)]
struct GameAssets {
    player_image: Handle<Image>,
    player_layout: Handle<TextureAtlasLayout>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.insert_resource(GameAssets {
        player_image: asset_server.load("spritesheet_players.png"),
        player_layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(128, 256),
            7,
            8,
            None,
            None,
        )),
    });
}
```

## Displaying Those Assets

```rust,no_run
# extern crate bevy;
# #[derive(Resource)]
# struct GameAssets {
#     player_image: Handle<Image>,
#     player_layout: Handle<TextureAtlasLayout>,
# }
fn display_level(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Sprite::from_atlas_image(
            assets.player_image.clone(),
            TextureAtlas {
                layout: assets.player_layout.clone(),
                index: 0,
            },
        ),
        Transform::from_scale(Vec3::splat(0.5)),
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

## Exercises

<div class="warning">

Your turn!

</div>

### Displaying the Ground

File `spritesheet_ground.png` has a spritesheet for the ground.

You can use different sprites for the borders.

Tips:
* Add new fields to the `GameAssets` resource for the ground
* The index is `column * number_of_columns + line`

### Player Sprite Animation

It should be flipped when changing direction.

There are two sprites available in the spritesheet to display an animation when walking.

Tips:
* The [`Sprite` component](https://docs.rs/bevy/0.15.0-rc.1/bevy/prelude/struct.Sprite.html) has a `flip_x` field
* You can use a local step counter (adding a parameter `mut steps: Local<u32>`) in system `control_player` and changing every 10 steps if the player is moving
* The `Sprite` component has a `texture_atlas` field that can be used to change the index

### Player Acceleration

Make the player acceletare and decelerate instead of directly changing it's position when a key is pressed.

Tips:
* Store the player current velocity and target velocity in a new component
* When a key is pressed, change the target velocity accordingly
* In a separate system, change the current velocity towards the target velocity
* Move the player according to its current velocity

## Basic Physics

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

### Disjoint Queries

Accessing the `Transform` component both mutably and immutably in the same system is not possible, unless we can prove the two queries are disjoint. The filters do this.

## Exercises

<div class="warning">

Your turn!

</div>

### Jumping

You can change sprite when jumping.

Can you jump only when touching ground? Do you want double jump? The jump height to vary depending on the duration of the button press? Can the player control direction during jump? Those will change the feel of your game.

[Youtube: Why Does Celeste Feel So Good to Play?](https://www.youtube.com/watch?v=yorTG9at90g)

Tips:
* Split gravity into two systems, one checking if player is on the ground and updating a dedicated component, one handling falling when not on the ground
    * You'll need to keep the "falling" system running *after* the "on_ground" system
* Add a new component `JumpSpeed(f32)`. When the player jumps, set the value to something higher than gravity (`10.0` in the example above). While the player is jumping, decrease this value until it reaches `0.0`
* If you did the "Player Acceleration" exercise, they can share component and systems

### Falling

Make the game go back to the menu when the player fall offscreen.

Tips:
* Check if the position of the player is below a certain threshold. Then change state to go back to the menu.

## What You've learned

* Loading sprites and displaying them
* Handling user input
* Writing more complex queries, and updating components

## Going Further

The "physics" engine we've done is *very* basic.

Movement is stopped after collisions happened, it would be better to do "shapecasting" on the AABB, find the point of collision, and stop there:
* Before moving, get the AABB
* Move it to where the player would be after moving
* If it collides with something, find the point where it would stop colliding between the two positions
* Move there instead of the original target

Gravity is not a constant speed downward, it's an acceleration which would increase the speed every unit of time.

Depending on the feel you want for your game, you can use a complete physics engine ([avian2d](https://crates.io/crates/avian2d) or [rapier2d](https://rapier.rs)) or build your own, tailored to what you want.
