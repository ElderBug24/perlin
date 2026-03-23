# Multi dimensional perlin noise generation
Perlin noise (https://en.wikipedia.org/wiki/Perlin_noise) is a noise generation algorithm that works by interpolating between gradient vectors.
It allows for a smooth noise instead of random jittering.
It can be generated in 1, 2 or really any amount of dimensions like 3 for example with the x , y and time axis.
This project allows Perlin noise generation in n-dimensions, with consistent generation across dimensions: you can generate noise in 3D as well as in 5D on the same map, with lower dimensions corresponding to slices of the higher-dimensional space.
It also supports generating this noise with multiple layers, to make it look more or less smooth.

## Outputs
Here is a classical 2D output:

![Alt text](./output2.png?raw=true)

And here is a 3D output (the x, y and z axis where x and y correspond to the x and y pixel coordinates and z is the distance to the center of the image; it also is more 'detailed', as is was generated with more layers):
![Alt text](./output.png?raw=true)
