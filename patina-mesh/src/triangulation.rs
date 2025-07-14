use crate::directed_mesh_edge::DirectedMeshEdge;
use crate::mesh_triangle::MeshTriangle;
use arrayvec::ArrayVec;
use itertools::Itertools;
use ordered_float::NotNan;
use patina_geo::geo2::polygon2::Polygon2;
use patina_geo::geo2::ray2::Ray2;
use patina_geo::geo2::segment2::Segment2;
use patina_geo::geo2::triangle2::Triangle2;
use patina_vec::vec2::Vec2;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::mem;

pub struct Triangulation {
    vertices: Vec<Vec2>,
    reverse: HashMap<usize, usize>,
    forward: HashMap<usize, usize>,
    triangles: Vec<MeshTriangle>,
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
struct Vertical {
    vertex: usize,
    up: Option<DirectedMeshEdge>,
    down: Option<DirectedMeshEdge>,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
enum MonoSide {
    Up,
    Down,
    Both,
}
#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
struct MonoVertex {
    vertex: usize,
    side: MonoSide,
}

#[derive(Clone, Debug)]
struct Trap {
    left: Vertical,
    right: Vertical,
}

impl Triangulation {
    pub fn new() -> Self {
        Triangulation {
            vertices: vec![],
            forward: HashMap::new(),
            reverse: HashMap::new(),
            triangles: vec![],
        }
    }
    pub fn add_polygon(&mut self, p: &Polygon2) {
        let mut vns = vec![];
        for v in p.points() {
            vns.push(self.vertices.len());
            self.vertices.push(*v);
        }
        for (v1, v2) in vns.into_iter().circular_tuple_windows() {
            self.forward.insert(v1, v2);
            self.reverse.insert(v2, v1);
        }
    }
    fn traps(&self) -> Vec<Trap> {
        let mut order = (0..self.vertices.len()).collect::<Vec<_>>();
        order.sort_by_key(|i| NotNan::new(self.vertices[*i].x()).unwrap());
        let mut traps: Vec<Trap> = vec![];
        let mut parts: Vec<Vertical> = vec![];
        for &vertex in &order {
            let pred_reverse = parts.iter().position(|part| {
                part.down
                    .map_or(part.vertex == self.reverse[&vertex], |e| e.v2() == vertex)
            });
            let pred_forward = parts.iter().position(|part| {
                part.up
                    .map_or(part.vertex == self.forward[&vertex], |e| e.v1() == vertex)
            });
            match (pred_reverse, pred_forward) {
                (None, None) => {
                    let d1 = (self.vertices[self.reverse[&vertex]] - self.vertices[vertex]).slope();
                    let d2 = (self.vertices[self.forward[&vertex]] - self.vertices[vertex]).slope();
                    if d1 < d2 {
                        let pred = parts
                            .iter()
                            .enumerate()
                            .filter(|(index, part)| {
                                let ray1 = match &part.up {
                                    None => Segment2::new(
                                        self.vertices[part.vertex],
                                        self.vertices[self.reverse[&part.vertex]],
                                    )
                                    .as_ray(),
                                    Some(up) => Segment2::new(
                                        self.vertices[up.v2()],
                                        self.vertices[up.v1()],
                                    )
                                    .as_ray(),
                                };
                                let ray2 = match &part.down {
                                    None => Segment2::new(
                                        self.vertices[part.vertex],
                                        self.vertices[self.forward[&part.vertex]],
                                    )
                                    .as_ray(),
                                    Some(down) => Segment2::new(
                                        self.vertices[down.v1()],
                                        self.vertices[down.v2()],
                                    )
                                    .as_ray(),
                                };
                                assert!(ray1.dir().x() >= 0.0);
                                assert!(ray2.dir().x() >= 0.0);
                                ray1.above(self.vertices[vertex]) != Ordering::Less
                                    && ray2.above(self.vertices[vertex]) != Ordering::Greater
                            })
                            .exactly_one()
                            .unwrap()
                            .0;
                        let pred = parts.remove(pred);
                        let up = pred.up.unwrap_or_else(|| {
                            let v = pred.vertex;
                            DirectedMeshEdge::new(self.reverse[&v], v)
                        });
                        let down = pred.down.unwrap_or_else(|| {
                            let v = pred.vertex;
                            DirectedMeshEdge::new(v, self.forward[&v])
                        });
                        traps.push(Trap {
                            left: pred,
                            right: Vertical {
                                vertex,
                                up: Some(up),
                                down: Some(down),
                            },
                        });
                        parts.push(Vertical {
                            vertex,
                            up: Some(up),
                            down: None,
                        });
                        parts.push(Vertical {
                            vertex,
                            up: None,
                            down: Some(down),
                        });
                    } else if d2 < d1 {
                        parts.push(Vertical {
                            vertex,
                            up: None,
                            down: None,
                        })
                    } else if d1.is_nan() || d2.is_nan() {
                        panic!("Bad vertices");
                    } else {
                        panic!("Equal vertices");
                    }
                }
                (Some(pred), None) => {
                    let pred = parts.remove(pred);
                    let up = pred.up.unwrap_or_else(|| {
                        let v = pred.vertex;
                        DirectedMeshEdge::new(self.reverse[&v], v)
                    });
                    let next = Vertical {
                        vertex,
                        up: Some(up),
                        down: None,
                    };
                    traps.push(Trap {
                        left: pred,
                        right: next.clone(),
                    });
                    parts.push(next);
                }
                (None, Some(pred)) => {
                    let pred = parts.remove(pred);
                    let down = pred.down.unwrap_or_else(|| {
                        let v = pred.vertex;
                        DirectedMeshEdge::new(v, self.forward[&v])
                    });
                    let next = Vertical {
                        vertex,
                        up: None,
                        down: Some(down),
                    };
                    traps.push(Trap {
                        left: pred,
                        right: next.clone(),
                    });
                    parts.push(next);
                }
                (Some(pred_reverse), Some(pred_forward)) if pred_reverse == pred_forward => {
                    let pred = parts.remove(pred_reverse);
                    traps.push(Trap {
                        left: pred,
                        right: Vertical {
                            vertex,
                            up: None,
                            down: None,
                        },
                    })
                }
                (Some(pred_reverse), Some(pred_forward)) => {
                    let (pred_reverse, pred_forward) = if pred_reverse < pred_forward {
                        let pred_forward = parts.remove(pred_forward);
                        let pred_reverse = parts.remove(pred_reverse);
                        (pred_reverse, pred_forward)
                    } else {
                        let pred_reverse = parts.remove(pred_reverse);
                        let pred_forward = parts.remove(pred_forward);
                        (pred_reverse, pred_forward)
                    };
                    let up = pred_reverse.up.unwrap_or_else(|| {
                        let v = pred_reverse.vertex;
                        DirectedMeshEdge::new(self.reverse[&v], v)
                    });
                    let down = pred_forward.down.unwrap_or_else(|| {
                        let v = pred_forward.vertex;
                        DirectedMeshEdge::new(v, self.forward[&v])
                    });
                    let next = Vertical {
                        vertex,
                        up: Some(up),
                        down: Some(down),
                    };
                    traps.push(Trap {
                        left: pred_reverse,
                        right: Vertical {
                            vertex,
                            up: Some(up),
                            down: None,
                        },
                    });
                    traps.push(Trap {
                        left: pred_forward,
                        right: Vertical {
                            vertex,
                            up: None,
                            down: Some(down),
                        },
                    });
                    parts.push(next);
                }
            }
        }
        assert!(parts.is_empty(), "not empty at end: {:?}", parts);
        traps
    }
    fn diagonalize_monotonic(&self, mono: &[MonoVertex], diags: &mut Vec<DirectedMeshEdge>) {
        let mut stack = vec![];
        stack.push(mono[0].clone());
        stack.push(mono[1].clone());
        for i in 2..mono.len() - 1 {
            if mono[i].side != stack.last().unwrap().side {
                while stack.len() >= 2 {
                    let next = stack.pop().unwrap();
                    if next.side == MonoSide::Down {
                        diags.push(DirectedMeshEdge::new(next.vertex, mono[i].vertex));
                    } else {
                        diags.push(DirectedMeshEdge::new(mono[i].vertex, next.vertex));
                    }
                }
                stack.pop().unwrap();
                assert!(stack.is_empty());
                stack.push(mono[i - 1]);
                stack.push(mono[i]);
            } else {
                let ul = stack.pop().unwrap();
                let mut u = ul;
                while stack.len() >= 1 {
                    if u.side == mono[i].side {
                        break;
                    }
                    if u.side == MonoSide::Down {
                        diags.push(DirectedMeshEdge::new(u.vertex, mono[i].vertex));
                    } else {
                        diags.push(DirectedMeshEdge::new(mono[i].vertex, u.vertex));
                    }
                    u = stack.pop().unwrap();
                }
                stack.push(ul);
                stack.push(mono[i]);
            }
        }
        for v in &stack[1..stack.len() - 1] {
            match v.side {
                MonoSide::Up => {
                    diags.push(DirectedMeshEdge::new(mono.last().unwrap().vertex, v.vertex));
                }
                MonoSide::Down => {
                    diags.push(DirectedMeshEdge::new(v.vertex, mono.last().unwrap().vertex));
                }
                MonoSide::Both => unreachable!(),
            }
        }
    }
    pub fn build(self) -> Vec<Triangle2> {
        let mut traps = self.traps();
        let mut traps2 = vec![];
        for trap in traps {
            let up_half = Trap {
                left: Vertical {
                    down: None,
                    ..trap.left
                },
                right: Vertical {
                    down: None,
                    ..trap.right
                },
            };
            let down_half = Trap {
                left: Vertical {
                    up: None,
                    ..trap.left
                },
                right: Vertical {
                    up: None,
                    ..trap.right
                },
            };
            for half in [up_half, down_half] {
                if half.left.up.is_some()
                    || half.left.down.is_some()
                    || half.right.up.is_some()
                    || half.right.down.is_some()
                {
                    traps2.push(half);
                }
            }
        }
        traps = traps2;
        let mut monos: Vec<Vec<Trap>> = vec![];
        'mono_builder: for trap in traps {
            if trap.left.up.is_none() && trap.left.down.is_none() {
                monos.push(vec![trap]);
                continue 'mono_builder;
            }
            for mono in &mut monos {
                if mono.last().unwrap().right == trap.left {
                    mono.push(trap);
                    continue 'mono_builder;
                }
            }
            unreachable!();
        }
        let mut tris = vec![];
        for mono in &monos {
            for (t1, t2) in mono.iter().tuple_windows() {
                assert_eq!(t1.right, t2.left);
            }
            let mut verticals = vec![];
            for t1 in mono {
                verticals.push(t1.left.clone());
            }
            verticals.push(mono.last().unwrap().right.clone());
            let verticals: Vec<_> = verticals
                .into_iter()
                .map(|x| {
                    //
                    let side = match (x.up.is_some(), x.down.is_some()) {
                        (false, false) => MonoSide::Both,
                        (false, true) => MonoSide::Down,
                        (true, false) => MonoSide::Up,
                        (true, true) => panic!(),
                    };
                    MonoVertex {
                        vertex: x.vertex,
                        side,
                    }
                })
                .collect();
            let mut diags = vec![];
            match verticals[1].side {
                MonoSide::Up => {
                    diags.push(DirectedMeshEdge::new(
                        verticals[0].vertex,
                        verticals[1].vertex,
                    ));
                }
                MonoSide::Down => {
                    diags.push(DirectedMeshEdge::new(
                        verticals[1].vertex,
                        verticals[0].vertex,
                    ));
                }
                MonoSide::Both => unreachable!(),
            }

            self.diagonalize_monotonic(&verticals, &mut diags);
            match verticals[verticals.len() - 2].side {
                MonoSide::Up => {
                    diags.push(DirectedMeshEdge::new(
                        verticals[verticals.len() - 1].vertex,
                        verticals[verticals.len() - 2].vertex,
                    ));
                }
                MonoSide::Down => {
                    diags.push(DirectedMeshEdge::new(
                        verticals[verticals.len() - 2].vertex,
                        verticals[verticals.len() - 1].vertex,
                    ));
                }
                MonoSide::Both => unreachable!(),
            }

            for (d1, d2) in diags.iter().tuple_windows() {
                if d1.v1() == d2.v1() {
                    tris.push(MeshTriangle::new(d2.v2(), d1.v1(), d1.v2()));
                } else if d1.v2() == d2.v2() {
                    tris.push(MeshTriangle::new(d1.v1(), d1.v2(), d2.v1()));
                } else {
                    unreachable!();
                }
            }
        }
        tris.into_iter()
            .map(|x| x.for_vertices2(&self.vertices))
            .collect()
    }
}

#[test]
fn test_traps() {
    let mut tris = Triangulation::new();
    tris.add_polygon(&Polygon2::new(vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(2.0, 1.0),
        Vec2::new(1.0, 2.0),
    ]));
    println!("{:#?}", tris.build());
}

#[test]
fn test_triangulation() {
    for size in 3..=8 {
        println!("size = {:?}", size);
        for seed in 4..1000 {
            println!("seed = {:?}", seed);
            let mut rng = XorShiftRng::seed_from_u64(seed);
            let poly = Polygon2::random(&mut rng, size);
            println!("{}", poly);
            let mut tri = Triangulation::new();
            tri.add_polygon(&poly);
            let result = tri.build();
            for t in &result {
                println!("area 1: {:?}", t.signed_area());
            }
            let a1 = poly.signed_area();
            let a2 = result.iter().map(|x| x.signed_area()).sum::<f64>();
            assert!((a1 - a2).abs() < 10e-10, "{:?} ~= {:?}", a1, a2);
        }
    }
}
