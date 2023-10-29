#version 330 core

layout(location = 0) in vec2 in_position;// Vertex position

uniform uvec2 u_position;// Widget position
uniform uvec4 u_margin;// Margin (top, right, bottom, left)
uniform uvec4 u_padding;// Padding (top, right, bottom, left)

out vec2 frag_position;

void main() {
    // Adjust the vertex position based on the widget position, margin, and padding
    vec2 adjusted_position = in_position + u_position - u_margin.xy - u_padding.xy;
    gl_Position = vec4(adjusted_position, 0.0, 1.0);
    frag_position = adjusted_position;// Pass adjusted position to fragment shader
}