# The Application

For now, the goal is to open a window!

## Empty Application

The most basic application. This will exit immediately when running, and do nothing.

```rust
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new().run();
}
```

## Default Bevy Plugins

Default plugins are added to handle windowing, rendering, input, audio, ... are added. This application opens a windows then does nothing.

```rust,no_run
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
```

Plugins can be configured, in this case to give a name to the window.

```rust,no_run
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Workshop".into(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
```
