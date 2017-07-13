extern crate some_library;
extern crate some_extension;
extern crate serde_json;

use some_library::Shape;
use some_extension::Shaper;

fn main() {
    let circle = Shaper(Shape::Circle{ x: 11, y: 22, radius: 33 });
    let ser_c = serde_json::to_string(&circle).unwrap();
    println!("{}", ser_c);
    let rect = Shaper(Shape::Rect{ x: 44, y: 55, width: 66, height: 77} );
    let ser_r = serde_json::to_string(&rect).unwrap();
    println!("{}", ser_r);
}
