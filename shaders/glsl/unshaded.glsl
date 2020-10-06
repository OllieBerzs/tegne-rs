// Oliver Berzs
// https://github.com/oberzs/draw-it

// unshaded shader

#define DEPTH test_and_write
#define CULL back
#define SHAPE filled_triangles

layout(location = 0) out vec4 out_color;

void fragment() {
    vec4 color = vec4(material.arg_1.rgb, 1.0);
    uint texture = uint(material.arg_1.w);
    out_color = tex(texture, in_uv) * in_color * color;
}
