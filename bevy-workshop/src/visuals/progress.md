# Progress Report

## What You've learned

* Defining a custom material
  * With the [`AsBindGroup`](https://docs.rs/bevy/0.15.0-rc.3/bevy/render/render_resource/trait.AsBindGroup.html) derive and its attributes to handle data transfer to the GPU
  * Implementing the [`Material2d`](https://docs.rs/bevy/0.15.0-rc.3/bevy/sprite/trait.Material2d.html) trait to define the shader
  * And some basic WGSL
* And using that material
  * Adding it to the app with the [`Material2dPlugin`](https://docs.rs/bevy/0.15.0-rc.3/bevy/sprite/struct.Material2dPlugin.html)
  * With the [`Mesh2d`](https://docs.rs/bevy/0.15.0-rc.3/bevy/prelude/struct.Mesh2d.html) component to define the shape
  * And the [`MeshMaterial2d`](https://docs.rs/bevy/0.15.0-rc.3/bevy/prelude/struct.MeshMaterial2d.html) component to define the material

## Going Further

Shaders and rendering is a *very big* domain.
