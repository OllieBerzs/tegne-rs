// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// skybox sampling shader

#define DEPTH test
#define CULL disabled
#define SHAPE filled_triangles

#define VERTEX_POSITION_SKYBOXSPACE

layout(location = 0) out vec4 out_color;

void fragment() {
    out_color = texture(samplerCube(skybox, sampler_em), in_modelspace_position);
}
