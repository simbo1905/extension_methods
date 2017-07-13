
extern crate serde;
extern crate serde_json;

extern crate some_library;
use some_library::Shape;

use serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Debug)]
pub struct Shaper(pub Shape);

impl Serialize for Shaper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match self.0 {
            Shape::Circle { x, y, radius } => {
                // 3 is the number of fields in the struct.
                let mut state = serializer.serialize_struct("Circle", 3)?;
                state.serialize_field("x", &x)?;
                state.serialize_field("y", &y)?;
                state.serialize_field("r", &radius)?;
                state.end()
            }
                
            Shape::Rect { x, y, width, height } => {
                // 4 is the number of fields in the struct.
                let mut state = serializer.serialize_struct("Rect", 3)?;
                state.serialize_field("x", &x)?;
                state.serialize_field("y", &y)?;
                state.serialize_field("width", &width)?;
                state.serialize_field("height", &height)?;
                state.end()
            }
        }
    }
}

pub trait JSON {
    fn to_json(&self) -> String
            where Self: Serialize {
        return serde_json::to_string(&self).unwrap();
    }
}

impl JSON for Shaper {
    // add code here
}
