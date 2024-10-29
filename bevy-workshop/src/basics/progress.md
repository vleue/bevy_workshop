# Progress Report

## What You've learned

* Loading sprites and displaying them
  * [the `AssetServer::load` function](https://docs.rs/bevy/0.15.0-rc.2/bevy/asset/struct.AssetServer.html#method.load)
  * [the `Sprite` component](https://docs.rs/bevy/0.15.0-rc.2/bevy/prelude/struct.Sprite.html)
* Handling user input
  * [`ButtonInput<T>`](https://docs.rs/bevy/0.15.0-rc.2/bevy/input/struct.ButtonInput.html)
  * [`KeyCode`](https://docs.rs/bevy/0.15.0-rc.2/bevy/input/keyboard/enum.KeyCode.html)
* Writing more complex queries, and updating components
  * [`With` query filter](https://docs.rs/bevy/0.15.0-rc.2/bevy/ecs/prelude/struct.With.html)
  * [`&mut` to query data mutably](https://docs.rs/bevy/0.15.0-rc.2/bevy/ecs/change_detection/struct.Mut.html)

## Going Further

The "physics" engine we've done is *very* basic.

Movement is stopped after collisions happened, it would be better to do "shapecasting" on the AABB, find the point of collision, and stop there:
* Before moving, get the AABB
* Move it to where the player would be after moving
* If it collides with something, find the point where it would stop colliding between the two positions
* Move there instead of the original target

In real life, gravity is not a constant speed downward, it's an acceleration which would increase the speed every unit of time. It should be a force that's applied to the player and modifies its vertical speed.

Depending on the feel you want for your game, you can use a complete physics engine ([avian2d](https://crates.io/crates/avian2d) or [rapier2d](https://rapier.rs)) or build your own, tailored to what you want.
