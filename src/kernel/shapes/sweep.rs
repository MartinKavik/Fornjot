use nalgebra::vector;
use parry3d_f64::{bounding_volume::AABB, math::Isometry, shape::Triangle};

use crate::{
    kernel::{edges::Edges, faces::Faces, Shape},
    math::Point,
};

impl Shape for fj::Sweep {
    fn bounding_volume(&self) -> AABB {
        let mut aabb = self.shape.bounding_volume();
        aabb.maxs.z = self.length;
        aabb
    }

    fn faces(&self, tolerance: f64) -> Faces {
        let mut triangles = Vec::new();

        let original_triangles = self.shape.faces(tolerance);

        // Bottom face
        triangles.extend(
            original_triangles.triangles().iter().map(|triangle| {
                Triangle::new(triangle.a, triangle.c, triangle.b)
            }),
        );

        // Top face
        triangles.extend(original_triangles.triangles().iter().map(
            |triangle| {
                triangle.transformed(&Isometry::translation(
                    0.0,
                    0.0,
                    self.length,
                ))
            },
        ));

        let segments = self.shape.edges().approx_segments(tolerance);

        let mut quads = Vec::new();
        for segment in segments {
            let [v0, v1] = segment.0;
            let [v3, v2] = segment.translate(vector![0.0, 0.0, self.length]).0;

            quads.push([v0, v1, v2, v3]);
        }

        for [v0, v1, v2, v3] in quads {
            triangles.push([v0, v1, v2].into());
            triangles.push([v0, v2, v3].into());
        }

        Faces::Triangles(triangles)
    }

    fn edges(&self) -> Edges {
        // TASK: Implement.
        todo!()
    }

    fn vertices(&self) -> Vec<Point> {
        // TASK: Implement.
        todo!()
    }
}