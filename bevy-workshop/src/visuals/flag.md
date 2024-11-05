# Dynamic Flag

We'll build a first shader adding some particles to the flag depending on how close the player is.

## Custom GPU type

First step is to declare the data we'll send to the GPU:

```rust
# extern crate bevy;
# use bevy::{
#     prelude::*,
#     render::render_resource::{AsBindGroup, ShaderRef},
#     sprite::{AlphaMode2d, Material2d, Material2dPlugin},
# };
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FlagMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub atlas: Handle<Image>,
    #[uniform(2)]
    pub index: Vec4,
    #[uniform(3)]
    pub distance: Vec4,
}
```

By deriving the [`AsBindGroup`](https://docs.rs/bevy/0.15.0-rc.3/bevy/render/render_resource/trait.AsBindGroup.html) trait and annotating the field of the struct, Bevy will be able to know how to transform the data from Rust type to what is expected by the GPU:
* `atlas` has the handle to the spritesheet
* `index` is the index of the sprite in the spritesheet. Bevy uses a single `u32` for that, and get the number of rows and columns from the [`TextureAtlasLayout`](https://docs.rs/bevy/0.15.0-rc.3/bevy/prelude/struct.TextureAtlasLayout.html). We'll do simpler and hard code some values, and use `(i, j)` coordinatesto specify which sprite to use
* `distance` is the distance between the flag and the player

<div class="warning">

`index` will have a `Vec2`, and `distance` a `f32`, but they are both defined as `Vec4`. This is for WebGL2 compatibility, where types must be aligned on 16 bytes.

The two strategies to solve that are padding and packing. Padding is using bigger types than necessary and wasting memory, packing is grouping fields that have separate meaning in a single type.

This workshop use padding as it's easier to read and the material is only used once, so doesn't waste a lot of memory.

</div>

## Custom Material

Next is to define the shader that will be used to render the data. This is done by implementing the [`Material2d`](https://docs.rs/bevy/0.15.0-rc.3/bevy/sprite/trait.Material2d.html) trait:

```rust
# extern crate bevy;
# use bevy::{
#     prelude::*,
#     render::render_resource::{AsBindGroup, ShaderRef},
#     sprite::{AlphaMode2d, Material2d, Material2dPlugin},
# };
# #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
# pub struct FlagMaterial {}
impl Material2d for FlagMaterial {
    fn fragment_shader() -> ShaderRef {
        "flag_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
```

The trait has more customisation than used here, and use sane defaults. By just using a string for the fragment shader, Bevy will load the file specified from the asset folder.

This is a basic shader that will display the sprite selected by the `index` from a sprite sheet:

```wgsl
#import bevy_sprite::{
    mesh2d_vertex_output::VertexOutput,
    mesh2d_view_bindings::globals,
}

@group(2) @binding(0) var base_color_texture: texture_2d<f32>;
@group(2) @binding(1) var base_color_sampler: sampler;
@group(2) @binding(2) var<uniform> index: vec4<f32>;
@group(2) @binding(3) var<uniform> distance_to_player: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let atlas_width = 1024.0;
    let atlas_height = 512.0;
    let sprite_size = 128.0;

    var texture = textureSample(
        base_color_texture,
        base_color_sampler,
        vec2<f32>((mesh.uv.x + index.x) * sprite_size / atlas_width, (mesh.uv.y + index.y) * sprite_size / atlas_height)
    );

    return texture;
}
```

Bevy has some extensions to WGSL to allow imports and expose some helpful features.

Variables with the `@group(2)` will match the bind group declared on Rust side.

## Using the Material

Our new material must be added to Bevy before it can be used. This can be done in a plugin:

```rust
# extern crate bevy;
# use bevy::{
#     prelude::*,
#     render::render_resource::{AsBindGroup, ShaderRef},
#     sprite::{AlphaMode2d, Material2d, Material2dPlugin},
# };
# #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
# pub struct FlagMaterial {}
# impl Material2d for FlagMaterial {}
pub struct FlagPlugin;

impl Plugin for FlagPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<FlagMaterial>::default());
    }
}
```

Then we can replace `Sprite` for the flag with our new material:

```rust
# extern crate bevy;
# use bevy::{
#     prelude::*,
#     render::render_resource::{AsBindGroup, ShaderRef},
#     sprite::{AlphaMode2d, Material2d, Material2dPlugin},
# };
# #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
# pub struct FlagMaterial {
#     #[texture(0)]
#     #[sampler(1)]
#     pub atlas: Handle<Image>,
#     #[uniform(2)]
#     pub index: Vec4,
#     #[uniform(3)]
#     pub distance: Vec4,
# }
# impl Material2d for FlagMaterial {}
# enum Tile { Flag }
# #[derive(Component)]
# struct Flag;
# #[derive(Event)]
# struct ReachedFlag;
# fn reached_flag(_trigger: Trigger<ReachedFlag>) {}
# struct GameAssets {
#     items_image: Handle<Image>,
#     items_layout: Handle<TextureAtlasLayout>,
# }
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Game }
fn display_tile(
    // ...
    meshes: &mut Assets<Mesh>,
    flag_materials: &mut Assets<FlagMaterial>,
) {
    # let commands: Commands = unimplemented!();
    # let assets: GameAssets = unimplemented!();
    # let (x, y) = (0.0, 0.0);
    # let tile = Tile::Flag;
    match tile {
        // ...
        Tile::Flag => {
            commands
                .spawn((
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(flag_materials.add(FlagMaterial {
                        atlas: assets.items_image.clone(),
                        index: Vec4::new(0.0, 1.0, 0.0, 0.0),
                        distance: Vec4::ZERO,
                    })),
                    Transform::from_xyz(x, y, 1.0).with_scale(Vec3::splat(0.5) * 128.0),
                    StateScoped(GameState::Game),
                    Flag,
                ))
                .observe(reached_flag);
        }
        // ...
    }
}
```
