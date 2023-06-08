// Vertex shader

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) xy: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) index: u32,
) -> VertexOutput {

    var pos = array<vec2<f32>,6>(
        // vec2<f32>( 0.0, 0.5),
        // vec2<f32>(-0.5, -0.5),
        // vec2<f32>( 0.5, -0.5)
        vec2<f32>( -1.0, -1.0),
        vec2<f32>(  1.0, -1.0),
        vec2<f32>( -1.0,  1.0),

        vec2<f32>( -1.0,  1.0),
        vec2<f32>(  1.0, -1.0),
        vec2<f32>(  1.0,  1.0),
    );
    var rainbow = array<vec2<f32>,6>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),

        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(1.0, 1.0),
    );

    var xy = array<vec2<f32>,6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>(-1.0,  1.0),

        vec2<f32>(-1.0,  1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
    );

    // let solid = ...
    // May only be indexed by a constant where as:
    // var solid = ...
    // May be indexed by a varialble.
    var solid = array<vec3<f32>,6>(
        vec3<f32>(1.0, 1.0, 0.0),
        vec3<f32>(1.0, 1.0, 0.0),
        vec3<f32>(1.0, 1.0, 0.0),

        vec3<f32>(1.0, 0.0, 1.0),
        vec3<f32>(1.0, 0.0, 1.0),
        vec3<f32>(1.0, 0.0, 1.0),
    );


    var out: VertexOutput;
    // let x = f32(1 - i32(in_vertex_index)) * 0.5;
    // let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    // out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.position = vec4<f32>(pos[index], 0.0, 1.0);
    // out.color = vec4<f32>(solid[index], 1.0);
    out.xy = xy[index];
    return out;
}

// Fragment shader

@fragment
fn fs_wire(in: VertexOutput) -> @location(0) vec4<f32> {
    // return vec4<f32>(0.5, 0.5, 0.0, 1.0);
    return vec4<f32>(in.xy, 0.0, 1.0);
}

@fragment
fn fs_fill(in: VertexOutput) -> @location(0) vec4<f32> {
    // return vec4<f32>(0.5, 0.5, 0.0, 1.0);
    return vec4<f32>(adjust(in.xy.x), adjust(in.xy.y), 0.0, 1.0);
}

fn adjust(x: f32) -> f32 { return (x + 1.0) / 2.0; }

@group(0) @binding(0)
var<uniform> div_size: vec2<f32>;

@fragment
fn fs_convert(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(
        in.position.x * div_size.x / 2.0,
        1.0 + in.position.y * div_size.y / 2.0,
        0.0,
        1.0
    );
}

@fragment
fn fs_sdf(in: VertexOutput) -> @location(0) vec4<f32> {
    let epsilon = 6.0E-8;
    // let epsilon = 0.001;
    // let xy = vec2<f32>(
    //     -1.0 + in.position.x * div_size.x,
    //     1.0 + in.position.y * div_size.y,
    // );
    // let xy = vec2<f32>(in.color.x, in.color.y);
    var znext: f32 = 0.0;
    var color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    var count: u32 = 0u;
    for (var z: f32 = 1.0; z >= 0.0; z -= znext) {
        znext = distSphere(vec3<f32>(in.xy, z));
        if abs(znext) <= epsilon {
            z -= znext;
            color = vec4<f32>(z, 1.0, 1.0, 1.0);
            break;
        }
        count++;
    }
    return color;
    // let z = distSphere(vec3<f32>(xy, 1.0));
    // return vec4<f32>(z, 0.0, 0.0, 1.0);
}

// distance from sphere
fn distSphere(p: vec3<f32>) -> f32 {
    let radius = 1.0;
    return length(p) - radius;
}
