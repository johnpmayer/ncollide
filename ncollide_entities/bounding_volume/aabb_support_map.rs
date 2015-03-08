use na::{Rotate, Transform};
use bounding_volume::{HasAABB, AABB};
use bounding_volume;
use shape::{Cone, Cylinder, Capsule};
use shape::{Triangle, Segment};
use math::{Scalar, Point, Vect};


impl<N, P, V, M> HasAABB<N, P, V, M> for Cone<N>
    where N: Scalar,
          P: Point<N, V>,
          V: Vect<N>,
          M: Transform<P> + Rotate<V> {
    #[inline]
    fn aabb(&self, m: &M) -> AABB<N, P, V> {
        bounding_volume::implicit_shape_aabb(m, self)
    }
}


impl<N, P, V, M> HasAABB<N, P, V, M> for Cylinder<N>
    where N: Scalar,
          P: Point<N, V>,
          V: Vect<N>,
          M: Transform<P> + Rotate<V> {
    #[inline]
    fn aabb(&self, m: &M) -> AABB<N, P, V> {
        bounding_volume::implicit_shape_aabb(m, self)
    }
}


impl<N, P, V, M> HasAABB<N, P, V, M> for Capsule<N>
    where N: Scalar,
          P: Point<N, V>,
          V: Vect<N>,
          M: Transform<P> + Rotate<V> {
    #[inline]
    fn aabb(&self, m: &M) -> AABB<N, P, V> {
        bounding_volume::implicit_shape_aabb(m, self)
    }
}


impl<N, P, V, M> HasAABB<N, P, V, M> for Triangle<N, P>
    where N: Scalar,
          P: Point<N, V>,
          V: Vect<N>,
          M: Transform<P> + Rotate<V> {
    #[inline]
    fn aabb(&self, m: &M) -> AABB<N, P, V> {
        // FIXME: optimize that
        bounding_volume::implicit_shape_aabb(m, self)
    }
}


impl<N, P, V, M> HasAABB<N, P, V, M> for Segment<N, P>
    where N: Scalar,
          P: Point<N, V>,
          V: Vect<N>,
          M: Transform<P> + Rotate<V> {
    #[inline]
    fn aabb(&self, m: &M) -> AABB<N, P, V> {
        // FIXME: optimize that
        bounding_volume::implicit_shape_aabb(m, self)
    }
}
