
#version 450
layout(location = 0) in vec2 UV;
layout(location = 0) out vec4 COLOR;

layout(set = 2, binding = 1) uniform texture2D texture_;
layout(set = 2, binding = 2) uniform sampler sampler_;

layout(set = 2, binding = 4) uniform texture2D star;
layout(set = 2, binding = 5) uniform sampler starSampler;

layout(set = 2, binding = 6) uniform int starType;

const vec2 sheetSize = vec2(144, 24);
const vec2 spriteSize = vec2(24, 24);

vec2 spriteSheetUV() {
  // Calculate the number of sprites per row and column
  vec2 spriteCount = sheetSize / spriteSize; // E.g., (8, 8) for a 192x192 sheet with 24x24 sprites

  // Calculate the row and column of the selected sprite
  int spriteRow = starType / int(spriteCount.x); // Row index
  int spriteCol = starType % int(spriteCount.x); // Column index

  // Base UV coordinates for the selected sprite
  vec2 spriteUVOffset = vec2(spriteCol, spriteRow) * spriteSize / sheetSize;

  // Scale the input UV to fit the size of one sprite
  vec2 scaledUV = UV * spriteSize / sheetSize;

  // Final UV for the selected sprite
  vec2 finalUV = spriteUVOffset + scaledUV;
  return finalUV;
}

void main() {
  vec2 uv = spriteSheetUV();
  float col_val = texture(sampler2D(star, starSampler), uv).r;
  float a = texture(star, uv).a;
  vec4 replace_col = texture(sampler2D(texture_, sampler_), vec2(round(col_val * 7.0) / 7.0, 0.0));

  COLOR = vec4(replace_col.rgb, a);

}
