	height: f64,
	depth: f64
}

trait Convey {
	fn width(&self) -> f64;
	fn dimensions(&self) -> Dimensions;
}

struct ConveyorBelt<T: Convey> {
	pub items: Vec<T>
}

impl <T>ConveyorBelt<T> 
where 
	T: Convey
{
	pub fn add(&mut self, item: T) {
		self.items.push(item);
	}
}


struct CarPart {
	width: f64,
	height: f64,
	weight: f64,
	depth: f64,
	part_number: String
}

impl Default for CarPart {
	fn default() -> Self {
		Self {
			width: 5.0,
			height: 1.0,
			depth: 2.0,
			weight: 3.0,
			part_number: "abc".to_owned()
		}
	}
}

impl Convey for CarPart {
	fn width(&self) -> f64 {
		self.width
	}

	fn dimensions(&self) -> Dimensions {
		Dimensions {
			width: self.width,
			height: self.height,
			depth: self.depth
		}	
	}
}


fn main() {
	let mut belt: ConveyorBelt<CarPart> = ConveyorBelt{items: Vec::new()};
	belt.add(CarPart::default());
}
