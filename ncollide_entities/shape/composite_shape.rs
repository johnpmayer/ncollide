use std::ops::Mul;
use na;
use math::{Scalar, Point, Vect, Isometry};
use partitioning::BVT;
use bounding_volume::AABB;
use shape::{Compound, TriMesh, Polyline};
use inspection::Repr;

/// Trait implemented by shapes composed of multiple simpler shapes.
///
/// A composite shape is composed of several shapes. Typically, it is a convex decomposition of
/// a concave shape.
pub trait CompositeShape<N, P, V, M> {
    /// Applies a function to each sub-shape of this concave shape.
    fn map_part_at(&self, usize, &mut FnMut(&M, &Repr<N, P, V, M>));
    /// Applies a transformation matrix and a function to each sub-shape of this concave
    /// shape.
    fn map_transformed_part_at(&self, m: &M, usize, &mut FnMut(&M, &Repr<N, P, V, M>));

    // FIXME: the following two methods really are not generic enough.
    /// Gets the AABB of the shape identified by the index `i`.
    fn aabb_at(&self, i: usize) -> &AABB<P>;
    /// Gets the acceleration structure of the concave shape.
    fn bvt(&self) -> &BVT<usize, AABB<P>>;
}

impl<N, P, V, M> CompositeShape<N, P, V, M> for Compound<N, P, V, M>
    where M: Copy + Mul<M, Output = M> {
    #[inline(always)]
    fn map_part_at(&self, i: usize, f: &mut FnMut(&M, &Repr<N, P, V, M>)) {
        let &(ref m, ref g) = &self.shapes()[i];

        f(m, &***g)
    }

    #[inline(always)]
    fn map_transformed_part_at(&self, m: &M, i: usize, f: &mut FnMut(&M, &Repr<N, P, V, M>)) {
        let elt = &self.shapes()[i];

        f(&(*m * elt.0), &**elt.1)
    }

    #[inline]
    fn aabb_at(&self, i: usize) -> &AABB<P> {
        &self.bounding_volumes()[i]
    }

    #[inline]
    fn bvt(&self) -> &BVT<usize, AABB<P>> {
        self.bvt()
    }
}

impl<N, P, V, M> CompositeShape<N, P, V, M> for TriMesh<N, P, V>
    where N: Scalar + 'static,
          P: Point<N, V> + 'static,
          V: Vect<N> + 'static,
          M: Isometry<N, P, V> + 'static {
    #[inline(always)]
    fn map_part_at(&self, i: usize, f: &mut FnMut(&M, &Repr<N, P, V, M>)) {
        let one: M = na::one();

        self.map_transformed_part_at(&one, i, f)
    }

    #[inline(always)]
    fn map_transformed_part_at(&self, m: &M, i: usize, f: &mut FnMut(&M, &Repr<N, P, V, M>)) {
        let element = self.triangle_at(i);

        f(m, &element)
    }

    #[inline]
    fn aabb_at(&self, i: usize) -> &AABB<P> {
        &self.bounding_volumes()[i]
    }

    #[inline]
    fn bvt(&self) -> &BVT<usize, AABB<P>> {
        self.bvt()
    }
}

impl<N, P, V, M> CompositeShape<N, P, V, M> for Polyline<N, P, V>
    where N: Scalar + 'static,
          P: Point<N, V> + 'static,
          V: Vect<N> + 'static,
          M: Isometry<N, P, V> + 'static {
    #[inline(always)]
    fn map_part_at(&self, i: usize, f: &mut FnMut(&M, &Repr<N, P, V, M>)) {
        let one: M = na::one();

        self.map_transformed_part_at(&one, i, f)
    }

    #[inline(always)]
    fn map_transformed_part_at(&self, m: &M, i: usize, f: &mut FnMut(&M, &Repr<N, P, V, M>)) {
        let element = self.segment_at(i);

        f(m, &element)
    }

    #[inline]
    fn aabb_at(&self, i: usize) -> &AABB<P> {
        &self.bounding_volumes()[i]
    }

    #[inline]
    fn bvt(&self) -> &BVT<usize, AABB<P>> {
        self.bvt()
    }
}
