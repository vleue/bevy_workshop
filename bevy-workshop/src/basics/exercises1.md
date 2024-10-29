# Exercises

## Displaying the Ground

File `spritesheet_ground.png` has a spritesheet for the ground.

You can use different sprites for the borders.

Tips:
* Add new fields to the `GameAssets` resource for the ground
* The index is `column * number_of_columns + line`

## Player Sprite Animation

It should be flipped when changing direction.

There are two sprites available in the spritesheet to display an animation when walking.

Tips:
* The [`Sprite` component](https://docs.rs/bevy/0.15.0-rc.2/bevy/prelude/struct.Sprite.html) has a `flip_x` field
* You can use a local step counter (adding a parameter `mut steps: Local<u32>`) in system `control_player` and changing every 10 steps if the player is moving
* The `Sprite` component has a `texture_atlas` field that can be used to change the index

## Player Acceleration

Make the player acceletare and decelerate instead of directly changing it's position when a key is pressed.

Tips:
* Store the player current velocity and target velocity in a new component
* When a key is pressed, change the target velocity accordingly
* In a separate system, change the current velocity towards the target velocity
* Move the player according to its current velocity
