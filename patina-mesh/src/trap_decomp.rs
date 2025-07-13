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
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::{Bound, Index};

#[derive(Copy, Clone, Debug)]
pub struct EdgeKey {
    left: Vec2,
    right: Vec2,
}

#[derive(Debug, Copy, Clone)]
enum Ray {
    None,
    Edge(MeshEdge),
    All,
}

#[derive(Debug)]
struct EdgeValue {
    edge: DirectedMeshEdge,
    left_up: Ray,
    left_down: Ray,
}

#[derive(Debug)]
struct Vertical {
    vertex: usize,
    ray: Ray,
    down: bool,
}

#[derive(Debug)]
pub struct Trap {
    left: Vertical,
    right: Vertical,
}

pub struct TrapDecomp {
    mesh: Mesh2,
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

impl EdgeKey {
    pub fn eval_y(&self, x: f64) -> f64 {
        if x == self.left.x() {
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
            .then_with(|| panic!());
        println!("{:?} cmp {:?} = {:?}", self, other, result);
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
        println!("{:?}", self);
        let vs = self.for_vertices(vs);
        println!("{:?}", vs);
        let mut total = 0.0;
        let v1 = vs[0];
        for (v2, v3) in vs[1..].iter().cloned().tuple_windows() {
            let area = Triangle2::new([v1, v2, v3]).signed_area();
            assert!(area > 0.0);
            total += area
        }
        println!("{:?}", total);
        total
    }
}

impl TrapDecomp {
    pub fn new(mesh: Mesh2) -> TrapDecomp {
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
        println!("{:?}", order);
        let mut adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
        for vertex in 0..self.mesh.vertices().len() {
            adjacency.insert(vertex, vec![]);
        }
        for edge in self.mesh.edges() {
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
            println!("incoming_seq for {} = {:#?}", v1, incoming_seq);
            for (index, (e1, e2)) in incoming_seq.iter().tuple_windows().enumerate() {
                if let (Some((ek1, ev1)), Some((ek2, ev2))) = (e1, e2) {
                    let left = if ek1.left.x() < ek2.left.x() {
                        Vertical {
                            vertex: ev2.edge.v1(),
                            ray: ev2.left_down.clone(),
                            down: true,
                        }
                    } else {
                        Vertical {
                            vertex: ev1.edge.v1(),
                            ray: ev1.left_up.clone(),
                            down: false,
                        }
                    };
                    let right = if index == 0 {
                        Vertical {
                            vertex: v1,
                            ray: Ray::Edge(ev1.edge.edge()),
                            down: true,
                        }
                    } else if index == incoming_seq.len() - 2 {
                        Vertical {
                            vertex: v1,
                            ray: Ray::Edge(ev2.edge.edge()),
                            down: false,
                        }
                    } else {
                        Vertical {
                            vertex: v1,
                            ray: Ray::None,
                            down: false,
                        }
                    };
                    self.traps.push(Trap { left, right });
                } else {
                    println!("Extra trap {:?} {:?}", e1, e2);
                }
            }
            for (key, v2) in incoming.iter() {
                self.edge_map.remove(&key);
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
                    println!("key = {:?}", key);
                    println!("table = {:?}", self.edge_map);
                    println!(
                        "count below {:?}",
                        self.edge_map.keys().filter(|k2| *k2 < key).count()
                    );
                    println!(
                        "count above {:?}",
                        self.edge_map.keys().filter(|k2| *k2 > key).count()
                    );
                    println!("left_up for {} {} = {:#?}", v1, v2, left_up);
                    println!("left_down for {} {} = {:#?}", v1, v2, left_down);
                    self.edge_map.insert(
                        *key,
                        EdgeValue {
                            edge: DirectedMeshEdge::new(v1, *v2),
                            left_up,
                            left_down,
                        },
                    );
                }
            }
        }
        self.traps
    }
}

#[test]
fn test_trap_decomp() {
    for size in 4..=8 {
        println!("size = {:?}", size);
        for seed in 14..1000 {
            println!("seed = {:?}", seed);
            let mut rng = XorShiftRng::seed_from_u64(seed);
            let poly = Polygon2::random(&mut rng, size);
            println!("{}", poly);
            let mut mesh = Mesh2::new(vec![], vec![]);
            mesh.add_polygon(&poly);
            let mut td = TrapDecomp::new(mesh.clone()).build();
            println!("{:#?}", td);
            let area: f64 = td.iter().map(|td| td.area(&mesh.vertices())).sum();
            let expected = poly.signed_area();
            assert!(
                (area - expected).abs() < 10e-10,
                "area: {:?} expected: {:?}",
                area,
                expected
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
