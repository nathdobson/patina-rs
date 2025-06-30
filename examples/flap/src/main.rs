#![feature(exit_status_error)]
#![deny(unused_must_use)]
#![allow(unused_mut)]

use patina_3mf::ModelContainer;
use patina_3mf::content_types::{ContentTypeDefault, ContentTypes};
use patina_3mf::model::build::{Build, Item};
use patina_3mf::model::mesh::{Mesh, Triangle, Triangles, Vertex, Vertices};
use patina_3mf::model::resources::{Component, Components, Object, ObjectType, Resources};
use patina_3mf::model::{Metadata, Model, Unit};
use patina_3mf::model_settings::{
    Assemble, AssembleItem, ModelInstance, ModelSettings, ObjectSettings, Part, Plate,
    SettingsMetadata,
};
use patina_3mf::project_settings::ProjectSettings;
use patina_3mf::relationships::{Relationship, Relationships};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut application_metadata =
        Metadata::new("Application".to_string()).value(Some("BambuStudio-01.10.01.50".to_string()));
    let mesh1 = Mesh::new(
        Vertices::new(vec![
            Vertex::new(0.0, 0.0, 0.0),
            Vertex::new(10.0, 0.0, 0.0),
            Vertex::new(0.0, 10.0, 0.0),
            Vertex::new(0.0, 0.0, 10.0),
        ]),
        Triangles::new(vec![
            Triangle::new(0, 1, 2),
            Triangle::new(0, 3, 1),
            Triangle::new(2, 3, 0),
            Triangle::new(1, 3, 2),
        ]),
    );
    let mesh2 = Mesh::new(
        Vertices::new(vec![
            Vertex::new(0.0, 0.0, 0.0),
            Vertex::new(-10.0, 0.0, 0.0),
            Vertex::new(0.0, -10.0, 0.0),
            Vertex::new(0.0, 0.0, 10.0),
        ]),
        Triangles::new(vec![
            Triangle::new(0, 1, 2),
            Triangle::new(0, 3, 1),
            Triangle::new(2, 3, 0),
            Triangle::new(1, 3, 2),
        ]),
    );
    let object1 = Object::new(1)
        .mesh(Some(mesh1))
        .object_type(Some(ObjectType::Model));
    let object2 = Object::new(2)
        .mesh(Some(mesh2))
        .object_type(Some(ObjectType::Model));
    let object9 = Object::new(9)
        .components(Some(Components::new(vec![
            Component::new(1),
            Component::new(2),
        ])))
        .object_type(Some(ObjectType::Model));
    let model = Model::new()
        .metadata(vec![application_metadata])
        .resources(Resources::new().object(vec![object1, object2, object9]))
        .build(Build::new().item(vec![Item::new(9).printable(Some(true))]))
        .unit(Unit::Millimeter);
    let content_types = ContentTypes::new().defaults(vec![
        ContentTypeDefault::new(
            "model".to_string(),
            "application/vnd.ms-package.3dmanufacturing-3dmodel+xml".to_string(),
        ),
        ContentTypeDefault::new(
            "rels".to_string(),
            "application/vnd.openxmlformats-package.relationships+xml".to_string(),
        ),
    ]);
    let relationships = Relationships::new().relationship(vec![
        Relationship::new()
            .target("/3D/3dmodel.model".to_string())
            .id("rel-1".to_string())
            .typ("http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel".to_string()),
    ]);
    br#"<?xml version="1.0" encoding="UTF-8"?>
<config>
  <object id="9">
    <metadata key="name" value="part1"/>
    <metadata key="extruder" value="1"/>
    <part id="1" subtype="normal_part">
      <metadata key="name" value="part1.stl"/>
      <metadata key="extruder" value="1"/>
    </part>
    <part id="2" subtype="normal_part">
      <metadata key="name" value="part2.stl"/>
      <metadata key="extruder" value="2"/>
    </part>
  </object>
  <plate>
    <metadata key="plater_id" value="1"/>
    <model_instance>
      <metadata key="object_id" value="9"/>
    </model_instance>
  </plate>
  <assemble>
   <assemble_item object_id="9" />
  </assemble>
