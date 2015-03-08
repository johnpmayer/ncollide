//! Definition of the segment shape.

use na::{Dim, Pnt2};
use na;
use shape::BaseMeshElement;
use std::marker::PhantomData;

/// A segment shape.
#[derive(PartialEq, Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct Segment<N, P> {
    a: P,
    b: P,
    params: PhantomData<N>
}

impl<N, P: Dim> Segment<N, P> {
    /// Creates a new segment from two points.
    #[inline]
    pub fn new(a: P, b: P) -> Segment<N, P> {
        assert!(na::dim::<P>() > 1);

        Segment {
            a: a,
            b: b
        }
    }
}

impl<N, P> Segment<N, P> {
    /// The first point of this segment.
    #[inline]
    pub fn a(&self) -> &P {
        &self.a
    }

    /// The second point of this segment.
    #[inline]
    pub fn b(&self) -> &P {
        &self.b
    }
}

impl<N, P: Dim + Copy> BaseMeshElement<Pnt2<usize>, P> for Segment<N, P> {
    #[inline]
    fn new_with_vertices_and_indices(vs: &[P], is: &Pnt2<usize>) -> Segment<N, P> {
        Segment::new(vs[is.x], vs[is.y])
    }
}
