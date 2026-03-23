# Multi dimensional perlin noise generation
Perlin noise (https://en.wikipedia.org/wiki/Perlin_noise) is a noise generation algorithm that works by interpolating between gradient vectors.
It produces smooth noise instead of random jittering.
It can be generated in any number of dimensions, for example 3D with the x, y, and time axes.
This project allows Perlin noise generation in n-dimensions, with consistent generation across dimensions: you can generate noise in 3D as well as in 5D on the same map, with lower dimensions corresponding to slices of the higher-dimensional space.
It also supports multiple layers, allowing the noise to appear more or less smooth.

## Outputs
Here is a classical 2D output:

![Alt text](./output2.png?raw=true)

Here is a 3D output (the x, y and z axes where x and y correspond to the pixel coordinates and z is the distance to the center of the image; it also is more detailed, as it was generated with more layers):
![Alt text](./output.png?raw=true)

And here is a screenshot of the 5D animation example:
This animation is 5-dimensional: each pixel uses as coordinates its (x, y) position, the (mx, my) mouse coordinates, as well as time.
The executable was released on this repo.
![Alt text](./animation.png?raw=true)

