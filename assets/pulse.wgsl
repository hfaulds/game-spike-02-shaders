#import bevy_sprite::mesh2d_view_bindings
#import bevy_sprite::mesh2d_bindings

let speed = 2.5;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = view.view_proj * mesh.model * vec4<f32>(vertex.position, 1.0);
    out.color = vertex.color;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let alph = (sin(globals.time * speed) + 1.0) / 2.0;
    return vec4<f32>(in.color[0], in.color[1], in.color[2], alph);
}
