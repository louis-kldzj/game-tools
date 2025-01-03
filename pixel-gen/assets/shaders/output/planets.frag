#version 450

layout(location = 0) in vec2 UV;
layout(location = 0) out vec4 COLOR;

layout(set = 2, binding = 0) uniform float size;
layout(set = 2, binding = 1) uniform int OCTAVES;
layout(set = 2, binding = 2) uniform float seed;
layout(set = 2, binding = 3) uniform float pixels;

layout(set = 2, binding = 5) uniform texture2D texture_;
layout(set = 2, binding = 6) uniform sampler sampler_;

layout(set = 2, binding = 4) uniform vec2 light_origin;

float rand(vec2 coord) {
	return fract(sin(dot(coord.xy ,vec2(12.9898,78.233))) * 15.5453 * seed);
}

float noise(vec2 coord){
	vec2 i = floor(coord);
	vec2 f = fract(coord);
		
	float a = rand(i);
	float b = rand(i + vec2(1.0, 0.0));
	float c = rand(i + vec2(0.0, 1.0));
	float d = rand(i + vec2(1.0, 1.0));

	vec2 cubic = f * f * (3.0 - 2.0 * f);

	return mix(a, b, cubic.x) + (c - a) * cubic.y * (1.0 - cubic.x) + (d - b) * cubic.x * cubic.y;
}

float fbm(vec2 coord){
	float value = 0.0;
	float scale = 0.5;

	for(int i = 0; i < OCTAVES ; i++){
		value += noise(coord) * scale;
		coord *= 2.0;
		scale *= 0.5;
	}
	return value;
}

bool dither(vec2 uv1, vec2 uv2) {
	return mod(uv1.y+uv2.x,2.0/pixels) <= 1.0 / pixels;
}

float circleNoise(vec2 uv) {
    float uv_y = floor(uv.y);
    uv.x += uv_y*.31;
    vec2 f = fract(uv);
	float h = rand(vec2(floor(uv.x),floor(uv_y)));
    float m = (length(f-0.25-(h*0.5)));
    float r = h*0.25;
    return smoothstep(0.0, r, m*0.75);
}

vec2 rotate(vec2 vec, float angle) {
	vec -=vec2(0.5);
	vec *= mat2(vec2(cos(angle),-sin(angle)), vec2(sin(angle),cos(angle)));
	vec += vec2(0.5);
	return vec;
}

vec2 spherify(vec2 uv) {
	vec2 centered = uv *2.0-1.0;
	float z = sqrt(1.0 - dot(centered.xy, centered.xy));
//	float z = pow(1.0 - dot(centered.xy, centered.xy), 0.5);
	vec2 sphere = centered/(z + 1.0);
	
	return sphere * 0.5+0.5;
}

float cloud_alpha(vec2 uv) {
	float c_noise = 0.0;
	
	// more iterations for more turbulence
	int iters = 2;
	for (int i = 0; i < iters; i++) {
		float relative = (float(i)/float(iters));
		vec2 c_uv = rotate(uv, relative * 6.28);
		c_noise += circleNoise((uv * 0.3) + (float(i+1)+10.));
	}
	float fbm = fbm(uv+c_noise);
	
	return fbm;
}


void main() {
	/// pixelzing and dithering
	vec2 uv = floor(UV * pixels) / pixels;
	bool dith = dither(UV, uv);
	
	// distance from center, to create a circle
	float d_to_center = distance(uv , vec2(0.5));
	uv = spherify(uv);
	
	// distance from light source, to create shading
	float d_to_light = distance(uv, light_origin);
	// bit of contrast
	d_to_light += pow(d_to_center * 3.0, 4.0) * 0.05;
	
	// noise for the planet
	float n = fbm(uv * size);
	float n2 = fbm(uv * size + n*3.0);
	float mixed = n2 * d_to_light;
	
	// optionally create some contrast with this
//	mixed = pow(mixed, 1.0);

	// apply dithering
	if (dith) {
		mixed *= 0.95;
	}
	
	// choose and apply colors
	float col_val = floor(mixed * 15.0) / 7.0;
	vec3 col = texture(sampler2D(texture_, sampler_), vec2(col_val, 0.0)).rgb;
	
	// apply alpha
	float a = step(d_to_center, 0.5);
	COLOR = vec4(col, a);
}
