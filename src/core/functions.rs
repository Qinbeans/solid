//map out the functions
//https://docs.rs/noise/latest/noise/
use serde::{Deserialize, Serialize};

//store results into a vector

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Default for Vector2D {
    fn default() -> Self {
        Vector2D {
            x: 0.0,
            y: 0.0
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vector3D {
    fn default() -> Self {
        Vector3D {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vector4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Default for Vector4D {
    fn default() -> Self {
        Vector4D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0
        }
    }
}

#[derive(Clone, Debug)]
pub struct Vector4T<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

//allow adding vectors
impl <T> std::ops::Add for Vector4T<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector4T {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Value {
    String(String),
    Float(f64),
    Int(i64),
    Function(Function),
    Vector2D(Vector2D),
    Vector3D(Vector3D),
    Vector4D(Vector4D),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Parameter {
    Id(String),
    Ui(String),
    Position(Vector3D),
    Size(Vector2D),
    Type(String),
    Font(f64),
    Text(String),
    Color(Vector4D),
    String(String),
    Actions(Vec<Function>),
    Rotation(f64),
    Vector3D(Vector3D),
    Vector2D(Vector2D),
    Vector4D(Vector4D),
    Scene(String),
    Texture(String),
    Int(i64),
    Float(f64),
    Boolean(bool),
    Max(i64),
    Min(i64),
    Default(Value),
    Placeholder(String),
    Options(Vec<Value>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
}

//functions
// - generate_map
// - render_map
// - spawn_window
// - spawn_entity
// - spawn_player //not inherantly playable
// - spawn_npc //special entity that can be interacted with
// - spawn_item //storable
