#include <stdio.h>
#include <math.h>
#include <time.h>
#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"

int main(int argc, char* argv[]) {
    // create clock instance to measure execution time
    clock_t begin = clock();
    // create labels for command line arguments
    char* output = argv[1];
    char* shape = argv[2];
    char* input = argv[3];
    // create width, height, channel count variables and load an image
    int width,height,channel_count;
    unsigned char *data = stbi_load(input,&width,&height,&channel_count,0);
    // DEV log image metadata
    printf("-- IMAGE\nWidth: %i\nHeight: %i\nChannel count: %i\n",width,height,channel_count);
    // create info counters for vertices and faces
    int vertex_count = 0,face_count = 0;
    // allocate space in memory for vertex data
    double* vertex_data = (double*)malloc(width*height*sizeof(double)*3);

    // generate vertex data
    switch(shape[1]) {
        case 's':
            for (int i=0;i<width*height;i++) {
                double phi = M_PI/2.0-(M_PI/(double)height)*((double)i/(double)width);
                double lambda = -M_PI+(2.0*M_PI/(double)width)*(i%width);
                double height = (double)data[i]/255.0/10.0;
                double a = 1.0;
                double b = 1.0;
                double e2 = (a*a-b*b)/(a*a);
                double n = a/sqrt(1.0-e2*sin(phi)*sin(phi));
                vertex_data[3*i+2] = (n+height)*cos(phi)*cos(lambda); // x
                vertex_data[3*i] = (n+height)*cos(phi)*sin(lambda); // y
                vertex_data[3*i+1] = (n*(1.0-e2)+height)*sin(phi); // z
            }
            break;
        case 'e':
            for (int i=0;i<width*height;i++) {
                double phi = M_PI/2.0-(M_PI/(double)height)*((double)i/(double)width);
                double lambda = -M_PI+(2.0*M_PI/(double)width)*(i%width);
                double height = (double)data[i]/255.0/10.0;
                double a = 1.0;
                double b = 0.95;
                double e2 = (a*a-b*b)/(a*a);
                double n = a/sqrt(1.0-e2*sin(phi)*sin(phi));
                vertex_data[3*i+2] = (n+height)*cos(phi)*cos(lambda); // x
                vertex_data[3*i] = (n+height)*cos(phi)*sin(lambda); // y
                vertex_data[3*i+1] = (n*(1.0-e2)+height)*sin(phi); // z
            }
            break;
        case 'p':
            {
                double pixel = width > height ? 1.0/(double)width : 1.0/(double)height;
            }
            break;
        default:
            break;
    }

    // save vertex data to output format
    switch(output[1]) {
        case '1':
            break;
        case '2':
            break;
        case '3':
            break;
        case '4':
            // create file stream
            FILE *file;
            file = fopen("model.obj","w");
            fprintf(file,"o DEM\n");
            for (int i=0;i<width*height;i++) {
                fprintf(file,"v %f %f %f\n",vertex_data[3*i],vertex_data[3*i+1],vertex_data[3*i+2]);
                vertex_count++;
            }
            fprintf(file,"s 1\n");
            for (int i=0;i<height+1;i++) {
                for (int j=0;j<width+1;j++) {
                    fprintf(file,"f %i %i %i %i\n",width*(i-1)+j,width*(i-1)+j+1,width*i+j+1,width*i+j);
                    face_count++;
                }
            }
            fclose(file);
            break;
        case '5':
            break;
        case '6':
            break;
        case '7':
            break;
        case '8':
            break;
        case '9':
            break;
        default:
            break;
    }

    free(vertex_data);
    stbi_image_free(data);
    clock_t end = clock();
    printf("-- DONE\nVertices: %i\nFaces: %i\nCPU time used: %fs\n",vertex_count,face_count,(double)(end-begin)/CLOCKS_PER_SEC);
    return 0;
}