use na::Translate;
use bounding_volume::{HasAABB, AABB};
use math::{Scalar, Point, Vect, Isometry};
use shape::{Ball, Capsule, Compound, Cone, Convex, Cuboid, Cylinder, TriMesh, Polyline, Plane,
            Segment, Triangle};
use inspection::Repr;

impl<N, P, V, M> HasAABB<P, M> for Repr<N, P, V, M>
    where N: Scalar + 'static,
          P: Point<N, V> + 'static,
          V: Vect<N> + Translate<P> + 'static,
          M: Isometry<N, P, V> + 'static {
    #[inline]
    fn aabb(&self, m: &M) -> AABB<P> {
        let repr = self.repr();

        if let Some(b) = repr.downcast_ref::<Ball<N>>() {
            b.aabb(m)
        }
        else if let Some(c) = repr.downcast_ref::<Capsule<N>>() {
            c.aabb(m)
        }
        else if let Some(c) = repr.downcast_ref::<Compound<N, P, V, M>>() {
            c.aabb(m)
        }
        else if let Some(c) = repr.downcast_ref::<Cone<N>>() {
            c.aabb(m)
        }
        else if let Some(c) = repr.downcast_ref::<Convex<P>>() {
            c.aabb(m)
        }
        else if let Some(c) = repr.downcast_ref::<Cuboid<V>>() {
            c.aabb(m)
        }
        else if let Some(c) = repr.downcast_ref::<Cylinder<N>>() {
            c.aabb(m)
        }
        else if let Some(t) = repr.downcast_ref::<TriMesh<N, P, V>>() {
            t.aabb(m)
        }
        else if let Some(p) = repr.downcast_ref::<Polyline<N, P, V>>() {
            p.aabb(m)
        }
        else if let Some(p) = repr.downcast_ref::<Plane<V>>() {
            p.aabb(m)
        }
        else if let Some(s) = repr.downcast_ref::<Segment<P>>() {
            s.aabb(m)
        }
        else if let Some(t) = repr.downcast_ref::<Triangle<P>>() {
            t.aabb(m)
        }
        else {
            /*
             * XXX: dispatch by custom type.
             */
            unimplemented!()
        }
    }
}
