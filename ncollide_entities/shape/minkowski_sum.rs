//! Minkowski sum.

use std::marker::PhantomData;
use std::ops::{Index, IndexMut, Add, Sub, Mul, Div, Neg};
use na::{Dim, ApproxEq, Orig, PntAsVec, Axpy, Translate, NumPnt, NumVec, POrd,
         POrdering, ScalarSub, ScalarAdd, ScalarMul, ScalarDiv, FloatPnt, Bounded};
use na;
use shape::Reflection;
use math::{Scalar, Point, Vect};


/// Type of an implicit representation of the Configuration Space Obstacle
/// formed by two geometric objects.
pub type CSO<'a, N, M, G1, G2> = MinkowskiSum<'a, N, M, G1, Reflection<'a, N, G2>>;
pub type AnnotatedCSO<'a, N, M, G1, G2> = AnnotatedMinkowskiSum<'a, N, M, G1, Reflection<'a, N, G2>>;

/**
 * SupportMap representation of the Minkowski sum of two shapes.
 *
 * The only way to obtain the sum points is to use its support mapping
 * function.
 *
 *  - `G1`: type of the first object involved on the sum.
 *  - `G2`: type of the second object involved on the sum.
 */
#[derive(Debug)]
pub struct MinkowskiSum<'a, N, M: 'a, G1: ?Sized + 'a, G2: ?Sized + 'a> {
    m1: &'a M,
    g1: &'a G1,
    m2: &'a M,
    g2: &'a G2,
    params: PhantomData<N>
}

impl<'a, N, M, G1: ?Sized, G2: ?Sized> MinkowskiSum<'a, N, M, G1, G2> {
    /**
     * Builds the Minkowski sum of two shapes. Since the representation is
     * implicit, this is done in constant time.
     */
    #[inline]
    pub fn new(m1: &'a M, g1: &'a G1, m2: &'a M, g2: &'a G2) -> MinkowskiSum<'a, N, M, G1, G2> {
        MinkowskiSum { m1: m1, g1: g1, m2: m2, g2: g2 }
    }

    /// The transformation matrix of the first shape of this Minkowski Sum.
    #[inline]
    pub fn m1(&self) -> &'a M {
        self.m1
    }

    /// The transformation matrix of the second shape of this Minkowski Sum.
    #[inline]
    pub fn m2(&self) -> &'a M {
        self.m2
    }

    /// The first shape of this Minkowski Sum.
    #[inline]
    pub fn g1(&self) -> &'a G1 {
        self.g1
    }

    /// The second shape of this Minkowski Sum.
    #[inline]
    pub fn g2(&self) -> &'a G2 {
        self.g2
    }
}

/**
 * Same as the MinkowskiSum but with a support mapping which keeps track of the
 * original supports points from the two wrapped shapes.
 *
 * * `G1`: type of the first object involved on the sum.
 * * `G2`: type of the second object involved on the sum.
 */
#[derive(Debug)]
pub struct AnnotatedMinkowskiSum<'a, N, M: 'a, G1: ?Sized + 'a, G2: ?Sized + 'a> {
    m1: &'a M,
    g1: &'a G1,
    m2: &'a M,
    g2: &'a G2,
    params: PhantomData<N>
}

impl<'a, N, M, G1: ?Sized, G2: ?Sized> AnnotatedMinkowskiSum<'a, N, M, G1, G2> {
    /**
     * Builds the Minkowski sum of two shapes. Since the representation is
     * implicit, this is done in constant time.
     */
    #[inline]
    pub fn new(m1: &'a M, g1: &'a G1, m2: &'a M, g2: &'a G2) -> AnnotatedMinkowskiSum<'a, N, M, G1, G2> {
        AnnotatedMinkowskiSum { m1: m1, g1: g1, m2: m2, g2: g2 }
    }

    /// The transformation matrix of the first shape of this Minkowski Sum.
    #[inline]
    pub fn m1(&self) -> &'a M {
        self.m1
    }

    /// The transformation matrix of the second shape of this Minkowski Sum.
    #[inline]
    pub fn m2(&self) -> &'a M {
        self.m2
    }

    /// The first shape of this Minkowski Sum.
    #[inline]
    pub fn g1(&self) -> &'a G1 {
        self.g1
    }

    /// The second shape of this Minkowski Sum.
    #[inline]
    pub fn g2(&self) -> &'a G2 {
        self.g2
    }
}

