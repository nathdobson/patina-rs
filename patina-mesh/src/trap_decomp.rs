use crate::directed_mesh_edge::DirectedMeshEdge;
use crate::mesh_edge::MeshEdge;
use crate::mesh2::Mesh2;
use arrayvec::ArrayVec;
use itertools::Itertools;
use ordered_float::NotNan;
use patina_geo::geo2::polygon2::Polygon2;
use patina_geo::geo2::triangle2::Triangle2;
use patina_geo::geo3::triangle3::Triangle3;
use patina_vec::vec2::Vec2;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::{Bound, Index};

#[derive(Copy, Clone, Debug)]
pub struct EdgeKey {
    left: Vec2,
    right: Vec2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Ray {
    None,
    Edge(MeshEdge),
    All,
}

#[derive(Debug)]
struct EdgeValue {
    edge: DirectedMeshEdge,
    left_up: Vertical,
    left_down: Vertical,
}

#[derive(Debug, Clone)]
pub struct Vertical {
    vertex: usize,
    ray: Ray,
    down: bool,
}

#[derive(Debug)]
pub struct Trap {
    left: Vertical,
    right: Vertical,
    top_direction: bool,
    bottom_direction: bool,
}

// enum TrapVertex {
//     Original { pos: Vec2, vertex: usize },
//     Virtual { pos: Vec2 },
// }

pub struct TrapDecomp<'mesh> {
    mesh: &'mesh Mesh2,
    edge_map: BTreeMap<EdgeKey, EdgeValue>,
    traps: Vec<Trap>,
}

impl Eq for EdgeKey {}

impl PartialEq<Self> for EdgeKey {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

impl PartialOrd<Self> for EdgeKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Vertical {
    fn eq(&self, other: &Self) -> bool {
        self.vertex == other.vertex
            && ((self.ray == other.ray && self.down == other.down)
                || (self.ray == Ray::None && other.ray == Ray::None))
    }
}

impl Eq for Vertical {}

impl Hash for Vertical {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.vertex.hash(state);
        self.ray.hash(state);
        match self.ray {
            Ray::None => {}
            _ => self.down.hash(state),
        }
    }
}

impl EdgeKey {
    pub fn eval_y(&self, x: f64) -> f64 {
        if self.left.x() == self.right.x() {
            return (self.left.y() + self.right.y()) / 2.0;
        } else if x == self.left.x() {
            return self.left.y();
        } else if x == self.right.x() {
            return self.right.y();
        }
        let dx = self.right.x() - self.left.x();
        if dx == 0.0 {
            (self.left.y() + self.right.y()) / 2.0
        } else {
            let t = (x - self.left.x()) / dx;
            assert!(0.0 <= t && t <= 1.0);
            self.left.y() * (1.0 - t) + self.right.y() * t
        }
    }
}

impl Ord for EdgeKey {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        let minx = self.left.x().max(other.left.x());
        let maxx = self.right.x().min(other.right.x());
        let result = self
            .eval_y(minx)
            .total_cmp(&other.eval_y(minx))
            .then_with(|| self.eval_y(maxx).total_cmp(&other.eval_y(maxx)))
            .then_with(|| panic!("Cannot compare {:?} and {:?}", self, other));
        result
    }
}

impl EdgeKey {
    pub fn new(v1: Vec2, v2: Vec2) -> EdgeKey {
        if v1 < v2 {
            EdgeKey {
                left: v1,
                right: v2,
            }
        } else if v1 > v2 {
            EdgeKey {
                left: v2,
                right: v1,
            }
        } else if v1 == v2 {
            EdgeKey {
                left: v1,
                right: v2,
            }
        } else {
            panic!("nan vertex");
        }
    }
}

impl Vertical {
    pub fn for_vertices(&self, vs: &[Vec2]) -> ArrayVec<Vec2, 2> {
        let mut result = ArrayVec::<Vec2, 2>::new();
        match self.ray {
            Ray::None => result.push(vs[self.vertex]),
            Ray::Edge(edge) => {
                result.push(vs[self.vertex]);
                let y = EdgeKey::new(vs[edge.v1()], vs[edge.v2()]).eval_y(vs[self.vertex].x());
                result.push(Vec2::new(vs[self.vertex].x(), y));
                if !self.down {
                    result.reverse();
                }
            }
            Ray::All => todo!(),
        }
        result
    }
    pub fn vertex(&self) -> usize {
        self.vertex
    }
    pub fn ray(&self) -> Ray {
        self.ray
    }
    pub fn down(&self) -> bool {
        self.down
    }
}

