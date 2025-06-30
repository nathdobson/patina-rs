#![feature(exit_status_error)]
#![deny(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use patina_3mf::ModelContainer;
use patina_3mf::content_types::{ContentTypeDefault, ContentTypes};
use patina_3mf::model::build::{ModelBuild, ModelItem};
use patina_3mf::model::mesh::{
    ModelMesh, ModelTriangle, ModelTriangles, ModelVertex, ModelVertices,
};
use patina_3mf::model::resources::{
    ModelComponent, ModelComponents, ModelObject, ModelObjectType, ModelResources,
};
use patina_3mf::model::{Model, ModelMetadata, ModelUnit};
use patina_3mf::model_settings::{
    Assemble, AssembleItem, ModelInstance, ModelSettings, ObjectSettings, Part, Plate,
    SettingsMetadata,
};
use patina_3mf::project_settings::ProjectSettings;
use patina_3mf::relationships::{Relationship, Relationships};
use patina_bambu::{BambuBuilder, BambuObject, BambuPart, BambuPlate};
use patina_cad::math::vec3::Vec3;
use patina_cad::meshes::mesh::Mesh;
use patina_cad::meshes::mesh_triangle::MeshTriangle;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut bambu = BambuBuilder::new();

    bambu.add_plate({
        let mut plate = BambuPlate::new();
        plate.add_object({
            let mut object = BambuObject::new();
            object.add_part(BambuPart::new(
                1,
                Mesh::new(
                    vec![
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(10.0, 0.0, 0.0),
                        Vec3::new(0.0, 10.0, 0.0),
                        Vec3::new(0.0, 0.0, 10.0),
                    ],
                    vec![
                        MeshTriangle::new(0, 1, 2),
                        MeshTriangle::new(0, 3, 1),
                        MeshTriangle::new(2, 3, 0),
                        MeshTriangle::new(1, 3, 2),
                    ],
                ),
            ));
            object.add_part(BambuPart::new(
                2,
                Mesh::new(
                    vec![
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(-10.0, 0.0, 0.0),
                        Vec3::new(0.0, -10.0, 0.0),
                        Vec3::new(0.0, 0.0, 10.0),
                    ],
                    vec![
                        MeshTriangle::new(0, 1, 2),
                        MeshTriangle::new(0, 3, 1),
                        MeshTriangle::new(2, 3, 0),
                        MeshTriangle::new(1, 3, 2),
                    ],
                ),
            ));
            object
        });
        plate
    });
    bambu.build().await?;
    Ok(())
}
