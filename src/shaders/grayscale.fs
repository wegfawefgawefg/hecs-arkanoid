#version 330

in vec2 fragTexCoord;

uniform sampler2D image;
uniform vec4 colDiffuse;

out vec4 finalColor;

const int samples = 5; // Number of samples for averaging
const float blurWidth = 0.3; // Control the blur extent, adjust as needed

void main() {
    vec4 color = vec4(0.0);
    vec2 texSize = textureSize(image, 0);
    vec2 texOffset = 1.0 / texSize;

    for(int i = -samples; i <= samples; i++) {
        for(int j = -samples; j <= samples; j++) {
            color += texture(image, fragTexCoord + vec2(texOffset.x * i * blurWidth, texOffset.y * j * blurWidth));
        }
    }

    finalColor = (color / float((samples * 2 + 1) * (samples * 2 + 1))) * colDiffuse;
}
