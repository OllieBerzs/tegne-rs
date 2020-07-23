layout(location = 0) out vec4 out_color;

void fragment() {
    vec3 tint = material.arg_1.rgb;
    float alpha = tex(object.albedo_index, in_uv).r;

    out_color = vec4(tint, alpha);
}