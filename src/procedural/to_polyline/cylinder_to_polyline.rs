use na::{Pnt2, Vec2};
use na;
use shape::Cylinder2;
use procedural::{ToPolyline, Polyline2};
use procedural;
use math::Scalar;

impl<N: Scalar> ToPolyline<N, Pnt2<N>, Vec2<N>, ()> for Cylinder2<N> {
    fn to_polyline(&self, _: ()) -> Polyline2<N> {
        let _2: N = na::cast(2.0f64);

        procedural::rectangle(&Vec2::new(self.radius() * _2, self.half_height() * _2))
    }
}
