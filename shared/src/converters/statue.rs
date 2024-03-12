use crate::formats::printer::{Print, Shape};
use crate::formats::statue::{Bounds, Cube, Statue};

impl From<Print> for Statue {
    fn from(value: Print) -> Self {
        Self {
            name: value.label,
            author: None,
            light_level: value.light_level,
            cubes: convert_shapes_to_cubes(value.shapes_off),
        }
    }
}

fn convert_shapes_to_cubes(shapes: Vec<Shape>) -> Vec<Cube> {
    let mut cubes = Vec::new();
    for shape in shapes {
        let bounds = Bounds {
            x1: shape.bounds[0],
            y1: shape.bounds[1],
            z1: shape.bounds[2],
            x2: shape.bounds[3],
            y2: shape.bounds[4],
            z2: shape.bounds[5],
        };

        let tint = match shape.tint {
            Some(tint) => Some(u32::from_str_radix(&tint, 16).unwrap()),
            None => None,
        };

        let cube = Cube {
            texture: shape.texture,
            tint: tint,
            bounds,
        };

        cubes.push(cube);
    }

    cubes
}