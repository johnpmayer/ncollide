use std::marker::PhantomData;

#[derive(PartialEq, Debug, Clone, RustcEncodable, RustcDecodable)]
/// The implicit convex hull of a set of points.
pub struct Convex<N, P> {
    points: Vec<P>,
    params: PhantomData<N>
}

impl<N, P> Convex<N, P> {
    /// Creates a polytope from a set of point.
    ///
    /// The set of point as not assumed to form a convex polytope.
    #[inline]
    pub fn new(points: Vec<P>) -> Convex<N, P> {
        Convex {
            points: points
        }
    }
}

impl<N, P> Convex<N, P> {
    /// The list of points of this convex polytope.
    #[inline]
    pub fn points(&self) -> &[P] { // FIXME: naming: `points` vs. `points`?
        self.points.as_slice()
    }
}
