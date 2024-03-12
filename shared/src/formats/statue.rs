// `statue` is a custom format specifically made for UnlimitedPeripheralWorks statues
// It bears ressemblance to sc-peripherals' 3DJ format.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Statue {
    pub name: Option<String>,
    pub author: Option<String>,
    pub light_level: Option<u8>,
    pub cubes: Vec<Cube>,
}

pub struct StatueBuilder {
    statue: Statue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cube {
    pub texture: String,
    pub tint: Option<u32>,
    pub bounds: Bounds,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bounds {
    pub x1: u8,
    pub y1: u8,
    pub z1: u8,
    pub x2: u8,
    pub y2: u8,
    pub z2: u8,
}

impl From<Bounds> for Vec<u8> {
    fn from(value: Bounds) -> Self {
        vec![value.x1, value.y1, value.z1, value.x2, value.y2, value.z2]
    }
}

impl Statue {
    pub fn builder() -> StatueBuilder {
        StatueBuilder {
            statue: Statue::new(None),
        }
    }

    pub fn new(name: Option<String>) -> Self {
        Statue {
            name,
            author: None,
            light_level: None,
            cubes: Vec::new(),
        }
    }
}

impl StatueBuilder {
    pub fn with_name(mut self, name: String) -> StatueBuilder {
        self.statue.name = Some(name);
        self
    }

    pub fn with_author(mut self, author: String) -> StatueBuilder {
        self.statue.author = Some(author);
        self
    }

    pub fn with_light_level(mut self, level: u8) -> StatueBuilder {
        self.statue.light_level = Some(level);
        self
    }

    pub fn with_cubes(mut self, cubes: Vec<Cube>) -> StatueBuilder {
        self.statue.cubes = cubes;
        self
    }
}

impl Default for Statue {
    fn default() -> Self {
        Self::new(None)
    }
}
