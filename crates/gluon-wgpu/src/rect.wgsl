struct Viewport {
  physical_size: vec2f,
  scale_factor: f32,
  padding: f32,
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

  let logical_position = rect.xy + positions[index] * rect.zw;
  let physical_position = logical_position * viewport.scale_factor;

  return VertexOutput(
    vec4f(
      physical_position.x / viewport.physical_size.x * 2.0 - 1.0,
      1.0 - physical_position.y / viewport.physical_size.y * 2.0,
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