// FIXME: AnnotatedPoint is not a good name.
// XXX: do not hide the documentation!
#[doc(hidden)]
#[derive(Clone, Copy, Debug, RustcEncodable, RustcDecodable)]
pub struct AnnotatedPoint<N, P, V> {
    orig1: P,
    orig2: P,
    point: P,
    params: PhantomData<(N,V)>
}

impl<N, P, V: Translate<P>> AnnotatedPoint<N, P, V> {
    #[doc(hidden)]
    #[inline]
    pub fn new(orig1: P, orig2: P, point: P) -> AnnotatedPoint<N, P, V> {
        AnnotatedPoint {
            orig1: orig1,
            orig2: orig2,
            point: point
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn point<'r>(&'r self) -> &'r P {
        &self.point
    }

    #[doc(hidden)]
    #[inline]
    pub fn orig1(&self) -> &P {
        &self.orig1
    }

    #[doc(hidden)]
    #[inline]
    pub fn orig2(&self) -> &P {
        &self.orig2
    }

    #[doc(hidden)]
    #[inline]
    pub fn translate_1(&mut self, t: &V) {
        self.orig1 = na::translate(t, &self.orig1);
        self.point = na::translate(t, &self.point);
    }

    #[doc(hidden)]
    #[inline]
    pub fn translate_2(&mut self, t: &V) {
        self.orig2 = na::translate(t, &self.orig2);
        self.point = na::translate(t, &self.point);
    }
}

impl<N: Scalar, P: PntAsVec<V>, V> PntAsVec<V> for AnnotatedPoint<N, P, V> {
    #[inline]
    fn to_vec(self) -> V {
        self.point.to_vec()
    }

    #[inline]
    fn as_vec<'a>(&'a self) -> &'a V {
        self.point.as_vec()
    }

    #[inline]
    fn set_coords(&mut self, _: V) {
        panic!(".set_coords is not implemented for annotated points.")
    }
}

impl<N: Scalar, P: Index<usize>, V> Index<usize> for AnnotatedPoint<N, P, V> {
    type Output = P::Output;

    #[inline]
    fn index(&self, i: &usize) -> &P::Output {
        &self.point[*i]
    }
}

impl<N: Scalar, P: IndexMut<usize>, V> IndexMut<usize> for AnnotatedPoint<N, P, V> {
    #[inline]
    fn index_mut(&mut self, _: &usize) -> &mut P::Output {
        unimplemented!()
    }
}

impl<N, P, V> POrd for AnnotatedPoint<N, P, V> {
    fn inf(&self, _: &AnnotatedPoint<N, P, V>) -> AnnotatedPoint<N, P, V> {
        unimplemented!()
    }

    fn sup(&self, _: &AnnotatedPoint<N, P, V>) -> AnnotatedPoint<N, P, V> {
        unimplemented!()
    }

    fn partial_cmp(&self, _: &AnnotatedPoint<N, P, V>) -> POrdering {
        unimplemented!()
    }
}

impl<N, P: Orig, V> Orig for AnnotatedPoint<N, P, V> {
    #[inline]
    fn orig() -> AnnotatedPoint<N, P, V> {
        AnnotatedPoint::new(na::orig(), na::orig(), na::orig())
    }

    #[inline]
    fn is_orig(&self) -> bool {
        self.point.is_orig()
    }
}


impl<N, P, V> Add<V> for AnnotatedPoint<N, P, V>
    where N: Scalar,
          P: Add<V, Output = P>,
          V: Copy + Mul<N, Output = V> {
    type Output = AnnotatedPoint<N, P, V>;

    #[inline]
    fn add(self, other: V) -> AnnotatedPoint<N, P, V> {
        let _0_5: N = na::cast(0.5f64);

        AnnotatedPoint::new(
            self.orig1 + other * _0_5,
            self.orig2 + other * _0_5,
            self.point + other
        )
    }
}

impl<N, P: Axpy<N>, V> Axpy<N> for AnnotatedPoint<N, P, V> {
    #[inline]
    fn axpy(&mut self, a: &N, x: &AnnotatedPoint<N, P, V>) {
        self.orig1.axpy(a, &x.orig1);
        self.orig2.axpy(a, &x.orig2);
        self.point.axpy(a, &x.point);
    }
}

impl<N, P: Sub<P>, V> Sub<AnnotatedPoint<N, P, V>> for AnnotatedPoint<N, P, V> {
    type Output = P::Output;
    #[inline]
    fn sub(self, other: AnnotatedPoint<N, P, V>) -> P::Output {
        self.point - other.point
    }
}


