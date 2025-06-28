#[deny(unused_must_use)]
use std::io::{Cursor, Write};
use std::mem;
use zip::ZipWriter;
use zip::result::ZipResult;
use zip::write::SimpleFileOptions;

#[test]
fn test() {}

pub struct Model {}

impl Model {
    pub fn encode(&self) -> ZipResult<Vec<u8>> {
        let mut buffer = vec![];
        let mut zip = ZipWriter::new(Cursor::new(&mut buffer));
        let opts = SimpleFileOptions::default();
        zip.start_file("[Content_Types].xml", opts.clone())?;
        zip.write_all(br#"<?xml version="1.0" encoding="UTF-8"?>
            <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
                <Default Extension="model" ContentType="application/vnd.ms-package.3dmanufacturing-3dmodel+xml"/>
                <Default Extension="png" ContentType="image/png"/>
                <Default Extension="gcode" ContentType="text/x.gcode"/>
            </Types>
        "#)?;
        zip.add_directory("_rels", opts.clone())?;
        zip.start_file("_rels/.rels", opts.clone())?;
        zip.write_all(br#"<?xml version="1.0" encoding="UTF-8"?>
            <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                <Relationship Target="/3D/3dmodel.model" Id="rel-1" Type="http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel"/>
            </Relationships>
        "#)?;
        zip.add_directory("3D/", opts.clone())?;
        zip.add_directory("3D/_rels", opts.clone())?;
        zip.start_file("3D/_rels/3dmodel.model.rels", opts.clone())?;
        zip.write_all(br#"<?xml version="1.0" encoding="UTF-8"?>
            <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                <Relationship Target="/3D/Objects/object_1.model" Id="rel-1" Type="http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel"/>
            </Relationships>
        "#)?;
        zip.start_file("3D/3dmodel.model", opts.clone())?;
        zip.write_all(br#"<?xml version="1.0" encoding="UTF-8"?>
            <model unit="millimeter" xml:lang="en-US" xmlns="http://schemas.microsoft.com/3dmanufacturing/core/2015/02" xmlns:BambuStudio="http://schemas.bambulab.com/package/2021" xmlns:p="http://schemas.microsoft.com/3dmanufacturing/production/2015/06" requiredextensions="p">
                <metadata name="Application">BambuStudio-02.01.00.59</metadata>
                <metadata name="BambuStudio:3mfVersion">1</metadata>
                <metadata name="Copyright"></metadata>
                <metadata name="CreationDate">2025-06-05</metadata>
                <metadata name="Description"></metadata>
                <metadata name="Designer"></metadata>
                <metadata name="DesignerCover"></metadata>
                <metadata name="DesignerUserId">2800368633</metadata>
                <metadata name="License"></metadata>
                <metadata name="ModificationDate">2025-06-05</metadata>
                <metadata name="Origin"></metadata>
                <metadata name="Title"></metadata>
                <resources>
                  <object id="3" p:UUID="00000001-61cb-4c03-9d28-80fed5dfa1dc" type="model">
                    <components>
                      <component p:path="/3D/Objects/object_1.model" objectid="1" p:UUID="00010000-b206-40ff-9872-83e8017abed1" transform="1 0 0 0 1 0 0 0 1 0 0 0"/>
                      <component p:path="/3D/Objects/object_1.model" objectid="2" p:UUID="00010001-b206-40ff-9872-83e8017abed1" transform="1 0 0 0 1 0 0 0 1 -0.0397386551 -4.09487247 0.300000012"/>
                    </components>
                  </object>
                </resources>
                <build p:UUID="2c7c17d8-22b5-4d84-8835-1976022ea369">
                    <item objectid="3" p:UUID="00000003-b1ec-4553-aec9-835e5b724bb4" transform="-1 -1.2246468e-16 0 1.2246468e-16 -1 0 0 0 1 157.50001 142.86583 0.5" printable="1"/>
                </build>
            </model>
        "#)?;
        zip.add_directory("3D/Objects", opts.clone())?;
        zip.start_file("3D/Objects/object_1.model", opts.clone())?;
        zip.write_all(br#"<?xml version="1.0" encoding="UTF-8"?>
            <model unit="millimeter" xml:lang="en-US" xmlns="http://schemas.microsoft.com/3dmanufacturing/core/2015/02" xmlns:BambuStudio="http://schemas.bambulab.com/package/2021" xmlns:p="http://schemas.microsoft.com/3dmanufacturing/production/2015/06" requiredextensions="p">
                <metadata name="BambuStudio:3mfVersion">1</metadata>
                <resources>
                    <object id="1" p:UUID="00010000-81cb-4c03-9d28-80fed5dfa1dc" type="model">
                        <mesh>
                            <vertices>
                                <vertex x="-4.05348825" y="-17.5" z="0.100000024"/>
                                <vertex x="-4.68930483" y="9.31025505" z="0.100000024"/>
                                <vertex x="-4.05348825" y="-17.5" z="0.5"/>
                                <vertex x="-4.68930483" y="9.31025505" z="0.5"/>
                                <vertex x="4.02699566" y="-17.5" z="0.5"/>
                                <vertex x="4.609828" y="9.31025505" z="0.5"/>
                                <vertex x="4.02699566" y="-17.5" z="0.100000024"/>
                                <vertex x="4.609828" y="9.31025505" z="0.100000024"/>
                                <vertex x="-19.5" y="0.5" z="0.5"/>
                                <vertex x="-19.5" y="-15.1000004" z="0.5"/>
                                <vertex x="-19.5" y="-16.2999992" z="0.5"/>
                                <vertex x="-21.5" y="-15.1000004" z="0.5"/>
                                <vertex x="-21.5" y="-16.2999992" z="0.5"/>
                                <vertex x="-21.5" y="17.5" z="0.5"/>
                                <vertex x="21.5" y="17.5" z="0.5"/>
                                <vertex x="19.5" y="0.5" z="0.5"/>
                                <vertex x="21.5" y="0.5" z="0.5"/>
                                <vertex x="19.5" y="-15.1000004" z="0.5"/>
                                <vertex x="19.5" y="-16.2999992" z="0.5"/>
                                <vertex x="21.5" y="-16.2999992" z="0.5"/>
                                <vertex x="19.5" y="-17.5" z="0.5"/>
                                <vertex x="21.5" y="-15.1000004" z="0.5"/>
                                <vertex x="-21.5" y="0.5" z="0.5"/>
                                <vertex x="-19.5" y="-17.5" z="0.5"/>
                                <vertex x="-19.5" y="0.5" z="-0.5"/>
                                <vertex x="-19.5" y="-15.1000004" z="-0.5"/>
                                <vertex x="-21.5" y="0.5" z="-0.5"/>
                                <vertex x="-21.5" y="17.5" z="-0.5"/>
                                <vertex x="21.5" y="17.5" z="-0.5"/>
                                <vertex x="21.5" y="0.5" z="-0.5"/>
                                <vertex x="19.5" y="0.5" z="-0.5"/>
                                <vertex x="19.5" y="-15.1000004" z="-0.5"/>
                                <vertex x="21.5" y="-15.1000004" z="-0.5"/>
                                <vertex x="21.5" y="-16.2999992" z="-0.5"/>
                                <vertex x="19.5" y="-16.2999992" z="-0.5"/>
                                <vertex x="19.5" y="-17.5" z="-0.5"/>
                                <vertex x="-19.5" y="-17.5" z="-0.5"/>
                                <vertex x="-19.5" y="-16.2999992" z="-0.5"/>
                                <vertex x="-21.5" y="-16.2999992" z="-0.5"/>
                                <vertex x="-21.5" y="-15.1000004" z="-0.5"/>
                            </vertices>
                            <triangles>
                                <triangle v1="0" v2="1" v3="2"/>
                                <triangle v1="2" v2="1" v3="3"/>
                                <triangle v1="4" v2="5" v3="6"/>
                                <triangle v1="6" v2="5" v3="7"/>
                                <triangle v1="5" v2="3" v3="7"/>
                                <triangle v1="7" v2="3" v3="1"/>
                                <triangle v1="6" v2="7" v3="0"/>
                                <triangle v1="0" v2="7" v3="1"/>
                                <triangle v1="3" v2="8" v3="2"/>
                                <triangle v1="2" v2="8" v3="9"/>
                                <triangle v1="2" v2="9" v3="10"/>
                                <triangle v1="10" v2="9" v3="11"/>
                                <triangle v1="10" v2="11" v3="12"/>
                                <triangle v1="8" v2="3" v3="13"/>
                                <triangle v1="13" v2="3" v3="5"/>
                                <triangle v1="13" v2="5" v3="14"/>
                                <triangle v1="14" v2="5" v3="15"/>
                                <triangle v1="14" v2="15" v3="16"/>
                                <triangle v1="5" v2="4" v3="15"/>
                                <triangle v1="15" v2="4" v3="17"/>
                                <triangle v1="17" v2="4" v3="18"/>
                                <triangle v1="17" v2="18" v3="19"/>
                                <triangle v1="4" v2="20" v3="18"/>
                                <triangle v1="19" v2="21" v3="17"/>
                                <triangle v1="13" v2="22" v3="8"/>
                                <triangle v1="10" v2="23" v3="2"/>
                                <triangle v1="24" v2="25" v3="8"/>
                                <triangle v1="8" v2="25" v3="9"/>
                                <triangle v1="26" v2="24" v3="22"/>
                                <triangle v1="22" v2="24" v3="8"/>
                                <triangle v1="27" v2="26" v3="13"/>
                                <triangle v1="13" v2="26" v3="22"/>
                                <triangle v1="28" v2="27" v3="14"/>
                                <triangle v1="14" v2="27" v3="13"/>
                                <triangle v1="29" v2="28" v3="16"/>
                                <triangle v1="16" v2="28" v3="14"/>
                                <triangle v1="30" v2="29" v3="15"/>
                                <triangle v1="15" v2="29" v3="16"/>
                                <triangle v1="31" v2="30" v3="17"/>
                                <triangle v1="17" v2="30" v3="15"/>
                                <triangle v1="32" v2="31" v3="21"/>
                                <triangle v1="21" v2="31" v3="17"/>
                                <triangle v1="33" v2="32" v3="19"/>
                                <triangle v1="19" v2="32" v3="21"/>
                                <triangle v1="34" v2="33" v3="18"/>
                                <triangle v1="18" v2="33" v3="19"/>
                                <triangle v1="35" v2="34" v3="20"/>
                                <triangle v1="20" v2="34" v3="18"/>
                                <triangle v1="2" v2="23" v3="0"/>
                                <triangle v1="0" v2="23" v3="36"/>
                                <triangle v1="0" v2="36" v3="6"/>
                                <triangle v1="6" v2="36" v3="35"/>
                                <triangle v1="6" v2="35" v3="20"/>
                                <triangle v1="20" v2="4" v3="6"/>
                                <triangle v1="37" v2="36" v3="10"/>
                                <triangle v1="10" v2="36" v3="23"/>
                                <triangle v1="38" v2="37" v3="12"/>
                                <triangle v1="12" v2="37" v3="10"/>
                                <triangle v1="39" v2="38" v3="11"/>
                                <triangle v1="11" v2="38" v3="12"/>
                                <triangle v1="25" v2="39" v3="9"/>
                                <triangle v1="9" v2="39" v3="11"/>
                                <triangle v1="39" v2="25" v3="38"/>
                                <triangle v1="38" v2="25" v3="37"/>
                                <triangle v1="37" v2="25" v3="31"/>
                                <triangle v1="37" v2="31" v3="34"/>
                                <triangle v1="34" v2="31" v3="32"/>
                                <triangle v1="34" v2="32" v3="33"/>
                                <triangle v1="31" v2="25" v3="30"/>
                                <triangle v1="30" v2="25" v3="24"/>
                                <triangle v1="30" v2="24" v3="28"/>
                                <triangle v1="28" v2="24" v3="27"/>
                                <triangle v1="27" v2="24" v3="26"/>
                                <triangle v1="28" v2="29" v3="30"/>
                                <triangle v1="35" v2="36" v3="34"/>
                                <triangle v1="34" v2="36" v3="37"/>
                            </triangles>
                        </mesh>
                    </object>
                    <object id="2" p:UUID="00010001-81cb-4c03-9d28-80fed5dfa1dc" type="model">
                        <mesh>
                            <vertices>
                                <vertex x="-4.64956665" y="13.4051275" z="-0.199999988"/>
                                <vertex x="-4.01375008" y="-13.4051275" z="-0.199999988"/>
                                <vertex x="-4.64956665" y="13.4051275" z="0.199999988"/>
                                <vertex x="-4.01375008" y="-13.4051275" z="0.199999988"/>
                                <vertex x="4.64956665" y="13.4051275" z="-0.199999988"/>
                                <vertex x="4.64956665" y="13.4051275" z="0.199999988"/>
                                <vertex x="4.06673431" y="-13.4051275" z="-0.199999988"/>
                                <vertex x="4.06673431" y="-13.4051275" z="0.199999988"/>
                            </vertices>
                            <triangles>
                                <triangle v1="0" v2="1" v3="2"/>
                                <triangle v1="2" v2="1" v3="3"/>
                                <triangle v1="4" v2="0" v3="5"/>
                                <triangle v1="5" v2="0" v3="2"/>
                                <triangle v1="6" v2="4" v3="7"/>
                                <triangle v1="7" v2="4" v3="5"/>
                                <triangle v1="1" v2="6" v3="3"/>
                                <triangle v1="3" v2="6" v3="7"/>
                                <triangle v1="7" v2="5" v3="3"/>
                                <triangle v1="3" v2="5" v3="2"/>
                                <triangle v1="6" v2="1" v3="4"/>
                                <triangle v1="4" v2="1" v3="0"/>
                            </triangles>
                        </mesh>
                    </object>
                </resources>
                <build/>
            </model>
        "#)?;
        zip.add_directory("Metadata",opts.clone())?;
        zip.start_file("Metadata/model_settings.config",opts.clone())?;
        zip.write_all(br#"<?xml version="1.0" encoding="UTF-8"?>
<config>
</config>"#)?;
        zip.finish()?;
        Ok(buffer)
    }
}
