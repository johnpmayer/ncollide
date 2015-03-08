use std::ops::Neg;
use na::{AbsoluteRotate, Translate};
use na;
use bounding_volume::{HasAABB, AABB};
use shape::Cuboid;
use math::{Scalar, Point};


impl<N, P, V: Clone, M> HasAABB<N, P, V, M> for Cuboid<N, V>
    where N: Scalar,
          P: Point<N, V>,
          V: Neg<Output = V>,
          M: Translate<P> + AbsoluteRotate<V> {
    #[inline]
    fn aabb(&self, m: &M) -> AABB<N, P, V> {
        let center          = m.translate(&na::orig());
        let ws_half_extents = m.absolute_rotate(self.half_extents());

        AABB::new(center + -ws_half_extents.clone(), center + ws_half_extents)
    }
}
