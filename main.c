#include <stdint.h>
#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"

int main(int argc, char *argv[]) {
    char *filename = argv[1];
    printf("%i",strcmp(filename,"test/plain.jpg"));

    int width,height,bpp;
    uint8_t* rgb_image = stbi_load(filename, &width, &height, &bpp, 3);
    printf("%i\n",width);
    stbi_image_free(rgb_image);
    return 0;
}