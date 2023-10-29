#version 330 core

uniform vec4 u_bg_color;// Background color
uniform vec4 u_fg_color;// Foreground color
uniform float u_border_thickness;// Border thickness
uniform uvec4 u_corner_radius;// Corner radius (top-left, top-right, bottom-right, bottom-left)

in vec2 frag_position;

out vec4 color;

void main() {
    // Simple color assignment (no border, corner radius, or other features yet)
    color = u_bg_color;
}
