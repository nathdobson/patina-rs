use crate::{BambuBuilder, BambuObject, BambuPart, BambuPlate};
use patina_geo::geo3::aabb3::Aabb3;
use patina_mesh::mesh::Mesh;
use patina_mesh::ser::create_test_path;
use patina_vec::vec3::Vec3;
use tokio::fs;

#[tokio::test]
async fn test_basic() -> anyhow::Result<()> {
    let mut bambu = BambuBuilder::new();
    bambu.add_plate({
        let mut plate = BambuPlate::new();
        plate.add_object({
            let mut obj = BambuObject::new();
            obj.add_part({
                let cube = Mesh::from_aabb(Aabb3::new(Vec3::splat(0.0), Vec3::splat(10.0)));
                let part = BambuPart::new(cube);
                part
            });
            obj
        });
        plate
    });
    fs::write(create_test_path("bambu.3mf").await?, bambu.build()?).await?;
    Ok(())
}
