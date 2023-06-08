use serde::Deserialize;
use serde_xml_rs::from_str;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Robot {
    pub name: String,
    pub links: Vec<Link>,
}
impl Robot {
    pub fn from_urdf_str(s: &str) -> Self {
        from_str(s).unwrap()
    }

    pub fn print(&self) {
        println!("Name: {}", self.name);

        for link in self.links.iter() {
            println!("Link\n\tname: {}", link.name)
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Link {
    pub name: String,
}
