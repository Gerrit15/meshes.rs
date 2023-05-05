# meshes.rs
Meshes.rs is a stupid adventure into computer graphics and linear algebra

## todo
- [x] create vector3 struct 
- [x] create triangle struct
- [x] create object struct
- [x] be able to transform, add, multiply, rotate, etc said structs 
- [x] create ray struct, with raymarching
- [x] output to ppm
- [ ] color, whether in the form of vec3 methods or unique structs
- [ ] refactor many parts into traits, such as rotation
- [ ] generate all primitaves with object struct (currently only rect and tetrahedron)
- [ ] impl addition, subtraction, etc for object
- [ ] make way to convert point cloud to convex hull
- [ ] make a way to import .obj files
- [ ] progress bars
- [ ] egui, so we can have runtime stuff
- [ ] multithread where possibile
- [ ] Reconfig sdf so the values are stored when triangle is created

## current issues
currently it appears that either the projection matrix is borked, the function to 
get distance from a point to a face is borked, or the output is borked (unlikely)

### resources/citations
Michael Walczyk's [ray marching](https://michaelwalczyk.com/blog-ray-marching.html) blogpost <br />
Peter Shirley's [ray tracing in one weekend](https://raytracing.github.io/) series <br />
The ever radiant Sebastian Lague's [raytracing](https://www.youtube.com/watch?v=Qz0KTGYJtUk) and [raymarching](https://www.youtube.com/watch?v=Cp5WWtMoeKg) videos <br />
This stackoverflow [question](https://stackoverflow.com/questions/849211/shortest-distance-between-a-point-and-a-line-segment) and the [article](http://paulbourke.net/geometry/pointlineplane/) the answers seem to be based on <br />
Wikipedia -somehow managed to be the best resource- for the [ppm](https://en.wikipedia.org/wiki/Netpbm#PPM_example) format <br />

