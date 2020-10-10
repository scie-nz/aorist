use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    fn getz(&self) -> f64 {
        self.z
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum AbstractPoint {
    Point { x: f64, y: f64 },
    Point3D { x: f64, y: f64, z: f64 },
}

fn main() -> Result<(), serde_yaml::Error> {
    let point = Point { x: 1.0, y: 2.0 };

    let s = serde_yaml::to_string(&point)?;
    assert_eq!(s, "---\nx: 1.0\ny: 2.0");

    let s_t = "---\nx: 1.0\ny: 2.0\ntype: Point";
    let deserialized_point: Point = serde_yaml::from_str(&s)?;
    assert_eq!(point, deserialized_point);
    let deserialized_point: Point = serde_yaml::from_str(&s_t)?;
    assert_eq!(point, deserialized_point);

    let s3d = "---\nx: 1.0\ny: 2.0\nz: 3.0\ntype: Point3D";
    let deserialized_point: AbstractPoint = serde_yaml::from_str(&s3d)?;
    println!("{}", serde_yaml::to_string(&deserialized_point)?);
    println!("{}", deserialized_point.getz());
    Ok(())
}
