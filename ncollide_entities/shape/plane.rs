//! Support mapping based Plane shape.
use std::any::Any;
use std::any::TypeId;
use std::marker::PhantomData;
use std::mem;
use na;
use na::Norm;
use inspection::{Repr, ReprDesc};
use math::Scalar;

/// SupportMap description of a plane.
#[derive(PartialEq, Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct Plane<N, V> {
    /// The plane normal.
    normal: V,
    params: PhantomData<N>
}


impl<N: Scalar, V: Norm<N>> Plane<N, V> {
    /// Builds a new plane from its center and its normal.
    #[inline]
    pub fn new(normal: V) -> Plane<N, V> {
        unsafe { Plane::new_normalized(na::normalize(&normal)) }
    }
}

impl<N, V> Plane<N, V> {
    /// Builds a new plane from its center and its normal.
    #[inline]
    pub unsafe fn new_normalized(normal: V) -> Plane<N, V> {
        Plane {
            normal: normal
        }
    }

    /// The plane normal.
    #[inline]
    pub fn normal(&self) -> &V {
        &self.normal
    }
}

impl<N, P, V, M> Repr<N, P, V, M> for Plane<N, V>
    where V: Send + Sync {
    #[inline(always)]
    fn repr(&self) -> ReprDesc<N, P, V, M> {
        unsafe {
            ReprDesc::new(
                TypeId::of::<Plane<N, V>>(),
                TypeId::of::<&Any>(),
                mem::transmute(self as &Any)
            )
        }
    }
}
