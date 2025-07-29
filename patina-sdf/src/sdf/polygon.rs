use crate::sdf::leaf::{SdfLeaf, SdfLeafImpl};
use crate::sdf::{AsSdf, Sdf};
use itertools::Itertools;
use patina_geo::geo2::polygon2::Polygon2;
use patina_scalar::Scalar;
use patina_vec::vec2::{Vec2, Vector2};
use patina_vec::vec3::Vector3;

impl SdfLeafImpl<2> for Polygon2 {
    fn evaluate<T: Scalar>(&self, p: Vector2<T>) -> T {
        let mut sd = T::from_f64(f64::MAX);
        let mut sign = T::from_f64(1.0);
        for (&v1, &v2) in self.points().iter().circular_tuple_windows() {
            let e = (v2 - v1).into_scalars::<T>();
            let d = p.clone() - v1.into_scalars();
            let v1 = v1.into_scalars::<T>();
            let v2 = v2.into_scalars::<T>();
            let proj = (d.clone().dot(e.clone()) / e.clone().dot(e.clone()))
                .minimum(T::from_f64(1.0))
                .maximum(T::from_f64(0.0));
            let disp = d.clone() - e.clone() * proj.clone();
            sd = sd.minimum(disp.clone().dot(disp.clone()));
            let a1 = p.y() - v1.y();
            let a2 = v2.y() - p.y();
            let area = e.cross(d);
            sign = a1.clone().piecewise(
                a2.clone().piecewise(area.clone().sign(), T::from_f64(1.0)),
                a2.clone().piecewise(T::from_f64(1.0), -area.clone().sign()),
            ) * sign;
        }
        let result = sign * sd.sqrt();
        result
    }
}

impl AsSdf<2> for Polygon2 {
    fn as_sdf(&self) -> Sdf<2> {
        Sdf::new(SdfLeaf::new(self.clone()))
    }
}

#[test]
fn test() {
    let poly = Polygon2::new(vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 1.0),
    ]);
    let sdf = poly.as_sdf();

    assert_eq!(sdf.evaluate(Vec2::new(0.0, 0.0)), 0.0);
    assert_eq!(sdf.evaluate(Vec2::new(1.0, 0.0)), 0.0);
    assert_eq!(sdf.evaluate(Vec2::new(0.0, 1.0)), 0.0);
    assert_eq!(sdf.evaluate(Vec2::new(0.1, 0.2)), -0.1);
    assert_eq!(sdf.evaluate(Vec2::new(0.2, 0.1)), -0.1);
    assert_eq!(sdf.evaluate(Vec2::new(-3.0, -4.0)), 5.0);
    assert_eq!(sdf.evaluate(Vec2::new(1.0, 1.0)), 2.0.sqrt() / 2.0);
}
