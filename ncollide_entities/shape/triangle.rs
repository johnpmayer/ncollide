//! Definition of the triangle shape.

use na::{Dim, Pnt3};
use na;
use shape::BaseMeshElement;
use std::marker::PhantomData;

/// A triangle shape.
#[derive(PartialEq, Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct Triangle<N, P> {
    a: P,
    b: P,
    c: P,
    params: PhantomData<N>
}

impl<N, P: Dim> Triangle<N, P> {
    /// Creates a triangle from three points.
    #[inline]
    pub fn new(a: P, b: P, c: P) -> Triangle<N, P> {
        assert!(na::dim::<P>() > 1);

        Triangle {
            a: a,
            b: b,
            c: c
        }
    }
}

impl<N, P> Triangle<N, P> {
    /// The fist point of this triangle.
    #[inline]
    pub fn a(&self) -> &P {
        &self.a
    }

    /// The second point of this triangle.
    #[inline]
    pub fn b(&self) -> &P {
        &self.b
    }

    /// The third point of this triangle.
    #[inline]
    pub fn c(&self) -> &P {
        &self.c
    }
}

impl<N, P: Copy + Dim> BaseMeshElement<Pnt3<usize>, P> for Triangle<N, P> {
    #[inline]
    fn new_with_vertices_and_indices(vs: &[P], is: &Pnt3<usize>) -> Triangle<N, P> {
        Triangle::new(vs[is.x], vs[is.y], vs[is.z])
    }
}
