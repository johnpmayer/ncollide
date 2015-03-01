use na::{Transform, Translate};
use bounding_volume::{BoundingVolume, BoundingSphere, HasBoundingSphere};
use shape::Compound;
use math::{Scalar, Point, Vect, Isometry};


impl<N, P, V, M, M2> HasBoundingSphere<N, P, M2> for Compound<N, P, V, M>
    where N:  Scalar + 'static,
          P:  Point<N, V> + 'static,
          V:  Vect<N> + Translate<P> + 'static,
          M:  Isometry<N, P, V> + 'static,
          M2: Transform<P> + Translate<P> + 'static {
    #[inline]
    fn bounding_sphere(&self, m: &M2) -> BoundingSphere<N, P> {
        let shapes = self.shapes();

        let mut res = shapes[0].1.bounding_sphere(&shapes[0].0);

        for &(ref t, ref s) in shapes.slice_from(1).iter() {
            res.merge(&s.bounding_sphere(t));
        }

        BoundingSphere::new(m.transform(res.center()), res.radius())
    }
}
