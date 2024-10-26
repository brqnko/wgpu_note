struct Background {
    width_percentage: f32,
    height_percentage: f32,
};

@group(0) @binding(0)
var<uniform> background: Background;

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0)
    );

    return vec4<f32>(
        positions[vertex_index].x * background.width_percentage,
        positions[vertex_index].y * background.height_percentage,
        0.0,
        1.0
    );
}

@fragment
fn fs_main(
    @builtin(position) vertex_position: vec4<f32>
) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
