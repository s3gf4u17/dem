#include <stdio.h>
#include <math.h>
#include <time.h>
#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"

int main(int argc, char* argv[]) {
    clock_t begin = clock();
    char* input = argv[1];
    char* base = argv[2];
    int x,y,n;
    unsigned char *data = stbi_load(input,&x,&y,&n,0);

    // printf("opened image %ix%i in %i channel mode\n",x,y,n);

    FILE *file;
    file = fopen("model.obj","w");
    fprintf(file,"o DEM\n");

    int vertices,faces = 0;

    switch(n) {
        case 1:
            for (int i=0;i<x*y*n;i++) {
                double phi = M_PI/2.0-(M_PI/(double)y)*((double)i/(double)x);
                double lambda = -M_PI+(2.0*M_PI/(double)x)*(i%x);
                double h = (double)data[i]/255.0/10.0;

                double a = 1.0;
                double b = 0.95;
                double e2 = (a*a-b*b)/(a*a);
                double n = a/sqrt(1.0-e2*sin(phi)*sin(phi));
                double px = (n+h)*cos(phi)*cos(lambda);
                double py = (n+h)*cos(phi)*sin(lambda);
                double pz = (n*(1.0-e2)+h)*sin(phi);
                
                fprintf(file,"v %f %f %f\n",py,pz,px);
                vertices++;
            }
            for (int i=1;i<y+1;i++) {
                for (int j=1;j<x+1;j++) {
                    fprintf(file,"f %i %i %i %i\n",x*(i-1)+j,x*(i-1)+j+1,x*i+j+1,x*i+j);
                    faces++;
                }
            }
            break;
        default:
            break;
    }

    fclose(file);
    stbi_image_free(data);
    clock_t end = clock();
    printf("-- DONE\nVertices: %i\nFaces: %i\nCPU time used: %fs\n",vertices,faces,(double)(end-begin)/CLOCKS_PER_SEC);
    return 0;
}