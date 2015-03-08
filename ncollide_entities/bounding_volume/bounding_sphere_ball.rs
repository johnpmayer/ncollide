use na::Translate;
use na;
use bounding_volume::{HasBoundingSphere, BoundingSphere};
use shape::Ball;
use math::{Scalar, Point};



impl<N, P, V, M> HasBoundingSphere<N, P, V, M> for Ball<N>
    where N: Scalar,
          P: Point<N, V>,
          M: Translate<P> {
    #[inline]
    fn bounding_sphere(&self, m: &M) -> BoundingSphere<N, P, V> {
        let center = m.translate(&na::orig());
        let radius = self.radius();

        BoundingSphere::new(center, radius)
    }
}
