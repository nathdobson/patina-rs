use patina_bambu::BambuPart;
use patina_extrude::ExtrusionBuilder;
use patina_font::PolygonOutlineBuilder;
use patina_mesh::bimesh2::Bimesh2;
use patina_mesh::edge_mesh2::EdgeMesh2;
use patina_mesh::ser::{encode_file, encode_test_file};
use patina_vec::mat4::Mat4;
use patina_vec::vec2::Vec2;
use patina_vec::vec3::Vec3;
use rusttype::{Font, Point, Rect, Scale};
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;

pub struct StackBuilder {
    pub width: f64,
    pub length: f64,
    pub incut: f64,
    pub extension: f64,
    pub axle_diameter: f64,
    pub drum_diameter: f64,

    pub thickness: f64,
    pub support_thickness: f64,
    pub letter_thickness: f64,

    pub letters: Vec<char>,
    pub font: Font<'static>,

    pub flap_separation: f64,
    pub wall_separation: f64,
    pub letter_scale: f32,
    pub shift_letter: Vec2,
}

impl StackBuilder {
    fn blank_poly(&self) -> EdgeMesh2 {
        let profile = vec![
            Vec2::new(self.width / 2.0 - self.incut, 0.0),
            Vec2::new(self.width / 2.0 - self.incut, self.extension),
            Vec2::new(self.width / 2.0, self.extension),
            Vec2::new(self.width / 2.0, self.extension + self.axle_diameter),
            Vec2::new(
                self.width / 2.0 - self.incut,
                self.extension + self.axle_diameter,
            ),
            Vec2::new(self.width / 2.0 - self.incut, self.drum_diameter),
            Vec2::new(self.width / 2.0, self.drum_diameter),
            Vec2::new(self.width / 2.0, self.length),
        ];
        let mut poly = profile.clone();
        for v in profile.iter().rev() {
            poly.push(Vec2::new(-v.x(), v.y()));
        }
        let mut mesh = EdgeMesh2::new();
        mesh.add_polygon(poly.into_iter());
        mesh
    }
    fn letter_poly(&self, index: usize) -> Arc<EdgeMesh2> {
        let scale = Scale::uniform(self.letter_scale);
        let v_metrics = self.font.v_metrics(scale);
        let v_shift = (v_metrics.ascent / 2.0) as f64;
        let glyph = self.font.glyph(self.letters[index]).scaled(scale);
        let h_metrics = glyph.h_metrics();
        let h_shift = (-h_metrics.advance_width / 2.0) as f64;
        let shift = self.shift_letter + Vec2::new(h_shift, v_shift);
        let mut outline = PolygonOutlineBuilder::new(1.0);
        let bb = glyph.exact_bounding_box().unwrap_or(Rect::default());
        let minx = bb.min.x as f64;
        let maxx = bb.max.x as f64;
        glyph.build_outline(&mut outline);
        let outline = outline.build();
        let mut outline_mesh = EdgeMesh2::new();
        for outline in outline {
            outline_mesh.add_polygon(outline.points().iter().map(|p| *p + shift));
        }
        Arc::new(outline_mesh)
    }
    fn letter_split(&self, letter: Arc<EdgeMesh2>) -> [EdgeMesh2; 2] {
        [false, true].map(|side| {
            let mut sub = EdgeMesh2::new();
            let minx = -self.width / 2.0 + self.incut + self.wall_separation;
            let maxx = self.width / 2.0 - self.incut - self.wall_separation;
            let miny;
            let maxy;
            if side {
                miny = -self.length + self.wall_separation - self.flap_separation / 2.0;
                maxy = -self.wall_separation - self.flap_separation / 2.0;
            } else {
                miny = self.wall_separation + self.flap_separation / 2.0;
                maxy = self.length - self.wall_separation + self.flap_separation / 2.0;
            }
            sub.add_polygon(
                vec![
                    Vec2::new(minx, miny),
                    Vec2::new(maxx, miny),
                    Vec2::new(maxx, maxy),
                    Vec2::new(minx, maxy),
                ]
                .into_iter(),
            );
            let bimesh = Bimesh2::new(letter.clone(), Arc::new(sub));
            let result = bimesh.intersection();
            if side {
                result.map_vertices(|v| Vec2::new(-v.x(), -v.y() - self.flap_separation / 2.0))
            } else {
                result
                    .map_vertices(|v| Vec2::new(-v.x(), v.y() - self.flap_separation / 2.0))
                    .invert_edges()
            }
        })
    }
    async fn render_svg(&self, index: usize, blank: &EdgeMesh2, split: &[EdgeMesh2; 2]) {
        let mut mixed = EdgeMesh2::new();
        mixed.add_mesh(
            &blank.map_vertices(|v| Vec2::new(v.x(), v.y() + self.flap_separation / 2.0)),
            false,
        );
        mixed.add_mesh(
            &blank.map_vertices(|v| Vec2::new(v.x(), -v.y() - self.flap_separation / 2.0)),
            true,
        );
        mixed.add_mesh(
            &split[0].map_vertices(|v| Vec2::new(-v.x(), v.y() + self.flap_separation / 2.0)),
            false,
        );
        mixed.add_mesh(
            &split[1].map_vertices(|v| Vec2::new(-v.x(), -v.y() - self.flap_separation / 2.0)),
            true,
        );
        encode_file(
            &mixed,
            Path::new(&format!(
                "examples/flap/output/letters/letter_{}.svg",
                index
            )),
        )
        .await
        .unwrap();
    }
    async fn body_part(
        &self,
        index: usize,
        blank: &EdgeMesh2,
        letter1: &EdgeMesh2,
        letter2: &EdgeMesh2,
        transform: [f64; 12],
    ) -> BambuPart {
        let start = Instant::now();
        let mut ext = ExtrusionBuilder::new();
        let p1 = ext.add_plane(0.0, true);
        let p2 = ext.add_plane(self.letter_thickness, true);
        let p3 = ext.add_plane(self.thickness - self.letter_thickness, false);
        let p4 = ext.add_plane(self.thickness, false);
        ext.add_prism(&blank, (p1, false), (p4, false));
        ext.add_prism(&letter1, (p2, false), (p1, true));
        ext.add_prism(&letter2, (p4, true), (p3, false));
        let mesh = ext.build();
        if let Err(e) = mesh.check_manifold() {
            eprintln!("body_part {:?}", e);
        }
        println!("Built mesh in {}", start.elapsed().as_secs_f64());
        encode_file(
            &mesh,
            Path::new(&format!("examples/flap/output/flaps/flap_{}.stl", index)),
        )
        .await
        .unwrap();
        let mut body = BambuPart::new(mesh);
        body.material(Some(2));
        body.name(Some(format!("part({})", index)));
        body.transform(Some(transform));
        body
    }
    async fn letter_part(
        &self,
        index: usize,
        blank: &EdgeMesh2,
        letter1: &EdgeMesh2,
        letter2: &EdgeMesh2,
        transform: [f64; 12],
    ) -> BambuPart {
        let start = Instant::now();
        let mut ext = ExtrusionBuilder::new();
        let p1 = ext.add_plane(0.0, true);
        let p2 = ext.add_plane(self.letter_thickness, false);
        let p3 = ext.add_plane(self.thickness - self.letter_thickness, true);
        let p4 = ext.add_plane(self.thickness, false);
        ext.add_prism(&letter1, (p1, false), (p2, false));
        ext.add_prism(&letter2, (p3, false), (p4, false));
        let mesh = ext.build();
        if let Err(e) = mesh.check_manifold() {
            eprintln!("letter_part {:?}", e);
        }
        println!("Built mesh in {}", start.elapsed().as_secs_f64());
        encode_file(
            &mesh,
            Path::new(&format!(
                "examples/flap/output/inserts/letter_{}.stl",
                index
            )),
        )
        .await
        .unwrap();
        let mut body = BambuPart::new(mesh);
        body.material(Some(1));
        body.name(Some(format!("part({})", index)));
        body.transform(Some(transform));
        body
    }
    pub async fn flap_parts(
        &self,
        index: usize,
        blank: &EdgeMesh2,
        letter1: &EdgeMesh2,
        letter2: &EdgeMesh2,
    ) -> Vec<BambuPart> {
        let transform = Mat4::translate(Vec3::new(
            90.0,
            90.0 - self.length / 2.0,
            (index as f64) * (self.thickness + self.support_thickness) + self.support_thickness,
        ))
        .as_affine()
        .unwrap();
        vec![
            self.body_part(index, &blank, &letter1, &letter2, transform)
                .await,
            self.letter_part(index, &blank, &letter1, &letter2, transform)
                .await,
        ]
    }
    pub async fn build(&self) -> Vec<BambuPart> {
        let blank = self.blank_poly();
        let mut letters = vec![];
        for index in 0..self.letters.len() {
            println!("Building letter {}", index);
            let split = self.letter_split(self.letter_poly(index));
            self.render_svg(index, &blank, &split).await;
            letters.push(split);
        }
        let mut result = vec![];
        let max_part = self.letters.len();
        let max_part = 5;
        for index in 0..max_part {
            println!("Building part {}", index);
            result.extend(
                self.flap_parts(
                    index,
                    &blank,
                    &letters[index][1],
                    &letters[(index + 1) % letters.len()][0],
                )
                .await,
            );
        }
        result
    }
}
