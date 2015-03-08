use na::Translate;
use na;
use bounding_volume::{HasBoundingSphere, BoundingSphere};
use shape::Cuboid;
use math::{Scalar, Point, Vect};



impl<N, P, V, M> HasBoundingSphere<N, P, V, M> for Cuboid<N, V>
    where N: Scalar,
          P: Point<N, V>,
          V: Vect<N>,
          M: Translate<P> {
    #[inline]
    fn bounding_sphere(&self, m: &M) -> BoundingSphere<N, P, V> {
        let center = m.translate(&na::orig::<P>());
        let radius = na::norm(self.half_extents());

        BoundingSphere::new(center, radius)
    }
}