</config>
"#;
    let model_settings = ModelSettings::new()
        .object(vec![
            ObjectSettings::new("9".to_string())
                .metadata(vec![
                    SettingsMetadata::new("name".to_string()).value(Some("part1".to_string())),
                    SettingsMetadata::new("extruder".to_string()).value(Some("1".to_string())),
                ])
                .part(vec![
                    Part::new("1".to_string())
                        .subtype("normal_part".to_string())
                        .metadata(vec![
                            SettingsMetadata::new("name".to_string())
                                .value(Some("part1.stl".to_string())),
                            SettingsMetadata::new("extruder".to_string())
                                .value(Some("1".to_string())),
                        ]),
                    Part::new("2".to_string())
                        .subtype("normal_part".to_string())
                        .metadata(vec![
                            SettingsMetadata::new("name".to_string())
                                .value(Some("part2".to_string())),
                            SettingsMetadata::new("extruder".to_string())
                                .value(Some("2".to_string())),
                        ]),
                ]),
        ])
        .plate(vec![
            Plate::new()
                .metadata(vec![
                    SettingsMetadata::new("plater_id".to_string()).value(Some("1".to_string())),
                ])
                .model_instance(vec![ModelInstance::new().metadata(vec![
                    SettingsMetadata::new("object_id".to_string()).value(Some("9".to_string())),
                ])]),
        ])
        .assemble(Some(
            Assemble::new().assemble_item(vec![AssembleItem::new("9".to_string())]),
        ));
    let project_settings = ProjectSettings::new()
        .filament_colour(Some(vec![
            "#0000FF".to_string(),
            "#FFFFFF".to_string(),
            "#8E9089".to_string(),
            "#000000".to_string(),
            "#000000".to_string(),
        ]))
        .filament_is_support(Some(vec![false, false, false, false, false]))
        .filament_settings_id(Some(vec![
            "Bambu PLA Basic @BBL A1M".to_string(),
            "Bambu PLA Basic @BBL A1M".to_string(),
            "Bambu PLA Basic @BBL A1M".to_string(),
            "Bambu PLA Basic @BBL A1M".to_string(),
            "Bambu PLA Basic @BBL A1M".to_string(),
        ]))
        .filament_type(Some(vec![
            "PLA".to_string(),
            "PLA".to_string(),
            "PLA".to_string(),
            "PLA".to_string(),
            "PLA".to_string(),
        ]))
        .flush_volumes_matrix(Some(vec![
            0.0, 100.0, 100.0, 100.0, 100.0, //
            100.0, 0.0, 100.0, 100.0, 100.0, //
            100.0, 100.0, 0.0, 100.0, 100.0, //
            100.0, 100.0, 100.0, 0.0, 100.0, //
            100.0, 100.0, 100.0, 100.0, 0.0, //
        ]))
        .nozzle_diameter(Some(vec![0.4]))
        .print_settings_id(Some("0.20mm Standard @BBL A1M".to_string()))
        .printable_height(Some(180.0))
        .printer_settings_id(Some("Bambu Lab A1 mini 0.4 nozzle".to_string()))
        .enable_prime_tower(Some(true))
        .wipe_tower_x(Some(50.0))
        .wipe_tower_y(Some(50.0));

    let model_cont = ModelContainer::new(model)
        .content_types(Some(content_types))
        .relationships(Some(relationships))
        .model_settings(Some(model_settings))
        .project_settings(Some(project_settings));

    let encoded = model_cont.encode()?;
    tokio::fs::write("examples/flap/output.3mf", encoded).await?;
    tokio::fs::remove_dir_all("examples/flap/output").await.ok();
    tokio::fs::create_dir("examples/flap/output").await.ok();

    tokio::process::Command::new("unzip")
        .arg("-q")
        .arg("../output.3mf")
        .current_dir("examples/flap/output")
        .spawn()?
        .wait()
        .await?
        .exit_ok()?;
    let mut slicer =
        tokio::process::Command::new("/Applications/BambuStudio.app/Contents/MacOS/BambuStudio");
    slicer.arg("--debug").arg("2");
    slicer.arg("--slice").arg("0");
    let filament = "/Users/nathan/Library/Application Support/BambuStudio/system/BBL/filament/Bambu PLA Basic @BBL A1M 0.2 nozzle.json";
    slicer.arg("--load-filaments").arg(filament);
    let machine = "/Users/nathan/Library/Application Support/BambuStudio/system/BBL/machine/Bambu Lab A1 mini 0.4 nozzle.json";
    let process = "/Users/nathan/Library/Application Support/BambuStudio/system/BBL/process/0.20mm Standard @BBL A1M.json";
    slicer.arg("--enable-timelapse");
    slicer.arg("--timelapse-type=1");
    slicer
        .arg("--load-settings")
        .arg(format!("{};{}", machine, process));
    slicer
        .arg("--export-3mf")
        .arg("/Users/nathan/Documents/workspace/patina/examples/flap/sliced.3mf");
    slicer.arg("examples/flap/output.3mf");
    slicer.spawn()?.wait().await?.exit_ok()?;
    tokio::fs::remove_dir_all("examples/flap/sliced").await.ok();
    tokio::fs::create_dir("examples/flap/sliced").await.ok();
    tokio::process::Command::new("unzip")
        .arg("-q")
        .arg("../sliced.3mf")
        .current_dir("examples/flap/sliced")
        .spawn()?
        .wait()
        .await?
        .exit_ok()?;
    Ok(())
}
