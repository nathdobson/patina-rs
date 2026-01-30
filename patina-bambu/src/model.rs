use crate::{BambuObject, BambuPart, BambuPartType};
use patina_geo::geo3::cylinder::Cylinder;
use patina_mesh::mesh::Mesh;
use patina_sdf::marching_mesh::MarchingMesh;
use patina_sdf::sdf::{AsSdf, Sdf3};
use patina_threads::ThreadMetrics;
use patina_vec::vec3::Vec3;
use std::rc::Rc;
use patina_3mf::brim_points::BrimPoint;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ModelModifier {
    ModifierPart(BambuPart),
}

#[derive(Clone)]
pub struct SdfModel {
    pub sdf: Sdf3,
    pub metadata: Vec<ModelModifier>,
}

#[derive(Clone, Debug)]
pub struct MeshModel {
    pub mesh: Mesh,
    pub metadata: Vec<ModelModifier>,
}

impl SdfModel {
    pub fn new() -> Self {
        SdfModel {
            sdf: Sdf3::empty(),
            metadata: vec![],
        }
    }
    pub fn build(self, marching: MarchingMesh) -> MeshModel {
        MeshModel {
            mesh: marching.build(&self.sdf),
            metadata: self.metadata,
        }
    }
    pub fn add_metadata(&mut self, metadata: ModelModifier) {
        self.metadata.push(metadata);
    }
    pub fn add_sdf(&mut self, sdf: &Sdf3) {
        self.sdf = self.sdf.union(sdf);
    }
    pub fn subtract_sdf(&mut self, sdf: &Sdf3) {
        self.sdf = self.sdf.difference(sdf);
    }
    pub fn drill_ruthex(&mut self, position: Vec3, axis: Vec3, threads: &ThreadMetrics) {
        self.subtract_sdf(
            &Cylinder::new(position, axis * threads.ruthex_depth, threads.ruthex_radius).as_sdf(),
        );
        let mut part = BambuPart::new(Mesh::from_cylinder(
            &Cylinder::new(
                position,
                axis * threads.ruthex_depth,
                threads.ruthex_radius + threads.ruthex_width,
            ),
            100,
        ));
        part.typ(BambuPartType::Modifier);
        part.wall_loops(Some(3));
        part.name(Some(format!("ruthex_{}", threads.name)));
        self.add_metadata(ModelModifier::ModifierPart(part))
    }
}

impl MeshModel {
    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }
}

impl BambuObject {
    pub fn from_model(model: MeshModel, brim_points: &[BrimPoint]) -> Self {
        let mut object = BambuObject::new();
        object.name(Some("main_object".to_string()));
        let mut main = BambuPart::new(model.mesh().clone());
        main.name(Some("main_part".to_string()));
        for brim_point in brim_points {
            main.add_brim_point(brim_point.clone());
        }
        object.add_part(main);
        for meta in model.metadata {
            match meta {
                ModelModifier::ModifierPart(part) => object.add_part(part),
            }
        }
        object
    }
}

#[test]
fn test() {}
