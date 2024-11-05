#import bevy_sprite::{
    mesh2d_vertex_output::VertexOutput,
    mesh2d_view_bindings::globals,
}


@group(2) @binding(0) var base_color_texture: texture_2d<f32>;
@group(2) @binding(1) var base_color_sampler: sampler;
@group(2) @binding(2) var<uniform> index: vec4<f32>;
@group(2) @binding(3) var<uniform> distance_to_player: vec4<f32>;


fn Hash12(t: f32) -> vec2<f32> {
    let x = fract(sin(t*748.32)*367.34);
    let y = fract(sin((t+x)*623.785)*292.45);
    
    return vec2(x,y)-.5;
}

fn Hash12Polar(t: f32) -> vec2<f32> {
    let a = fract(sin(t*748.31)*367.34)*6.2832;
    let d = fract(sin((t+a)*623.785)*292.45);
    
    return vec2<f32>(cos(a),sin(a))*d;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let atlas_width = 1024.0;
    let atlas_height = 512.0;
    let sprite_size = 128.0;

    var color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    let max_distance = 750.0;

    if distance_to_player.x < max_distance {
        // Adapted from https://www.shadertoy.com/view/7sjfRy
        for(var j = 0; j < 3; j++){
            for(var i = 0; i < 100; i++){

                let t = fract(globals.time);
                let bright = mix(0.002 * (1.0 - distance_to_player.x / max_distance), 0.001, smoothstep(0.025, 0.0, t) );
                let dir = Hash12Polar(f32(i)+1.);
                let dist = distance(mesh.uv - vec2(0.5,0.5) - dir*t, vec2(0, 0)+(Hash12Polar(f32(j*i))/2.));

                if bright / dist > 0.1 {
                    color.r = bright / dist * 2.0;
                    color.g = bright / dist / 2.0;
                    color.b = bright / dist / 2.0;
                    color.a = 1.0;
                }
            }
        }
    
    }

    var texture = textureSample(
        base_color_texture,
        base_color_sampler,
        vec2<f32>((mesh.uv.x + index.x) * sprite_size / atlas_width, (mesh.uv.y + index.y) * sprite_size / atlas_height)
    );

    if texture.a > 0.1 {
        color = texture;
    }

    return color;
}
