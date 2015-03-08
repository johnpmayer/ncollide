use na::Bounded;
use na;
use bounding_volume::{HasBoundingSphere, BoundingSphere};
use shape::Plane;
use math::{Scalar, Point, Vect};


impl<N, P, V, M> HasBoundingSphere<N, P, V, M> for Plane<N, V>
    where N: Scalar,
          P: Point<N, V>,
          V: Vect<N> {
    #[inline]
    fn bounding_sphere(&self, _: &M) -> BoundingSphere<N, P, V> {
        let center = na::orig();
        let radius = Bounded::max_value(); // FIXME: is this a good idea?

        BoundingSphere::new(center, radius)
    }
}
