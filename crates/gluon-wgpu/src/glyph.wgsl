struct Viewport {
  physical_size: vec2f,
  scale_factor: f32,
  padding: f32,
}

@group(0) @binding(0)
var<uniform> viewport: Viewport;

@group(1) @binding(0)
var glyph_atlas: texture_2d<f32>;

@group(1) @binding(1)
var glyph_sampler: sampler;

struct VertexOutput {
  @builtin(position) position: vec4f,
  @location(0) uv: vec2f,
  @location(1) color: vec4f,
}

@vertex
fn vertex_main(
  @builtin(vertex_index) index: u32,
  @location(0) rect: vec4f,
  @location(1) uv: vec4f,
  @location(2) color: vec4f,
) -> VertexOutput {
  let positions = array<vec2f, 6>(
    vec2f(0.0, 0.0),
    vec2f(1.0, 0.0),
    vec2f(0.0, 1.0),
    vec2f(0.0, 1.0),
    vec2f(1.0, 0.0),
    vec2f(1.0, 1.0),
  );

  let logical_position = rect.xy + positions[index] * rect.zw;
  let physical_position = logical_position * viewport.scale_factor;

  return VertexOutput(
    vec4f(
      physical_position.x / viewport.physical_size.x * 2.0 - 1.0,
      1.0 - physical_position.y / viewport.physical_size.y * 2.0,
      0.0,
      1.0,
    ),
    uv.xy + positions[index] * uv.zw,
    color,
  );
}

@fragment
fn fragment_main(input: VertexOutput) -> @location(0) vec4f {
  let coverage = textureSample(glyph_atlas, glyph_sampler, input.uv).r;

  return vec4f(input.color.rgb, input.color.a * coverage);
}
