use na::Translate;
use na;
use bounding_volume::{HasAABB, AABB};
use shape::Ball;
use math::{Scalar, Point};

/// Computes the Axis-Aligned Bounding Box of a ball.
#[inline]
pub fn ball_aabb<N, P, V>(center: &P, radius: N) -> AABB<N, P, V>
    where N: Scalar,
          P: Point<N, V> {
    AABB::new(center.sub_s(&radius), center.add_s(&radius))
}


impl<N, P, V, M> HasAABB<N, P, V, M> for Ball<N>
    where N: Scalar,
          P: Point<N, V>,
          M: Translate<P> {
    #[inline]
    fn aabb(&self, m: &M) -> AABB<N, P, V> {
        ball_aabb(&m.translate(&na::orig()), self.radius())
    }
}
