# meshes.rs
Meshes.rs is a stupid adventure into computer graphics and linear algebra
this is somewhat of a prelude into a project I'm going to initate next year, 
diving into computer graphics, gpu acceleration, and glsl through vulkan. 
I want this to be without as many crates as possible, as of now I intend to only use the standard rust library
Many thanks to %⌵ᐂ⛉ for providing a lot of advice and guidance

## todo
- [ ] Reconfig threads so they A.) work, and B.) only use up to 15 threads (num of system threads)
- [ ] Turn the object struct into a few enums to have more efficient sdf
- [ ] Fix whatever is going on with more complicated object rendering
- [ ] Remove the stupid spherical rendering option. It is not useful. 
- [ ] generate all primitaves with object struct (currently only rect and tetrahedron)
- [ ] make way to convert point cloud to convex hull
- [ ] make a way to import .obj files
- [ ] progress bars
- [ ] egui, so we can have runtime stuff (dream is being able to watch rendering process "realtime")

## How does it work?
Wow thanks for asking. The core struct that makes this system work is the concept of a Vec3, 
or a three dimensional vector, pulled (fairly) directly from languages like GLSL. All it is, excluding the associated functions, 
is 3 floating point numbers, for the x, y, and z components. While math might call a vector one thing, and computer science another, 
I like to take the physics approach: "It's whatever I want it to be". In this system a Vec3 represents points in space, directions, arrows, rotations, etc.
With this can be constructed the next two inegral pieces: Triangles and rays.<br />
A triangle is simply a struct that contains 3 Vec3 cordinates for it's vertecies, and another for it's origin (oh, and a normal<sup>1</sup>). While there are plans for more sophisticated representations of 3d shapes, any possible 3d shape can be made from a *lot* of triangles, so this is sufficient for now. 
However for reasons explained when we discuss rays, we need to know the distance from an abritrary point to the surface of the triangle. This is called a SDF, or signed distance field. Do not ask me how it works please, but it does and I thank Inigo Quilez for this magic.<br />
A ray is a little more complicated. We can use them to do a great number of things, but the core concept remains the same: a ray finds the longest straight line possible from a point. While this could be done in a vaguely calculus like manner with extremely small steps until a ray has a distance of 0 (or close to it) to a face. 
However this solution sucks. It will often require a ton of unnecessary steps, where a single step can be safely made. The solution to this issue is called "raymarching", where starting at an arbitrary point in space you take the shortest distance to a face in the scene, set that as a radius, and then move that radius's value in the ray's direction.
This ensures that the ray will never pass through an object, but that it will go much, much faster then simple short steps.<br />
With these two tools, triangles can represent objects in a scene, and using a projection matrix, rays can be used to simulate light (and their bounces).<br />
But what is this about a projection matrix? The next step of this process is to render out an image. But how should this be done? As suggested by the Ray Marching In One Weekend book,
I found the `.ppm` file format to be most useful, because it allows for very easy, very simple direct writing of the color for each pixel. But the process of figuring out what color to be place is handled by the camera struct.<br />
Imagine a flat plane in 3d space. Now imagine a point somewhere above the center of it, and then a **bunch** of rays coming down to every point on that plane from that point. This is essentially how the camera operats. 
It takes a 1x1<sup>2</sup> ranging from (-0.5, 0.5) to (0.5, -0,5), and takes a pixel resolution to help distrobute a set of points across that plane. These points are already vectors for the origin, and thus are the direction (after accounting for global position, which these rays do not possess because they are centered at (0, 0))
of each ray when it is cast out into the scene. After this each ray returns a cast ray and an optional `Object` struct, holding the data of what it has hit. 

<br />
<br />
<sup>1</sup>alternatively, this has already been added and I'm either too lazy to fix it or haven't remembered about this to fix it.<br />
<sup>2</sup>as with the above note, there are plans to allow for changing the aspect ratio, however that might not be reflected here.<br />

### resources/citations
Michael Walczyk's [ray marching](https://michaelwalczyk.com/blog-ray-marching.html) blogpost <br />
Peter Shirley's [ray tracing in one weekend](https://raytracing.github.io/) series <br />
The ever radiant Sebastian Lague's [raytracing](https://www.youtube.com/watch?v=Qz0KTGYJtUk) and [raymarching](https://www.youtube.com/watch?v=Cp5WWtMoeKg) videos <br />
This stackoverflow [question](https://stackoverflow.com/questions/849211/shortest-distance-between-a-point-and-a-line-segment) and the [article](http://paulbourke.net/geometry/pointlineplane/) the answers seem to be based on <br />
Wikipedia -somehow managed to be the best resource- for the [ppm](https://en.wikipedia.org/wiki/Netpbm#PPM_example) format <br />
Inigo Quilez's [EVERYTHING](https://iquilezles.org/)
