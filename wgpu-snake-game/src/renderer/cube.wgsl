struct Cube {
    block_size_percentage_x: f32,
    block_size_percentage_y: f32
};

struct CubeInstance {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>
};

@group(0) @binding(0)
var<uniform> cube: Cube;

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    instance: CubeInstance
) -> VertexOutput {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0)
    );

    return VertexOutput(
        vec4<f32>(
            positions[vertex_index] * vec2<f32>(cube.block_size_percentage_x, cube.block_size_percentage_y) + instance.position,
            0.0,
            1.0
        ),
        instance.color
    );
}

@fragment
fn fs_main(
    vertex_output: VertexOutput
) -> @location(0) vec4<f32> {
    return vertex_output.color;
}