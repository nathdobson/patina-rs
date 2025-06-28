pub mod mesh;
pub mod model;

use crate::threemf::model::Model;
use serde::{Deserialize, Serialize};
#[deny(unused_must_use)]
use std::io::{Cursor, Write};
use std::mem;
use zip::ZipWriter;
use zip::result::ZipResult;
use zip::write::SimpleFileOptions;

pub struct ModelContainer {
    pub model: Model,
}

const CONTENT_TYPES_FILE: &[u8] = //
    br#"<?xml version="1.0" encoding="utf-8"?>
    <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
        <Default
            Extension="model"
            ContentType="application/vnd.ms-package.3dmanufacturing-3dmodel+xml" />
        <Default
            Extension="rels"
            ContentType="application/vnd.openxmlformats-package.relationships+xml" />
        <Default
            Extension="texture"
            ContentType="application/vnd.ms-package.3dmanufacturing-3dmodeltexture" />
    </Types>
"#;

const RELS_FILE:&[u8]=//
    br#"<?xml version="1.0" encoding="UTF-8"?>
            <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                <Relationship Target="/3D/model.model" Id="rel-1" Type="http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel"/>
            </Relationships>
        "#;

impl ModelContainer {
    pub fn encode(&self) -> anyhow::Result<Vec<u8>> {
        let mut buffer = vec![];
        let mut zip = ZipWriter::new(Cursor::new(&mut buffer));
        let opts = SimpleFileOptions::default();
        zip.start_file("[Content_Types].xml", opts.clone())?;
        zip.write_all(CONTENT_TYPES_FILE)?;
        zip.add_directory("_rels", opts.clone())?;
        zip.start_file("_rels/.rels", opts.clone())?;
        zip.write_all(RELS_FILE)?;
        zip.add_directory("3D/", opts.clone())?;
        zip.start_file("3D/model.model", opts.clone())?;
        zip.write_all(serde_xml_rs::to_string(&self.model)?.as_bytes())?;
        // zip.write_all(br#"<?xml version="1.0" encoding="utf-8"?>
        //     <model xmlns="http://schemas.microsoft.com/3dmanufacturing/core/2015/02" unit="millimeter">
        //       <metadata name="a">b</metadata>
        //       <resources>
        //         <object id="1">
        //           <mesh>
        //             <vertices>
        //               <vertex x="0" y="0" z="0"/>
        //               <vertex x="100" y="0" z="0"/>
        //               <vertex x="0" y="100" z="0"/>
        //               <vertex x="0" y="0" z="100"/>
        //             </vertices>
        //             <triangles>
        //               <triangle v1="0" v2="1" v3="2"/>
        //               <triangle v1="1" v2="0" v3="3"/>
        //               <triangle v1="2" v2="1" v3="3"/>
        //               <triangle v1="0" v2="2" v3="3"/>
        //             </triangles>
        //           </mesh>
        //         </object>
        //         <object id="2">
        //           <mesh>
        //             <vertices>
        //               <vertex x="100" y="100" z="100"/>
        //               <vertex x="100" y="0" z="0"/>
        //               <vertex x="0" y="100" z="0"/>
        //               <vertex x="0" y="0" z="100"/>
        //             </vertices>
        //             <triangles>
        //               <triangle v1="0" v2="1" v3="2"/>
        //               <triangle v1="1" v2="0" v3="3"/>
        //               <triangle v1="2" v2="1" v3="3"/>
        //               <triangle v1="0" v2="2" v3="3"/>
        //             </triangles>
        //           </mesh>
        //         </object>
        //         <object id="3">
        //           <components>
        //             <component objectid="1"/>
        //             <component objectid="2"/>
        //           </components>
        //         </object>
        //       </resources>
        //       <build>
        //         <item objectid="3"/>
        //       </build>
        //     </model>
        // "#)?;
        zip.finish()?;
        Ok(buffer)
    }
}
