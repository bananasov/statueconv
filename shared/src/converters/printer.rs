use crate::formats::printer::{Print, Shape};
use crate::formats::statue::{Cube, Statue};

impl From<Statue> for Print {
    fn from(value: Statue) -> Self {
        Self {
            label: value.name,
            tooltip: None,
            is_button: Some(false),
            collide_when_off: Some(true),
            collide_when_on: Some(true),
            light_level: value.light_level,
            redstone_level: None,
            shapes_off: convert_cubes_to_shapes(value.cubes),
            shapes_on: Vec::new(),
        }
    }
}

fn convert_cubes_to_shapes(cubes: Vec<Cube>) -> Vec<Shape> {
    let mut shapes = Vec::new();

    for cube in cubes {
        let tint = match cube.tint {
            Some(tint) => Some(tint.to_string()),
            None => None,
        };

        let shape = Shape {
            bounds: cube.bounds.into(),
            texture: cube.texture,
            tint,
        };

        shapes.push(shape);
    }

    shapes
}
