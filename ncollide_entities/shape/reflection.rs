use std::marker::PhantomData;

/// SupportMap representation of the reflection of a shape.
///
/// A reflection is obtained with the central symmetry with regard to the origin.
#[derive(Debug)]
pub struct Reflection<'a, N, G: ?Sized + 'a> {
    shape: &'a G,
    params: PhantomData<N>
}

impl<'a, N, G: ?Sized> Reflection<'a, N, G> {
    /// Build the reflection of a shape. Since the representation is implicit,
    /// the reflection computation is done in constant time.
    #[inline]
    pub fn new(shape: &'a G) -> Reflection<'a, N, G> {
        Reflection { shape: shape }
    }

    /// The reflected shape.
    #[inline]
    pub fn shape(&self) -> &'a G {
        self.shape
    }
}
