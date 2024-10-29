# Progress Report

## What You've learned

* Application creation and adding Bevy default plugins
  * [the `App` struct](https://docs.rs/bevy/0.15.0-rc.1/bevy/app/struct.App.html)
  * [the `DefaultPlugins`](https://docs.rs/bevy/0.15.0-rc.1/bevy/struct.DefaultPlugins.html)
* Schedules and adding systems
  * [`Schedule`](https://docs.rs/bevy/0.15.0-rc.1/bevy/ecs/prelude/struct.Schedule.html)
  * [List of schedules](https://docs.rs/bevy/0.15.0-rc.1/bevy/ecs/schedule/trait.ScheduleLabel.html#implementors)
  * [`App::add_systems`](https://docs.rs/bevy/0.15.0-rc.1/bevy/app/struct.App.html#method.add_systems)
* Basic use of commands and queries
  * [the `Commands` queue](https://docs.rs/bevy/0.15.0-rc.1/bevy/ecs/prelude/struct.Commands.html)
  * [List of commands](https://docs.rs/bevy/0.15.0-rc.1/bevy/ecs/prelude/trait.Command.html#implementors)
  * [`Query`](https://docs.rs/bevy/0.15.0-rc.1/bevy/ecs/prelude/struct.Query.html)
* States, and running system only on a state or during state transition
  * [the `States` trait](https://docs.rs/bevy/0.15.0-rc.1/bevy/prelude/trait.States.html)
  * [`OnEnter` state transition](https://docs.rs/bevy/0.15.0-rc.1/bevy/state/prelude/struct.OnEnter.html)
  * [`NextState` resource](https://docs.rs/bevy/0.15.0-rc.1/bevy/prelude/enum.NextState.html)
* Code organization with plugins
  * [the `Plugin` trait](https://docs.rs/bevy/0.15.0-rc.1/bevy/app/trait.Plugin.html)
