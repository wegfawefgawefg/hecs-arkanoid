#version 330

in vec2 fragTexCoord;

uniform sampler2D image;
uniform vec4 colDiffuse;

out vec4 finalColor;

const int neighborhood_size = 4; // Number of neighborhood_size for averaging
const float strength = 0.6; 

void main() {
    vec4 c = texture(image, fragTexCoord)*colDiffuse;

    vec4 blur_color = vec4(0.0);
    vec2 texSize = textureSize(image, 0);
    vec2 texOffset = 1.0 / texSize;

    float total_weight = 0.0;

    for(int i = -neighborhood_size; i <= neighborhood_size; i++) {
        for(int j = -neighborhood_size; j <= neighborhood_size; j++) {
            vec2 offset = vec2(texOffset.x * i, texOffset.y * j);
            vec4 sample = texture(image, fragTexCoord + offset);
            
            float distance = length(offset);
            distance = distance * distance;
            float weight = 1.0 / (distance + 0.001); // Avoid division by zero
            
            blur_color += sample * weight;
            total_weight += weight;
        }
    }

    blur_color /= total_weight;
    blur_color *= strength;

    finalColor = c + blur_color;
}
