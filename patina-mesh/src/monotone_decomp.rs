use crate::edge_mesh2::EdgeMesh2;
use crate::trap_decomp::{Ray, TrapDecomp, Vertical};
use patina_geo::geo2::polygon2::Polygon2;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum MonoSide {
    Up,
    Down,
    Both,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct MonoVertex {
    vertex: usize,
    side: MonoSide,
}

pub struct MonotoneDecomp<'mesh> {
    mesh: &'mesh EdgeMesh2,
}

impl MonoVertex {
    pub fn vertex(&self) -> usize {
        self.vertex
    }
    pub fn side(&self) -> MonoSide {
        self.side
    }
}

impl<'mesh> MonotoneDecomp<'mesh> {
    pub fn new(mesh: &'mesh EdgeMesh2) -> MonotoneDecomp<'mesh> {
        MonotoneDecomp { mesh }
    }
    pub fn build(self) -> Vec<Vec<MonoVertex>> {
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
            let mut polygon = vec![];
            for vert in monotone {
                if vert.ray() == Ray::None {
                    polygon.push(MonoVertex {
                        vertex: vert.vertex(),
                        side: MonoSide::Both,
                    });
                } else if vert.down() {
                    polygon.push(MonoVertex {
                        vertex: vert.vertex(),
                        side: MonoSide::Down,
                    });
                } else {
                    polygon.push(MonoVertex {
                        vertex: vert.vertex(),
                        side: MonoSide::Up,
                    });
                }
            }
            polygons.push(polygon);
        }
        polygons
    }
}

// #[test]
// fn test_monotone() {
//     for poly in Polygon2::test_cases() {
//         let mut mesh = EdgeMesh2::new(vec![], vec![]);
//         mesh.add_polygon(&poly);
//         let monotones = MonotoneDecomp::new(&mesh).build();
//         let mut total = 0.0;
//         for monotone in monotones {
//             let area = monotone.signed_area();
//             assert!(area >= 0.0);
//             total += area;
//         }
//         let expected = poly.signed_area();
//         assert!(
//             (total - expected).abs() < 10e-10,
//             "{:?} {:?}",
//             total,
//             expected
//         );
//     }
// }
