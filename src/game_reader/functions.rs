//map out the functions
//https://docs.rs/noise/latest/noise/
use serde::{Deserialize, Serialize};

//store results into a vector

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vector4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
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
}

#[derive(Clone, Debug)]
pub struct Results {
    pub values: Vec<Value>,
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
