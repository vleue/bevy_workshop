# Visual Effects

Visual effects can help your game pop up. This is commonly done with shaders, which are programs that execute on the GPU. The best languages to write them in Bevy is the [WebGPU Shading Language](https://www.w3.org/TR/WGSL/), and it will be translated as needed by the platform on which the application is running.

Bevy offers several abstractions to render things on screen:
* Directly using images or colors or texture atlas, which is what we've been doing until now. The shaders are built-in Bevy, and use as many optimisation as possible at the cost of customisation.
* Custom materials, which we'll explore in this section. For 2d, you'll need to implement the [`Material2d`](https://docs.rs/bevy/0.15.0-rc.3/bevy/sprite/trait.Material2d.html) trait.
* Lower level abstractions, down to complete control on the whole rendering pipeline. This won't be in this workshop.
