
#version 450
layout(location = 0) in vec2 UV;
layout(location = 0) out vec4 COLOR;

layout(set = 2, binding = 1) uniform texture2D texture_;
layout(set = 2, binding = 2) uniform sampler sampler_;
