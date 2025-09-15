This ray tracer is based off of Peter Shirley's Raytracing in One Weekend book.

It implements multiple materials, including both smooth and rough metalics, lambertian diffuse, and clear refractives like glass.

Currenly the only supported geometery are spheres. I will need to implement bounding volume optimizations to make arbitrary triangles run fast.
The current setup is prepared to add more geometry types through dynamic dispatch, using some techniques adapted from the Bevy library's Component implementation to reduce pointer indirection. 

