# Progress Report

## What You've learned

* Loading sprites and displaying them
  * with the [`AssetServer::load`](https://docs.rs/bevy/0.15.0-rc.2/bevy/asset/struct.AssetServer.html#method.load) function
  * by adding the [`Sprite`](https://docs.rs/bevy/0.15.0-rc.2/bevy/prelude/struct.Sprite.html) component
* Defining components
  * With [required components](https://docs.rs/bevy/0.15.0-rc.2/bevy/ecs/component/trait.Component.html#required-components) to simplify adding related components
  * And using Zero Sized Types as tag components to filter entities in queries
* Handling user input
  * reading the [`ButtonInput<T>`](https://docs.rs/bevy/0.15.0-rc.2/bevy/input/struct.ButtonInput.html) resource
  * for the input [`KeyCode`](https://docs.rs/bevy/0.15.0-rc.2/bevy/input/keyboard/enum.KeyCode.html)
* Writing more complex queries, and updating components
  * the [`With`](https://docs.rs/bevy/0.15.0-rc.2/bevy/ecs/prelude/struct.With.html) and [`Without`](https://docs.rs/bevy/0.15.0-rc.2/bevy/ecs/prelude/struct.Without.html) query filters
  * and using [`&mut`](https://docs.rs/bevy/0.15.0-rc.2/bevy/ecs/change_detection/struct.Mut.html) to query data mutably

## Going Further

The "physics" engine we've done is *very* basic.

Movement is stopped after collisions happened, it would be better to do "shapecasting" on the AABB, find the point of collision, and stop there:
* Before moving, get the AABB
* Move it to where the player would be after moving
* If it collides with something, find the point where it would stop colliding between the two positions
* Move there instead of the original target

In real life, gravity is not a constant speed downward, it's an acceleration which would increase the speed every unit of time. It should be a force that's applied to the player and modifies its vertical speed.

Depending on the feel you want for your game, you can use a complete physics engine ([avian2d](https://crates.io/crates/avian2d) or [rapier2d](https://rapier.rs)) or build your own, tailored to what you want.
