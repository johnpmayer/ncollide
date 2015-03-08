//! Support mapping based Cuboid shape.

use std::marker::PhantomData;
use na::Iterable;
use na;
use math::Scalar;

/// Shape of a box.
#[derive(PartialEq, Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct Cuboid<N, V> {
    half_extents: V,
    params: PhantomData<N>
}


impl<N, V> Cuboid<N, V>
    where N: Scalar,
          V: Iterable<N> {
    /// Creates a new box from its half-extents. Half-extents are the box half-width along each
    /// axis. Each half-extent must be greater than 0.04.
    #[inline]
    pub fn new(half_extents: V) -> Cuboid<N, V> {
        assert!(half_extents.iter().all(|e| *e >= na::zero()));

        Cuboid {
            half_extents: half_extents
        }
    }
}

impl<N, V> Cuboid<N, V> {
    /// The half-extents of this box. Half-extents are the box half-width along each axis.
    #[inline]
    pub fn half_extents(&self) -> &V {
        &self.half_extents
    }
}
