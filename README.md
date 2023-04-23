# meshes.rs
Meshes.rs is a stupid adventure into computer graphics and linear algebra

## todo
- [x] create vector3 struct 
- [x] create triangle struct
- [x] create object struct
- [x] be able to transform, add, multiply, rotate, etc said structs 
- [x] create ray struct
- [ ] impliment a raymarching function to ray struct 
- [ ] color, whether in the form of vec3 methods or unique structs
- [ ] refactor many parts into traits, such as rotation
- [ ] generate all primitaves with object struct (currently only rect and tetrahedron)
- [ ] triangulation functioni

## current issues
right now, we're having a lot of issues making the ray marching work.

### resources/citations
Michael Walczyk's [ray marching](https://michaelwalczyk.com/blog-ray-marching.html) blogpost
Peter Shirley's [ray tracing in one weekend](https://raytracing.github.io/) series
The ever radiant Sebastian Lague's [raytracing](https://www.youtube.com/watch?v=Qz0KTGYJtUk) and [raymarching](https://www.youtube.com/watch?v=Cp5WWtMoeKg) videos
This stackoverflow [question](https://stackoverflow.com/questions/849211/shortest-distance-between-a-point-and-a-line-segment) and the [article](http://paulbourke.net/geometry/pointlineplane/) the answers seem to be based on
