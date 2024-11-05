# Progress Report

## What You've learned

* Application creation and adding Bevy default plugins
  * creating the [`App`](https://docs.rs/bevy/0.15.0-rc.3/bevy/app/struct.App.html) struct
  * and adding the [`DefaultPlugins`](https://docs.rs/bevy/0.15.0-rc.3/bevy/struct.DefaultPlugins.html)
* Schedules and adding systems
  * adding system with [`App::add_systems`](https://docs.rs/bevy/0.15.0-rc.3/bevy/app/struct.App.html#method.add_systems)
  * to a [`Schedule`](https://docs.rs/bevy/0.15.0-rc.3/bevy/ecs/prelude/struct.Schedule.html)
  * from the [list of schedules](https://docs.rs/bevy/0.15.0-rc.3/bevy/ecs/schedule/trait.ScheduleLabel.html#implementors)
* Basic use of commands and queries
  * the [`Commands`](https://docs.rs/bevy/0.15.0-rc.3/bevy/ecs/prelude/struct.Commands.html) queue
  * to issue a command from the [list of commands](https://docs.rs/bevy/0.15.0-rc.3/bevy/ecs/prelude/trait.Command.html#implementors)
  * and using a [`Query`](https://docs.rs/bevy/0.15.0-rc.3/bevy/ecs/prelude/struct.Query.html) to access components
* States, and running system only on a state or during state transition
  * using [`States`](https://docs.rs/bevy/0.15.0-rc.3/bevy/prelude/trait.States.html) trait
  * and the [`OnEnter`](https://docs.rs/bevy/0.15.0-rc.3/bevy/state/prelude/struct.OnEnter.html) state transition
  * with the [`NextState`](https://docs.rs/bevy/0.15.0-rc.3/bevy/prelude/enum.NextState.html) resource
* Code organization with plugins
  * the [`Plugin`](https://docs.rs/bevy/0.15.0-rc.3/bevy/app/trait.Plugin.html) trait
