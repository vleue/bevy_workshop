# Progress Report

## What You've learned

* Loading a custom asset file
  * creating a custom asset by defining a struct deriving the [`Asset`](https://docs.rs/bevy/0.15.0-rc.3/bevy/asset/trait.Asset.html) trait
  * and implementing the [`AssetLoader`](https://docs.rs/bevy/0.15.0-rc.3/bevy/asset/trait.AssetLoader.html) trait to load a file into this struct
* Getting an asset
  * Using the [`Assets<T>`](https://docs.rs/bevy/0.15.0-rc.3/bevy/asset/struct.Assets.html) resource
* Hot reloading

## Going Further

The level format we've done is good for a quick game but is limited. You should either:
* Use an existing level editor, like [LDtk](https://ldtk.io) (Level Designer toolkit), which is very powerful and is supported in Bevy through third party plugins
* Or build your own, that will allow you to include specific features for your game
