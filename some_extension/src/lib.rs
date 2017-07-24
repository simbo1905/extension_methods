
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate some_library;
use some_library::Shape;

use serde::ser::{Serialize};

// This enum is onlu used to generate the Serialize method using serde_derive
// https://serde.rs/remote-derive.html
// Serde calls this the definition of the remote type. It is just a copy of the
// remote type. The `remote` attribute gives the path to the actual type.
#[derive(Serialize)]
#[serde(remote = "Shape")]
pub enum ShapeDef {
    Circle { x: u32, y: u32, radius: u32},
    Rect { x: u32, y: u32, width: u32, height: u32}
}

// This is the "newtype" pattern. It serialises the external 
// create class using the serializer definition generated for the local type.
// https://serde.rs/remote-derive.html
// Now the remote type can be used almost like it had its own Serialize and
// Deserialize impls all along. The `with` attribute gives the path to the
// definition for the remote type. Note that the real type of the field is the
// remote type, not the definition type.
#[derive(Serialize)]
#[derive(Debug)]
pub struct Shaper(
    #[serde(with = "ShapeDef")]
    pub Shape
);

// This is the extention method trait which has a default to_json using serde
pub trait JSON {
    fn to_json(&self) -> String
            where Self: Serialize {
        return serde_json::to_string(&self).unwrap();
    }
}

// This adds the exention method trait to the newtype pattern object
impl JSON for Shaper {
    // add code here
}
