<h1 align="center">DEM data conversion</h1>

## Installation

- build from source

```bash
g++ main.c -O3
```



arguments:

- output format

| file format | file extenstion | argument value | support |
| --- | --- | --- | --- |
|Collada|.dae|-1|:x:|
|Alembic|.abc|-2|:x:|
|Universal Scene Description|.usd|-3|:x:|
|Wavefront|.obj|-4|:heavy_check_mark:|
|Stanford|.ply|-5|:x:|
|Stl|.stl|-6|:x:|
|FBX|.fbx|-7|:x:|
|glTF 2.0|.gltf|-8|:x:|
|X3D Extensible 3D|.x3d|-9|:x:|

- base shape

|shape|argument value|
| --- | --- |
|sphere|-s|
|ellipsoid|-e|
|plain|-p|

- input file

|extensions|argument value|
| --- | --- |
|jpg/png|(path-to-file)|