<h1 align="center">DEM</h1>

## Installation

- Linux

```bash
# download prebuilt binary from github repo releases
cd ~/Downloads/
sudo cp dem /bin/
```

arguments:

- output format

| file format | file extenstion | argument value |
| --- | --- | ---|
|Collada|.dae|-1|
|Alembic|.abc|-2|
|Universal Scene Description|.usd|-3|
|Wavefront|.obj|-4|
|Stanford|.ply|-5|
|Stl|.stl|-6|
|FBX|.fbx|-7|
|glTF 2.0|.gltf|-8|
|X3D Extensible 3D|.x3d|-9|

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