# Exercises

## Jumping

You can change sprite when jumping.

Can you jump only when touching ground? Do you want double jump? The jump height to vary depending on the duration of the button press? Can the player control direction during jump? Those will change the feel of your game.

[Youtube: Why Does Celeste Feel So Good to Play?](https://www.youtube.com/watch?v=yorTG9at90g)

Tips:
* Split gravity into two systems, one checking if player is on the ground and updating a dedicated component, one handling falling when not on the ground
    * You'll need to keep the "falling" system running *after* the "on_ground" system
* Add a new component `JumpSpeed(f32)`. When the player jumps, set the value to something higher than gravity (`10.0` in the example above). While the player is jumping, decrease this value until it reaches `0.0`
* If you did the "Player Acceleration" exercise, they can share component and systems

## Falling

Make the game go back to the menu when the player fall offscreen.

Tips:
* Check if the position of the player is below a certain threshold. Then change state to go back to the menu.
