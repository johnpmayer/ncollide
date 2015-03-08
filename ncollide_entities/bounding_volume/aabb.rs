//! Axis Aligned Bounding Box.

use std::marker::{PhantomData, PhantomFn};
use std::ops::Neg;
use na::{Translation, POrd, Translate, Bounded};
use na;
use bounding_volume::BoundingVolume;
use math::{Scalar, Point, Vect};

/// Trait of objects that can be bounded by an AABB.
pub trait HasAABB<N, P, V, M>: PhantomFn<(N, V)> {
    /// The object’s AABB.
    fn aabb(&self, &M) -> AABB<N, P, V>;
}

/// An Axis Aligned Bounding Box.
#[derive(Debug, PartialEq, Clone, RustcEncodable, RustcDecodable)]
pub struct AABB<N, P, V> {
    mins: P,
    maxs: P,
    params: PhantomData<(N,V)>
}

impl<N, P: POrd, V> AABB<N, P, V> {
    /// Creates a new AABB.
    ///
    /// # Arguments:
    ///   * `mins` - position of the point with the smallest coordinates.
    ///   * `maxs` - position of the point with the highest coordinates. Each component of `mins`
    ///   must be smaller than the related components of `maxs`.
    pub fn new(mins: P, maxs: P) -> AABB<N, P, V> {
        assert!(na::partial_le(&mins, &maxs));

        AABB {
            mins: mins,
            maxs: maxs
        }
    }
}

impl<N, P: Neg<Output = P> + POrd + Bounded, V> AABB<N, P, V> {
    /// Creates an invalid AABB with:
    /// * `mins = Bounded::max_value()`
    /// * `maxs = Bounded::max_value()`.
    /// This is useful to build aabb using merges.
    pub fn new_invalid() -> AABB<N, P, V> {
        let _max: P = Bounded::max_value();
        AABB {
            mins: Bounded::max_value(),
            maxs: -_max,
        }
    }
}

impl<N, P, V> AABB<N, P, V> {
    /// Reference to the AABB point with the smallest components along each axis.
    #[inline]
    pub fn mins(&self) -> &P {
        &self.mins
    }

    /// Reference to the AABB point with the biggest components along each axis.
    #[inline]
    pub fn maxs(&self) -> &P {
        &self.maxs
    }
}


impl<N, P, V> AABB<N, P, V>
    where N: Scalar,
          P: Point<N, V>,
          V: Vect<N> {
    /// The center of this AABB.
    #[inline]
    pub fn center(&self) -> P {
        na::center(&self.mins, &self.maxs)
    }

    /// The half extents of this AABB.
    #[inline]
    pub fn half_extents(&self) -> V {
        (self.maxs - self.mins) / na::cast(2.0f64)
    }
}


impl<N, P, V> BoundingVolume<N> for AABB<N, P, V>
    where N: Scalar,
          P: Point<N, V> {
    #[inline]
    fn intersects(&self, other: &AABB<N, P, V>) -> bool {
        na::partial_le(&self.mins, &other.maxs) &&
        na::partial_ge(&self.maxs, &other.mins)
    }

    #[inline]
    fn contains(&self, other: &AABB<N, P, V>) -> bool {
        na::partial_le(&self.mins, &other.mins) &&
        na::partial_ge(&self.maxs, &other.maxs)
    }

    #[inline]
    fn merge(&mut self, other: &AABB<N, P, V>) {
        self.mins = na::inf(&self.mins, &other.mins);
        self.maxs = na::sup(&self.maxs, &other.maxs);
    }

    #[inline]
    fn merged(&self, other: &AABB<N, P, V>) -> AABB<N, P, V> {
        AABB {
            mins: na::inf(&self.mins, &other.mins),
            maxs: na::sup(&self.maxs, &other.maxs)
        }
    }

    #[inline]
    fn loosen(&mut self, amount: N) {
        assert!(amount >= na::zero(), "The loosening margin must be positive.");
        self.mins = self.mins.sub_s(&amount);
        self.maxs = self.maxs.add_s(&amount);
    }

    #[inline]
    fn loosened(&self, amount: N) -> AABB<N, P, V> {
        assert!(amount >= na::zero(), "The loosening margin must be positive.");
        AABB {
            mins: self.mins.sub_s(&amount),
            maxs: self.maxs.add_s(&amount)
        }
    }

    #[inline]
    fn tighten(&mut self, amount: N) {
        assert!(amount >= na::zero(), "The tightening margin must be positive.");
        self.mins = self.mins.add_s(&amount);
        self.maxs = self.maxs.sub_s(&amount);
        assert!(na::partial_le(&self.mins, &self.maxs), "The tightening margin is to large.");
    }

    #[inline]
    fn tightened(&self, amount: N) -> AABB<N, P, V> {
        assert!(amount >= na::zero(), "The tightening margin must be positive.");

        AABB::new(self.mins.add_s(&amount), self.maxs.sub_s(&amount))
    }
}


impl<N, P, V> Translation<V> for AABB<N, P, V>
    where N: Scalar,
          P: Point<N, V>,
          V: Vect<N> + Translate<P> {
    #[inline]
    fn translation(&self) -> V {
        na::center(&self.mins, &self.maxs).to_vec()
    }

    #[inline]
    fn inv_translation(&self) -> V {
        -self.translation()
    }

    #[inline]
    fn append_translation_mut(&mut self, dv: &V) {
        self.mins = self.mins + *dv;
        self.maxs = self.maxs + *dv;
    }

    #[inline]
    fn append_translation(&self, dv: &V) -> AABB<N, P, V> {
        AABB::new(self.mins + *dv, self.maxs + *dv)
    }

    #[inline]
    fn prepend_translation_mut(&mut self, dv: &V) {
        self.append_translation_mut(dv)
    }

    #[inline]
    fn prepend_translation(&self, dv: &V) -> AABB<N, P, V> {
        self.append_translation(dv)
    }

    #[inline]
    fn set_translation(&mut self, v: V) {
        let center = self.translation();
        let total_translation = center + v;

        self.mins = na::inv_translate(&total_translation, &self.mins);
        self.maxs = na::inv_translate(&total_translation, &self.maxs);
    }
}
