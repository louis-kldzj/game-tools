#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 0) out vec2 UV;

layout(set = 2, binding = 9) uniform vec3 position;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
    mat4 View;
    mat4 InverseView;
    mat4 Projection;
    vec3 WorldPosition;
    float width;
    float height;
};

void main() {
    UV = Vertex_Uv;
    // Translate the vertex position by the object's position
    vec4 worldPosition = vec4(Vertex_Position + position, 1.0);
    // Transform the position to clip space using the ViewProj matrix
    gl_Position = ViewProj * worldPosition;

}


