# The Application

The initial goal is to open a window using Bevy!

## Empty Application

This is the most basic Bevy application. It will exit immediately upon running and perform no actions.

```rust
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new().run();
}
```

## Default Bevy Plugins

Default plugins are added to handle windowing, rendering, input, audio, and more. This application opens a window and then does nothing.

```rust,no_run
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
```

Plugins can be configured; in this example, we set a custom title for the window.

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
