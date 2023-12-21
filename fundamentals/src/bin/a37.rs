trait Perimeter {
	fn calculate_perimeter(&self) -> i32;
}

struct Square {
	side: i32
}

struct Triangle {
	side_a: i32,
	side_b: i32,
	side_c: i32,
}

impl Perimeter for Square {
	fn calculate_perimeter(&self) -> i32 {
		self.side * 4
	}
}

impl Perimeter for Triangle {
	fn calculate_perimeter(&self) -> i32 {
		self.side_a + self.side_b + self.side_c
	}
}

fn print_perimeter(shape: impl Perimeter) {
	let perimeter = shape.calculate_perimeter();
	println!("Perimeter = {:?}", perimeter);
}

fn main() {
	print_perimeter(Square {side: 5});
	print_perimeter(Triangle {side_a: 3, side_b: 4, side_c: 5});
}
