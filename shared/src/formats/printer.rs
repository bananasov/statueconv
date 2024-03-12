use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Print {
    /// The name of the 3D print, maximum 48 characters.
    pub label: Option<String>,
    /// The tooltip of the 3D print in the inventory, maximum 256 characters.
    pub tooltip: Option<String>,
    #[serde(rename = "isButton")]
    /// Whether the 3D print acts as a button when right-clicked.
    /// If true, the print will automatically switch to the `off` state after 20 ticks when right-clicked.
    /// If false, right-clicking will toggle the state.
    pub is_button: Option<bool>,
    #[serde(rename = "collideWhenOff")]
    /// Whether the 3D print is collidable when in the `off` state.
    pub collide_when_off: Option<bool>,
    #[serde(rename = "collideWhenOn")]
    /// Whether the 3D print is collidable when in the `on` state.
    pub collide_when_on: Option<bool>,
    #[serde(rename = "lightLevel")]
    /// The light level of the 3D print. Must be between 0 or 15,
    /// but values above 7 will be clamped to 7 unless the print is later crafted with glowstone dust.
    pub light_level: Option<u8>,
    #[serde(rename = "redstoneLevel")]
    /// The redstone level of the 3D print. Must be between 0 or 15.
    pub redstone_level: Option<i8>,
    #[serde(rename = "shapesOff")]
    /// The shapes of the 3D print when in the `off` state.
    /// Each object in the array must have a `bounds` property with the bounds of the shape,
    /// a `texture` property with the texture of the shape, and an optional `tint` property with the tint of the shape,
    /// which may be a number or a hex string (`RRGGBB)`.
    pub shapes_off: Vec<Shape>,
    #[serde(rename = "shapesOn")]
    /// Same as `shapes_off`, but for the `on` state. To disallow state changes and have no `on` state, pass an empty array.
    pub shapes_on: Vec<Shape>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Shape {
    /// The bounds of the shape, in the format `[minX, minY, minZ, maxX, maxY, maxZ]`.
    /// Numbers must be between 0 and 16 inclusive (16 is the edge of the block).
    pub bounds: Vec<u8>,
    /// The texture of the shape, including the namespace. For example, `minecraft:block/stone`
    pub texture: String,
    /// The tint of the shape, as a hex string in the format `RRGGBB`, or a single decimal value.
    pub tint: Option<String>,
}

impl Default for Print {
    fn default() -> Self {
        Self {
            label: None,
            tooltip: None,
            is_button: Some(false),
            collide_when_off: Some(true),
            collide_when_on: Some(false),
            light_level: None,
            redstone_level: Default::default(),
            shapes_off: Default::default(),
            shapes_on: Default::default(),
        }
    }
}
