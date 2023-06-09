use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Joint {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub joint_type: JointType,
    #[serde(rename = "parent")]
    pub parent: ParentLink,
    #[serde(rename = "child")]
    pub child: ChildLink,
}

impl fmt::Display for Joint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\tName: {}\n\tType: {:?}", self.name, self.joint_type)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum JointType {
    Revolute,
    Continuous,
    Prismatic,
    Fixed,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename = "parent")]
pub struct ParentLink {
    #[serde(rename = "@link")]
    pub link: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename = "child")]
pub struct ChildLink {
    #[serde(rename = "@link")]
    pub link: String,
}

pub struct Axis {
    xyz: String,
}

pub struct Limit {
    lower: f32,
    upper: f32,
}
