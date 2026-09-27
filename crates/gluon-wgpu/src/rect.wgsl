struct Viewport {
  size: vec2f,
  padding: vec2f,
}

@group(0) @binding(0)
var<uniform> viewport: Viewport;

struct VertexOutput {
  @builtin(position) position: vec4f,
  @location(0) color: vec4f,
}

@vertex
fn vertex_main(
  @builtin(vertex_index) index: u32,
  @location(0) rect: vec4f,
  @location(1) color: vec4f,
) -> VertexOutput {
  let positions = array<vec2f, 6>(
    vec2f(0.0, 0.0),
    vec2f(1.0, 0.0),
    vec2f(0.0, 1.0),
    vec2f(0.0, 1.0),
    vec2f(1.0, 0.0),
    vec2f(1.0, 1.0),
  );

  let pixel_position = rect.xy + positions[index] * rect.zw;

  return VertexOutput(
    vec4f(
      pixel_position.x / viewport.size.x * 2.0 - 1.0,
      1.0 - pixel_position.y / viewport.size.y * 2.0,
      0.0,
      1.0,
    ),
    color,
  );
}

@fragment
fn fragment_main(input: VertexOutput) -> @location(0) vec4f {
  return input.color;
}
