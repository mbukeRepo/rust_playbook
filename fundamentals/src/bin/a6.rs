enum Color {
	Brown,
	Red
}

impl Color {
	fn print(&self) {
		match self {
			Color::Brown => println!("brown"),
			Color::Red => println!("red")
		}
	}
}

struct Dimensions {
	width: f64,
	height: f64,
	depth: f64
}

impl Dimensions {
	fn print(&self) {
		println!("width: {:?}", self.width);
		println!("height: {:?}", self.height);
		println!("depth: {:?}", self.depth);
	}
}



struct ShippingBox {
	color: Color,
	weight: f64,
	dimensions: Dimensions
}

impl ShippingBox {
   	fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
		Self {
			weight,
			color,
			dimensions
		}
	}

	fn print(&self) {
		self.color.print();
		self.dimensions.print();
		println!("Weight: {:?}", self.weight)
	}
}


fn main() {
   	let small_dim = Dimensions {
		width: 1.0,
		height: 2.0,
		depth: 3.0
	};

	let small_box = ShippingBox::new(23.0, Color::Brown, small_dim);
	small_box.print(); 
}
