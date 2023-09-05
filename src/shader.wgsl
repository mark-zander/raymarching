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

@group(1) @binding(0)
var cube_tex: texture_cube<f32>;
@group(1)@binding(1)
var cube_sampler: sampler;

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
    var dnext: Data = Data(0.0, vec4<f32>(0.0));
    var hit: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    var color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    var count: u32 = 0u;
    for (var z: f32 = 1.0; z >= -1.0; z -= dnext.dist) {
        dnext = scene4(vec3<f32>(in.xy, z));
        if abs(dnext.dist) <= epsilon {
            // z -= znext;
            hit = vec3<f32>(in.xy, z);
            // color = vec4<f32>((z + 1.0) / 2.0, 1.0, 1.0, 1.0);
            color = dnext.color; // + (z + 1.0) / 2.0;
            break;
        }
        count++;
    }
    // color = textureSample(cube_tex, cube_sampler, hit);
    // if abs(znext) > epsilon { color = vec4<f32>(0.0); }
    return color;
    // let z = distSphere(vec3<f32>(xy, 1.0));
    // return vec4<f32>(z, 0.0, 0.0, 1.0);
}

const red: vec4<f32> = vec4<f32>(1.0, 0.0, 0.0, 1.0);
const green: vec4<f32> = vec4<f32>(0.0, 1.0, 0.0, 1.0);
const blue: vec4<f32> = vec4<f32>(0.0, 0.0, 1.0, 1.0);
const pi: f32 = 3.14159;

fn scene1(p: vec3<f32>) -> Data {
    let p2 = (translate( 0.25,  0.0,  0.0) * vec4(p, 1.0)).xyz;
    let p1 = (translate(-0.25,  0.0, -0.0) * vec4(p, 1.0)).xyz;
    return subtract(
        color(sphere(p1, 0.5), red),
        color(sphere(p2, 0.5), green)
    );
}

fn scene2(p: vec3<f32>) -> Data {
    return intersect(
        color(box(p, vec3(0.38)), red),
        color(sphere(p, 0.5), blue)
    );
}

fn scene3(p: vec3<f32>) -> Data {
    let p1 = (rotz(pi / 2.0) * vec4(p, 1.0)).xyz;
    let p2 = (rotx(pi / 2.0) * vec4(p, 1.0)).xyz;
    return unions(
        color(cappedCylinder(p2, 0.4, 0.1), green),
        unions(
            color(cappedCylinder(p, 0.4, 0.1), green),
            color(cappedCylinder(p1, 0.4, 0.1), green)
        )
    );
}

fn scene4(p: vec3<f32>) -> Data {
    return subtract(scene3(p), scene2(p));
}

fn scene5(p: vec3<f32>) -> Data {
    return color(box(p, vec3(0.5)), red);
}

// distance from sphere
fn sphere(p: vec3<f32>, radius: f32) -> f32 {
    return length(p) - radius;
}

// distance from a box
fn box(p: vec3<f32>, b: vec3<f32>) -> f32 {
  let q = abs(p) - b;
  // let zero = vec3(0.0);
  return length(max(q, vec3(0.0))) + min(max(q.x, max(q.y, q.z)), 0.0);
}

// fn sdCappedCylinder( vec3 p, float h, float r ) -> f32 {
fn cappedCylinder(p: vec3<f32>, h: f32, r: f32) -> f32 {
  let d = abs(vec2(length(p.xz),p.y)) - vec2(r,h);
  return min(max(d.x, d.y), 0.0) + length(max(d, vec2(0.0)));
}

fn trans(p: vec3<f32>, m: mat4x4<f32>) -> vec3<f32> {
    return (m * vec4(p, 1.0)).xyz;
}

struct Data {
    dist: f32,
    color: vec4<f32>,
}

fn color(dist: f32, color: vec4<f32>) -> Data {
    return Data(dist, color);
}

fn recolor(in: Data, color: vec4<f32>) -> Data {
    return Data(in.dist, color);
}

fn unions(c1: Data, c2: Data) -> Data {
    if c1.dist < c2.dist { return c1; }
    return c2;
}
fn intersect(c1: Data, c2: Data) -> Data {
    if c1.dist > c2.dist { return c1; }
    return c2;
}
fn subtract(c1: Data, c2: Data) -> Data {
    if -c1.dist > c2.dist { return Data(-c1.dist, c1.color); }
    return c2;
}
fn first(c1: Data, c2: Data) -> Data { return c1; }
fn second(c1: Data, c2: Data) -> Data { return c2; }

// fn unions(d1: f32, d2: f32) -> f32 { return min(d1, d2); }
// fn intersect(d1: f32, d2: f32) -> f32 { return max(d1, d2); }
// fn subtract(d1: f32, d2: f32) -> f32 { return max(-d1, d2); }

// matrix operations
fn translate(x: f32, y: f32, z: f32) -> mat4x4<f32> {
    return mat4x4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        x,   y,   z,   1.0
    );
}

fn rotx(theta: f32) -> mat4x4<f32> {
    return mat4x4(
        1.0, 0.0,        0.0,         0.0,
        0.0, cos(theta), -sin(theta), 0.0,
        0.0, sin(theta), cos(theta),  0.0,
        0.0, 0.0,        0.0,         1.0
    );
}

fn roty(theta: f32) -> mat4x4<f32> {
    return mat4x4(
        cos(theta),  0.0, sin(theta), 0.0,
        0.0,         1.0, 0.0,        0.0,
        -sin(theta), 0.0, cos(theta), 0.0,
        0.0,         0.0, 0.0,        1.0
    );
}

fn rotz(theta: f32) -> mat4x4<f32> {
    return mat4x4(
        cos(theta), -sin(theta), 0.0, 0.0,
        sin(theta), cos(theta),  0.0, 0.0,
        0.0,        0.0,         1.0, 0.0,
        0.0,        0.0,         0.0, 1.0
    );
}

