struct RectUniform {
    rect: vec4f,
    color: vec4f,
    viewport: vec2f,
    padding: vec2f,
}

@group(0) @binding(0)
var<uniform> rectangle: RectUniform;

struct VertexOutput {
    @builtin(position)
    position: vec4f,
}

@vertex
fn vertex_main(@builtin(vertex_index)index: u32) -> VertexOutput {
    let positions = array<vec2f, 6>(
        vec2f(0.0, 0.0),
        vec2f(1.0, 0.0),
        vec2f(0.0, 1.0),
        vec2f(0.0, 1.0),
        vec2f(1.0, 0.0),
        vec2f(1.0, 1.0),
    );

    let position = rectangle.rect.xy + positions[index] * rectangle.rect.zw;

    return VertexOutput(vec4f(
        position.x / rectangle.viewport.x * 2.0 - 1.0,
        1.0 - position.y / rectangle.viewport.y * 2.0,
        0.0,
        1.0
    ));
}

@fragment
fn fragment_main() -> @location(0) vec4f {
    return rectangle.color;
}
