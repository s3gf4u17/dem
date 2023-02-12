#include <stdio.h>
#include <math.h>
#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"

int main() {
    int x,y,n;
    unsigned char *data = stbi_load("mars.jpg",&x,&y,&n,0);

    // printf("opened image %ix%i in %i channel mode\n",x,y,n);

    FILE *file;
    file = fopen("model.obj","w");
    fprintf(file,"o DEM\n");

    switch(n) {
        case 1:
            for (int i=0;i<x*y*n;i++) {
                double phi = M_PI/2.0-(M_PI/(double)y)*((double)i/(double)x);
                double lambda = -M_PI+(2.0*M_PI/(double)x)*(i%x);
                double h = (double)data[i]/255.0*220000;

                double a = 3396190.0;
                double b = 3376200.0;
                double e2 = (a*a-b*b)/(a*a);
                double n = a/sqrt(1.0-e2*sin(phi)*sin(phi));
                double px = (n+h)*cos(phi)*cos(lambda);
                double py = (n+h)*cos(phi)*sin(lambda);
                double pz = (n*(1.0-e2)+h)*sin(phi);
                
                fprintf(file,"v %f %f %f\n",px,pz,py);
            }
            break;
        default:
            break;
    }

    fclose(file);
    stbi_image_free(data);
    return 0;
}