# meshes.rs
Meshes.rs is a stupid adventure into computer graphics and linear algebra
this is somewhat of a prelude into a project I'm going to initate next year, 
diving into computer graphics, gpu acceleration, and glsl through vulkan. 
I want this to be without as many crates as possible, as of now I intend to only use the standard rust library
Many thanks to %⌵ᐂ⛉ for providing a lot of advice and guidance

## todo
- [ ] Fix whatever issue in the triangle SDF is causing issues, probably vertex at 0.0, 0.0, 0.0?
- [ ] Fix whatever is going on with more complicated object rendering
- [ ] Turn the object struct into a few enums to have more efficient sdf
- [ ] generate all primitaves with object struct (currently only rect and tetrahedron)
- [ ] make way to convert point cloud to convex hull
- [ ] make a way to import .obj files
- [ ] progress bars
- [ ] egui, so we can have runtime stuff (dream is being able to watch rendering process "realtime")
- [ ] multithread properly (without, uh, memory leaks??)

## current issues
currently it appears that either the projection matrix is borked, the function to 
get distance from a point to a face is borked, or the output is borked (unlikely)

### resources/citations
Michael Walczyk's [ray marching](https://michaelwalczyk.com/blog-ray-marching.html) blogpost <br />
Peter Shirley's [ray tracing in one weekend](https://raytracing.github.io/) series <br />
The ever radiant Sebastian Lague's [raytracing](https://www.youtube.com/watch?v=Qz0KTGYJtUk) and [raymarching](https://www.youtube.com/watch?v=Cp5WWtMoeKg) videos <br />
This stackoverflow [question](https://stackoverflow.com/questions/849211/shortest-distance-between-a-point-and-a-line-segment) and the [article](http://paulbourke.net/geometry/pointlineplane/) the answers seem to be based on <br />
Wikipedia -somehow managed to be the best resource- for the [ppm](https://en.wikipedia.org/wiki/Netpbm#PPM_example) format <br />
Inigo Quilez's [EVERYTHING](https://iquilezles.org/)
