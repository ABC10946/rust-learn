#[derive(Debug)]
struct Person {
	name: String,
	age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
	x: f32,
	y: f32,
}

#[allow(dead_code)]
struct Rectangle {
	top_left: Point,
	bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
	let width = (rectangle.top_left.x - rectangle.bottom_right.x).abs();
	let height = (rectangle.top_left.y - rectangle.bottom_right.y).abs();
	width * height
}

fn main() {
	let name = String::from("Peter");
	let age = 27;
	let peter = Person {name, age};

	println!("{:?}", peter);

	let point: Point = Point {x:10.0, y:10.0};
	println!("point coordinates: ({}, {})", point.x, point.y);

	let bottom_right = Point {x:0.0, y: 0.0};
	println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

	let Point {x: left_edge, y:top_edge} = point;

	let _rectangle = Rectangle {
		top_left: Point {x:left_edge, y : top_edge},
		bottom_right: bottom_right,
	};

	println!("rectangle area: {}", rect_area(_rectangle));

	let _unit = Unit;
	let pair = Pair(1, 0.1);

	println!("pair contains {:?} and {:?}", pair.0, pair.1);

	let Pair(integer, decimal) = pair;

	println!("pair contains {:?} and {:?}", integer, decimal);

}