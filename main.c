#include <stdint.h>
#include <stdio.h>
#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"
#include <math.h>

void exportobj(double *coordinates,int width,int height,int isEllipsoid);
void ellipsoid(double * coordinates,uint8_t* rgb_image,int width,int height);
void plain(double *coordinates,uint8_t* rgb_image,int width,int height);

int main(int argc, char *argv[]) {
    // read image
    int width,height,bpp;
    uint8_t* rgb_image = stbi_load("test/mars.jpg", &width, &height, &bpp, 3);

    // allocate memory space for coordinates
    double *coordinates = (double*)malloc(width*height*sizeof(double));

    ellipsoid(coordinates,rgb_image,width,height);

    exportobj(coordinates,width,height,0);

    stbi_image_free(rgb_image);
    free(coordinates);
    return 0;
}

void ellipsoid(double *coordinates,uint8_t* rgb_image,int width,int height) {
    double a = 10.0;
    double b = 10.0;
    double e2 = ((a*a)-(b*b))/(a*a);
    // insert (phi lambda h) into coordinates
    for (int i=0;i<width*height;i=i+3){
        double phi = M_PI/2.0-(M_PI/double(height))*(i/width);
        double lambda = -M_PI+(2.0*M_PI/double(width))*(i%width);
        double h = double(rgb_image[i])/10.0;
        coordinates[i]=phi;
        coordinates[i+1]=lambda;
        coordinates[i+2]=h;
    }
    // transform coordinates to (x,y,z)
    for (int i=0;i<width*height;i=i+3){
        double phi = coordinates[i];
        double lambda = coordinates[i+1];
        double h = coordinates[i+2];
        double n = a/sqrt(1.0-e2*sin(phi)*sin(phi));
        double x = (n+h)*cos(phi)*cos(lambda);
        double y = (n+h)*cos(phi)*sin(lambda);
        double z = (n*(1-e2)+h)*sin(phi);
        coordinates[i]=x;
        coordinates[i+1]=y;
        coordinates[i+2]=z;
    }
}

void exportobj(double *coordinates,int width,int height,int isEllipsoid) {
    FILE *file;
    file = fopen("model.obj","w");

    fprintf(file,"o DEM\n");
    for (int i=0;i<width*height;i=i+3) {
        fprintf(file,"v %f %f %f\n",coordinates[i],coordinates[i+1],coordinates[i+2]);
    }

    fclose(file);
}