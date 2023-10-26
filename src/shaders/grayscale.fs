#version 330

in vec2 fragTexCoord;

uniform sampler2D image;
uniform vec4 colDiffuse;

out vec4 finalColor;

const int neighborhood_size = 20; // Number of neighborhood_size for averaging
const float strength = 0.5; 

void main() {
    vec4 c = texture(image, fragTexCoord)*colDiffuse;

    vec4 blur_color = vec4(0.0);
    vec2 texSize = textureSize(image, 0);
    vec2 texOffset = 1.0 / texSize;

    for(int i = -neighborhood_size; i <= neighborhood_size; i++) {
        for(int j = -neighborhood_size; j <= neighborhood_size; j++) {
            vec4 sample = texture(image, fragTexCoord + vec2(texOffset.x * i, texOffset.y * j));
            blur_color += sample;
        }
    }

    float num_samples = float((neighborhood_size * 2 + 1) * (neighborhood_size * 2 + 1));
    blur_color /= num_samples;
    blur_color *= strength;

    finalColor = c + blur_color;
}
