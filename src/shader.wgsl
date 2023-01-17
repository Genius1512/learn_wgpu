// Vertex shader

struct VertexInput {
   @location(0) position: vec3<f32>,
   @location(1) color: vec3<f32>, // location 1 is not used
}

struct VertexOutput {
   @builtin(position) clip_position: vec4<f32>, // The position will be specified by vs_main
   @location(0) color: vec3<f32>, // Uses the position, (which is stored in location 0) as the color
}

@vertex
fn vs_main(
    model: VertexInput, // The input, location 0 is specified by the program, which is the position
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color; // Set the color
    out.clip_position = vec4<f32>(model.position, 1.0); // Set the position
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> { // Pick the output of the vertex shader up and return it
    return vec4<f32>(in.color, 1.0);
}