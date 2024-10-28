# Next

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

In real life, gravity is not a constant speed downward, it's an acceleration which would increase the speed every unit of time. It should be a force that's applied to the player and modifies its vertical speed.

Depending on the feel you want for your game, you can use a complete physics engine ([avian2d](https://crates.io/crates/avian2d) or [rapier2d](https://rapier.rs)) or build your own, tailored to what you want.
