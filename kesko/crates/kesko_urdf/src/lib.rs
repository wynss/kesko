mod joint;
mod link;

use std::fmt;

use quick_xml::de::from_str;
use serde::Deserialize;

use crate::joint::Joint;
use crate::link::Link;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "robot")]
pub struct RawRobot {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub elements: Vec<Element>,
}
impl RawRobot {
    pub fn from_urdf_str(s: &str) -> Self {
        from_str(s).unwrap()
    }
}

impl fmt::Display for RawRobot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Robot Configuration\n\
            -------------------\n\
            Name: {}",
            self.name
        )?;

        writeln!(f, "Elements:")?;
        for el in self.elements.iter() {
            match el {
                Element::Link(link) => writeln!(f, "Link\n{}", link)?,
                Element::Joint(joint) => writeln!(f, "Joint\n{}", joint)?,
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Element {
    Link(Link),
    Joint(Joint),
}
