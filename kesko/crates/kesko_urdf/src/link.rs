use std::{fmt, str};

use quick_xml::de;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Link {
    #[serde(rename = "@name")]
    pub name: String,
    pub inertial: Option<Inertial>,
    pub visual: Option<Visual>,
    pub collision: Option<Collision>,
}
impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        if let Some(inertial) = &self.inertial {
            writeln!(f, "Inertial: {:?}", inertial)?;
        }
        if let Some(visual) = &self.visual {
            writeln!(f, "Visual: {:?}", visual)?;
        }
        if let Some(collision) = &self.collision {
            writeln!(f, "Collision: {:?}", collision)?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Visual {
    #[serde(rename = "@name")]
    name: Option<String>,
    origin: Option<Origin>,
    geometry: GeometryType,
    material: Option<Material>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Collision {
    name: String,
    origin: Option<Origin>,
    geometry: GeometryType,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Inertial {
    origin: Option<Origin>,
    mass: Mass,
    inertia: Inertia,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Origin {
    #[serde(rename = "@xyz")]
    xyz: String,
    #[serde(rename = "@rpy")]
    rpy: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Inertia {
    #[serde(rename = "@ixx")]
    ixx: String,
    #[serde(rename = "@ixy")]
    ixy: String,
    #[serde(rename = "@ixz")]
    ixz: String,
    #[serde(rename = "@iyy")]
    iyy: String,
    #[serde(rename = "@iyz")]
    iyz: String,
    #[serde(rename = "@izz")]
    izz: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Mass {
    #[serde(rename = "@value")]
    value: String,
}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GeometryType {
    #[serde(rename = "$value")]
    geometry: Geometry,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Geometry {
    Box {
        #[serde(rename = "@size")]
        size: String,
    },
    Cylinder {
        radius: String,
        length: String,
    },
    Sphere {
        radius: String,
    },
    Mesh {
        filename: String,
        scale: String,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Material {
    #[serde(rename = "@name")]
    name: String,
    color: Option<Color>,
    texture: Option<Texture>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Color {
    #[serde(rename = "@rgba")]
    rgba: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Texture {
    #[serde(rename = "@filename")]
    filename: String,
}
