#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit; // no attributes

// A tuple struct
struct Pair(i32, f32);

// A struct with 2 fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    // Calculates area of a given rectangle
    fn rect_area(rec: Rectangle) -> f32 {
        let width: f32 = rec.bottom_right.x - rec.top_left.x;
        let height: f32 = rec.top_left.y - rec.bottom_right.y;
        println!("the width is {} and the height is {}", width, height);
        width * height
    }

    // Create a square given a point and width
    fn square(top_left: Point, width: f32) -> Rectangle {
        Rectangle {
           top_left: Point { x: top_left.x, 
                             y: top_left.y },
           bottom_right: Point { x: top_left.x + width, 
                                 y: top_left.y - width },
        }
    }
}

fn main() {
    // Create a struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person {name, age};

    // debug the struct
    println!("{:?}", peter);

    // Instantiate the point
    let point: Point = Point { x: 10.3, y: 4.4 };

    // Access the fields of point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point using struct update syntax
    let bottom_right = Point { x: 15.2, ..point };

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;
    println!("left_edge is {} and top_edge is {}", left_edge, top_edge);
    let rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: Point { x: 15.4, y: 1.2 },
    };

    println!("Rectangle's top_left is {:?} and bottom_right is {:?}", rectangle.top_left, rectangle.bottom_right);
    println!("The area of the rectangle is {}", Rectangle::rect_area(rectangle) );

    let square: Rectangle = Rectangle::square(point, 30.0);
    println!("The square width is {} and the area is {}", (square.bottom_right.x - square.top_left.x), 
                Rectangle::rect_area(square) );

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);


}
