use crate::mesh2::Mesh2;
use crate::trap_decomp::{Ray, TrapDecomp, Vertical};
use patina_geo::geo2::polygon2::Polygon2;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct MonotoneDecomp<'mesh> {
    mesh: &'mesh Mesh2,
}

impl<'mesh> MonotoneDecomp<'mesh> {
    pub fn new(mesh: &'mesh Mesh2) -> MonotoneDecomp<'mesh> {
        MonotoneDecomp { mesh }
    }
    pub fn build(self) -> Vec<Polygon2> {
        let traps = TrapDecomp::new(self.mesh).build();
        let mut builders = HashMap::<Vertical, Vec<Vertical>>::new();
        let mut monotones = vec![];
        for trap in traps {
            if trap.top_direction() && trap.bottom_direction() {
                if let Some(mut monotone) = builders.remove(trap.left()) {
                    monotone.push(trap.right().clone());
                    if trap.right().ray() == Ray::None {
                        monotones.push(monotone);
                    } else {
                        assert!(builders.insert(trap.right().clone(), monotone).is_none());
                    }
                } else {
                    assert_eq!(trap.left().ray(), Ray::None);
                    assert!(
                        builders
                            .insert(
                                trap.right().clone(),
                                vec![trap.left().clone(), trap.right().clone()],
                            )
                            .is_none(),
                        "inserting {:#?}",
                        trap
                    );
                }
            }
        }
        assert!(builders.is_empty());
        let mut polygons = vec![];
        for monotone in monotones {
            assert_eq!(monotone.first().unwrap().ray(), Ray::None);
            assert_eq!(monotone.last().unwrap().ray(), Ray::None);
            let mut top = vec![];
            let mut bottom = vec![];
            for vert in monotone {
                if vert.down() {
                    top.push(self.mesh.vertices()[vert.vertex()]);
                } else {
                    bottom.push(self.mesh.vertices()[vert.vertex()]);
                }
            }
            bottom.extend(top.into_iter().rev());
            polygons.push(Polygon2::new(bottom));
        }
        polygons
    }
}

#[test]
fn test_monotone() {
    for poly in Polygon2::test_cases() {
        let mut mesh = Mesh2::new(vec![], vec![]);
        mesh.add_polygon(&poly);
        let monotones = MonotoneDecomp::new(&mesh).build();
        let mut total = 0.0;
        for monotone in monotones {
            let area = monotone.signed_area();
            assert!(area >= 0.0);
            total += area;
        }
        let expected = poly.signed_area();
        assert!(
            (total - expected).abs() < 10e-10,
            "{:?} {:?}",
            total,
            expected
        );
    }
}
