use bounding_volume::BoundingVolume;
use partitioning::BVTVisitor;
use std::marker::PhantomData;

/// Bounding Volume Tree visitor collecting interferences with a given bounding volume.
pub struct BoundingVolumeInterferencesCollector<'a, N, B: 'a, BV: 'a> {
    bv:        &'a BV,
    collector: &'a mut Vec<B>,
    params:    PhantomData<N>
}


impl<'a, N, B, BV> BoundingVolumeInterferencesCollector<'a, N, B, BV> {
    /// Creates a new `BoundingVolumeInterferencesCollector`.
    #[inline]
    pub fn new(bv: &'a BV, buffer: &'a mut Vec<B>) -> BoundingVolumeInterferencesCollector<'a, N, B, BV> {
        BoundingVolumeInterferencesCollector {
            bv:        bv,
            collector: buffer
        }
    }
}


impl<'a, N, B: Clone, BV: BoundingVolume<N>> BVTVisitor<B, BV> for BoundingVolumeInterferencesCollector<'a, N, B, BV> {
    #[inline]
    fn visit_internal(&mut self, bv: &BV) -> bool {
        bv.intersects(self.bv)
    }

    #[inline]
    fn visit_leaf(&mut self, b: &B, bv: &BV) {
        if (self.bv as *const BV) != (bv as *const BV) && bv.intersects(self.bv) {
            self.collector.push(b.clone())
        }
    }
}