impl<N, P, V> ScalarSub<N> for AnnotatedPoint<N, P, V>
    where P: Point<N, V> {
    fn sub_s(&self, _: &N) -> AnnotatedPoint<N, P, V> {
        unimplemented!()
    }
}


impl<N, P, V> ScalarAdd<N> for AnnotatedPoint<N, P, V>
    where P: Point<N, V> {
    fn add_s(&self, _: &N) -> AnnotatedPoint<N, P, V> {
        unimplemented!()
    }
}


impl<N, P, V> ScalarMul<N> for AnnotatedPoint<N, P, V>
    where P: Point<N, V> {
    fn mul_s(&self, _: &N) -> AnnotatedPoint<N, P, V> {
        unimplemented!()
    }
}


impl<N, P, V> ScalarDiv<N> for AnnotatedPoint<N, P, V>
    where P: Point<N, V> {
    fn div_s(&self, _: &N) -> AnnotatedPoint<N, P, V> {
        unimplemented!()
    }
}

impl<N, P: Neg<Output = P>, V> Neg for AnnotatedPoint<N, P, V> {
    type Output = AnnotatedPoint<N, P, V>;

    #[inline]
    fn neg(self) -> AnnotatedPoint<N, P, V> {
        AnnotatedPoint::new(-self.orig1, -self.orig2, -self.point)
    }
}

impl<N, P: Dim, V> Dim for AnnotatedPoint<N, P, V> {
    #[inline]
    fn dim(_: Option<AnnotatedPoint<N, P, V>>) -> usize {
        na::dim::<P>()
    }
}

impl<N: Copy, P: Div<N, Output = P>, V> Div<N> for AnnotatedPoint<N, P, V> {
    type Output = AnnotatedPoint<N, P, V>;

    #[inline]
    fn div(self, n: N) -> AnnotatedPoint<N, P, V> {
        AnnotatedPoint::new(self.orig1 / n, self.orig2 / n, self.point / n)
    }
}

impl<N: Copy, P: Mul<N, Output = P>, V> Mul<N> for AnnotatedPoint<N, P, V> {
    type Output = AnnotatedPoint<N, P, V>;

    #[inline]
    fn mul(self, n: N) -> AnnotatedPoint<N, P, V> {
        AnnotatedPoint::new(self.orig1 * n, self.orig2 * n, self.point * n)
    }
}

impl<N, P: PartialEq, V> PartialEq for AnnotatedPoint<N, P, V> {
    #[inline]
    fn eq(&self, other: &AnnotatedPoint<N, P, V>) -> bool {
        self.point == other.point
    }

    #[inline]
    fn ne(&self, other: &AnnotatedPoint<N, P, V>) -> bool {
        self.point != other.point
    }
}

impl<N, P, V> ApproxEq<N> for AnnotatedPoint<N, P, V>
    where N: Scalar,
          P: ApproxEq<N> {
    #[inline]
    fn approx_epsilon(_: Option<AnnotatedPoint<N, P, V>>) -> N {
        ApproxEq::approx_epsilon(None::<N>)
    }

    #[inline]
    fn approx_eq_eps(&self, other: &AnnotatedPoint<N, P, V>, eps: &N) -> bool {
        self.point.approx_eq_eps(&other.point, eps)
    }

    #[inline]
    fn approx_ulps(_: Option<AnnotatedPoint<N, P, V>>) -> u32 {
        ApproxEq::approx_ulps(None::<N>)
    }

    #[inline]
    fn approx_eq_ulps(&self, other: &AnnotatedPoint<N, P, V>, ulps: u32) -> bool {
        self.point.approx_eq_ulps(&other.point, ulps)
    }
}

impl<N, P, V> Bounded for AnnotatedPoint<N, P, V> {
    fn min_value() -> AnnotatedPoint<N, P, V> {
        unimplemented!()
    }

    fn max_value() -> AnnotatedPoint<N, P, V> {
        unimplemented!()
    }
}

impl<N, P, V> NumPnt<N, V> for AnnotatedPoint<N, P, V>
    where N: Scalar,
          P: NumPnt<N, V>,
          V: Copy + NumVec<N> {
}

impl<N, P, V> FloatPnt<N, V> for AnnotatedPoint<N, P, V>
    where N: Scalar,
          P: Point<N, V>,
          V: Vect<N> {
}

impl<N, P, V> Point<N, V> for AnnotatedPoint<N, P, V>
    where N: Scalar,
          P: Point<N, V>,
          V: Vect<N> {
}
