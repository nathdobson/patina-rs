use patina_vec::vec3::Vec3;
use std::io::Write;

#[derive(Clone, Debug)]
pub struct BrimPoint {
    pub pos: Vec3,
    pub radius: f64,
}

pub struct PartBrimPoints {
    pub object_id: usize,
    pub points: Vec<BrimPoint>,
}

pub struct BrimPoints {
    pub parts: Vec<PartBrimPoints>,
}

impl BrimPoints {
    pub fn serialize(&self, output: &mut Vec<u8>) {
        writeln!(output, "brim_points_format_version=1").unwrap();
        for part in &self.parts {
            part.serialize(output);
        }
    }
}

impl PartBrimPoints {
    pub fn serialize(&self, output: &mut Vec<u8>) {
        write!(output, "object_id={}|", self.object_id).unwrap();
        for point in &self.points {
            point.serialize(output);
        }
        writeln!(output).unwrap();
    }
}

impl BrimPoint {
    pub fn serialize(&self, output: &mut Vec<u8>) {
        write!(
            output,
            "{} {} {} {} {} ",
            self.pos.x(),
            self.pos.y(),
            self.pos.z(),
            self.radius,
            -1
        )
        .unwrap();
    }
}
