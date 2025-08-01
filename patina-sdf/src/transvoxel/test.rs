use crate::transvoxel::cube_edge::{CubeEdge, CubeEdgeSet};
use crate::transvoxel::cube_face::{CubeFace, CubeFaceSet};
use crate::transvoxel::cube_input::CubeInput;
use crate::transvoxel::cube_tetr::CubeTetrMesh;
use crate::transvoxel::cube_vertex::CubeVertexSet;
use itertools::Itertools;
use patina_mesh::ser::encode_test_file;

#[tokio::test]
async fn test_cube_state() -> anyhow::Result<()> {
    let mut face_set = CubeFaceSet::new();
    face_set[CubeFace::new(0, false)] = true;
    let mut vertices = CubeVertexSet::new();
    let mut mesh = CubeInput::new(face_set, CubeEdgeSet::new(), vertices).as_mesh();
    println!("{:#?}", mesh);
    let mesh = mesh.into_mesh();
    encode_test_file(&mesh, "mesh.stl").await?;
    Ok(())
}

#[test]
fn test_volume() {
    for edges in CubeEdge::all().iter().powerset() {
        let mut edge_set = CubeEdgeSet::new();
        for edge in edges {
            edge_set[*edge] = true;
        }
        for faces in CubeFace::all().iter().powerset() {
            let mut face_set = CubeFaceSet::new();
            for face in faces {
                face_set[*face] = true;
            }
            assert_eq!(
                CubeTetrMesh::divided_cube(&face_set, &edge_set).volume6(),
                48
            );
        }
    }
}

#[tokio::test]
async fn test_tetr_alignment() -> anyhow::Result<()> {
    for faces in CubeFace::all().iter().powerset() {
        let mut face_set = CubeFaceSet::new();
        for face in faces {
            face_set[*face] = true;
        }

        let mesh = CubeTetrMesh::divided_cube(&face_set, &CubeEdgeSet::new())
            .as_debug_tri_mesh()
            .into_mesh();
        encode_test_file(&mesh, &format!("mesh{:?}.stl", face_set)).await?;
    }
    for edges in CubeEdge::all().iter().powerset() {
        let mut edge_set = CubeEdgeSet::new();
        for edge in edges {
            edge_set[*edge] = true;
        }

        let mesh = CubeTetrMesh::divided_cube(&CubeFaceSet::new(), &edge_set)
            .as_debug_tri_mesh()
            .into_mesh();
        encode_test_file(&mesh, &format!("mesh{:?}.stl", edge_set)).await?;
    }
    Ok(())
}
