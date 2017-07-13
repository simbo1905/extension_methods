extern crate some_library;
extern crate some_extension;
extern crate serde_json;

use some_library::Shape;
use some_extension::Shaper;
use some_extension::JSON;

fn main() {
    let circle = Shaper(Shape::Circle{ x: 11, y: 22, radius: 33 });
    println!("{}", circle.to_json());
    let rect = Shaper(Shape::Rect{ x: 44, y: 55, width: 66, height: 77} );
    println!("{}", rect.to_json());
}
