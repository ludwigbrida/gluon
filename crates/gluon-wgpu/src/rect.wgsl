struct VertexOutput {
    @builtin(position)
    position: vec4f,
}

@vertex
fn vertex_main(@builtin(vertex_index)index: u32) -> VertexOutput {
    let positions = array<vec2f, 6>(
        vec2f(-0.75, 0.75),
        vec2f(-0.25, 0.75),
        vec2f(-0.75, 0.25),
        vec2f(-0.75, 0.25),
        vec2f(-0.25, 0.75),
        vec2f(-0.25, 0.25),
    );

    return VertexOutput(vec4f(positions[index], 0.0, 1.0));
}

@fragment
fn fragment_main() -> @location(0) vec4f {
    return vec4f(0.1, 0.4, 0.9, 1.0);
}