impl Trap {
    pub fn for_vertices(&self, vs: &[Vec2]) -> ArrayVec<Vec2, 4> {
        let mut result = ArrayVec::new();
        for x in self.left.for_vertices(vs) {
            result.push(x);
        }
        for x in self.right.for_vertices(vs).into_iter().rev() {
            result.push(x);
        }
        result
    }
    pub fn area(&self, vs: &[Vec2]) -> f64 {
        let vs = self.for_vertices(vs);
        let mut total = 0.0;
        let v1 = vs[0];
        for (v2, v3) in vs[1..].iter().cloned().tuple_windows() {
            let area = Triangle2::new([v1, v2, v3]).signed_area();
            assert!(area >= 0.0);
            total += area
        }
        total
    }
    pub fn left(&self) -> &Vertical {
        &self.left
    }
    pub fn right(&self) -> &Vertical {
        &self.right
    }
    pub fn top_direction(&self) -> bool {
        self.top_direction
    }
    pub fn bottom_direction(&self) -> bool {
        self.bottom_direction
    }
}

impl<'mesh> TrapDecomp<'mesh> {
    pub fn new(mesh: &'mesh Mesh2) -> TrapDecomp<'mesh> {
        TrapDecomp {
            mesh,
            edge_map: BTreeMap::new(),
            traps: vec![],
        }
    }
    pub fn edge_key(&self, edge: MeshEdge) -> EdgeKey {
        EdgeKey::new(
            self.mesh.vertices()[edge.v1()],
            self.mesh.vertices()[edge.v2()],
        )
    }
    pub fn build(mut self) -> Vec<Trap> {
        let mut order = (0..self.mesh.vertices().len()).collect::<Vec<usize>>();
        order.sort_by_key(|i| NotNan::new(self.mesh.vertices()[*i].x()).unwrap());
        let mut adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut forward: HashSet<DirectedMeshEdge> = HashSet::new();
        for vertex in 0..self.mesh.vertices().len() {
            adjacency.insert(vertex, vec![]);
        }
        for edge in self.mesh.edges() {
            forward.insert(edge.clone());
            adjacency.get_mut(&edge.v1()).unwrap().push(edge.v2());
            adjacency.get_mut(&edge.v2()).unwrap().push(edge.v1());
        }
        let mut visited = HashSet::new();
        for v1 in order {
            visited.insert(v1);
            let mut incoming = vec![];
            let mut outgoing = vec![];
            for &v2 in &adjacency[&v1] {
                if visited.contains(&v2) {
                    incoming.push((self.edge_key(MeshEdge::new(v1, v2)), v2));
                } else {
                    outgoing.push((self.edge_key(MeshEdge::new(v1, v2)), v2));
                }
            }
            incoming.sort();
            outgoing.sort();
            let mut incoming_seq: Vec<Option<_>> = vec![];
            incoming_seq.push(
                self.edge_map
                    .range(
                        ..incoming.first().map_or_else(
                            || EdgeKey::new(self.mesh.vertices()[v1], self.mesh.vertices()[v1]),
                            |first| first.0,
                        ),
                    )
                    .next_back(),
            );
            for (key, v2) in incoming.iter() {
                incoming_seq.push(Some(self.edge_map.get_key_value(key).unwrap()));
            }
            incoming_seq.push(
                self.edge_map
                    .range((
                        Bound::Excluded(incoming.last().map_or_else(
                            || EdgeKey::new(self.mesh.vertices()[v1], self.mesh.vertices()[v1]),
                            |first| first.0,
                        )),
                        Bound::Unbounded,
                    ))
                    .next(),
            );
            for (index, (e1, e2)) in incoming_seq.iter().tuple_windows().enumerate() {
                if let (Some((ek1, ev1)), Some((ek2, ev2))) = (e1, e2) {
                    let top = ev2.edge.inverted();
                    let bottom = ev1.edge;
                    let top_direction = if forward.contains(&top) {
                        true
                    } else if forward.contains(&top.inverted()) {
                        false
                    } else {
                        unreachable!()
                    };
                    let bottom_direction = if forward.contains(&bottom) {
                        true
                    } else if forward.contains(&bottom.inverted()) {
                        false
                    } else {
                        unreachable!()
                    };
                    let mut lefts = ArrayVec::<_, 2>::new();
                    if ev2.left_down == ev1.left_up {
                        lefts.push(ev2.left_down.clone());
                    } else {
                        lefts.push(ev2.left_down.clone());
                        lefts.push(ev1.left_up.clone());
                    }
                    for left in &lefts {
                        if incoming_seq.len() == 2 {
                            self.traps.push(Trap {
                                left: Vertical {
                                    vertex: left.vertex,
                                    ray: if left.down { left.ray } else { Ray::None },
                                    down: left.down,
                                },
                                right: Vertical {
                                    vertex: v1,
                                    ray: if lefts.len() == 2 && !left.down {
                                        Ray::None
                                    } else {
                                        Ray::Edge(ev1.edge.edge())
                                    },
                                    down: true,
                                },
                                top_direction,
                                bottom_direction,
                            });
                            self.traps.push(Trap {
                                left: Vertical {
                                    vertex: left.vertex,
                                    ray: if !left.down { left.ray } else { Ray::None },
                                    down: left.down,
                                },
                                right: Vertical {
                                    vertex: v1,
                                    ray: if lefts.len() == 2 && left.down {
                                        Ray::None
                                    } else {
                                        Ray::Edge(ev2.edge.edge())
                                    },
                                    down: false,
                                },
                                top_direction,
                                bottom_direction,
                            });
                        } else if index == 0 {
                            self.traps.push(Trap {
                                left: left.clone(),
                                right: Vertical {
                                    vertex: v1,
                                    ray: if lefts.len() == 2 && !left.down {
                                        Ray::None
                                    } else {
                                        Ray::Edge(ev1.edge.edge())
                                    },
                                    down: true,
                                },
                                top_direction,
                                bottom_direction,
                            });
                        } else if index == incoming_seq.len() - 2 {
                            self.traps.push(Trap {
                                left: left.clone(),
                                right: Vertical {
                                    vertex: v1,
                                    ray: if lefts.len() == 2 && left.down {
                                        Ray::None
                                    } else {
                                        Ray::Edge(ev2.edge.edge())
                                    },
                                    down: false,
                                },
                                top_direction,
                                bottom_direction,
                            });
                        }
                        if 0 < index && index < incoming_seq.len() - 2 {
                            self.traps.push(Trap {
                                left: left.clone(),
                                right: Vertical {
                                    vertex: v1,
                                    ray: Ray::None,
                                    down: false,
                                },
                                top_direction,
                                bottom_direction,
                            });
                        };
                    }
                }
            }
            for (key, v2) in incoming.iter() {
                self.edge_map.remove(&key);
            }
            let center = EdgeKey::new(self.mesh.vertices()[v1], self.mesh.vertices()[v1]);
            if let Some((dk, dv)) = self.edge_map.range_mut(..center).next_back() {
                dv.left_up = Vertical {
                    vertex: v1,
                    ray: Ray::Edge(dv.edge.edge()),
                    down: true,
                };
            }
            if let Some((uk, uv)) = self
                .edge_map
                .range_mut((Bound::Excluded(center), Bound::Unbounded))
                .next()
            {
                uv.left_down = Vertical {
                    vertex: v1,
                    ray: Ray::Edge(uv.edge.edge()),
                    down: false,
                };
            }
            if !outgoing.is_empty() {
                let left_down = if let Some((downk, downv)) = self
                    .edge_map
                    .range(..outgoing.first().unwrap().0)
                    .next_back()
                {
                    Ray::Edge(downv.edge.edge())
                } else {
                    Ray::All
                };
                let left_up = if let Some((upk, upv)) = self
                    .edge_map
                    .range((
                        Bound::Excluded(outgoing.last().unwrap().0),
                        Bound::Unbounded,
                    ))
                    .next()
                {
                    Ray::Edge(upv.edge.edge())
                } else {
                    Ray::All
                };
                for (index, (key, v2)) in outgoing.iter().enumerate() {
                    let left_down = if index == 0 { left_down } else { Ray::None };
                    let left_up = if index == outgoing.len() - 1 {
                        left_up
                    } else {
                        Ray::None
                    };
                    self.edge_map.insert(
                        *key,
                        EdgeValue {
                            edge: DirectedMeshEdge::new(v1, *v2),
                            left_up: Vertical {
                                vertex: v1,
                                ray: left_up,
                                down: false,
                            },
                            left_down: Vertical {
                                vertex: v1,
                                ray: left_down,
                                down: true,
                            },
                        },
                    );
                }
            }
        }
        self.traps
            .into_iter()
            .filter(|trap| trap.left.ray != Ray::None || trap.right.ray != Ray::None)
            .collect()
    }
}

#[test]
fn test_trap_decomp() {
    for size in 3..=8 {
        println!("size = {:?}", size);
        for seed in 272..1000 {
            println!("seed = {:?}", seed);
            let mut rng = XorShiftRng::seed_from_u64(seed);
            let xs = rng.random_range(4..10);
            let poly = Polygon2::random_discrete(&mut rng, xs, 10, size);
            println!("{}", poly);
            let mut mesh = Mesh2::new(vec![], vec![]);
            mesh.add_polygon(&poly);
            let mut td = TrapDecomp::new(&mesh).build();
            for td in &td {
                assert_eq!(td.bottom_direction, td.top_direction);
            }
            let area: f64 = td
                .iter()
                .filter(|td| td.bottom_direction)
                .map(|td| td.area(&mesh.vertices()))
                .sum();
            let expected = poly.signed_area();
            assert!(
                (area - expected).abs() < 10e-10,
                "area: {:?} expected: {:?} diff {:?}",
                area,
                expected,
                area - expected
            );
        }
    }
    //
    // let mut td = TrapDecomp::new(vs.clone(), es.clone());
    // let td = td.build();
    // println!("{:#?}", td);
    // let mut area = 0.0;
    // for trap in td {
    //     for (v1, v2, v3) in trap.for_vertices(&vs).into_iter().tuple_windows() {
    //         let tri = Triangle2::new([v1, v2, v3]);
    //         let tri_area = tri.signed_area();
    //         assert!(tri_area > 0.0);
    //         area += tri_area;
    //     }
    // }
    // println!("{:?}", area);
}
