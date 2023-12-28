use std::rc::Rc;

#[derive(Debug)]
struct Vehicle {
	vin: String
}


#[derive(Debug)]
struct Door {
	vehicle: Rc<Vehicle>
}

fn main() {
	let car = Rc::new(Vehicle {
		vin: "123".to_owned()
	});

	let left_door = Door {
		vehicle: Rc::clone(&car)
	};

	let right_door = Door {
		vehicle: Rc::clone(&car)
	};

	drop(car);
}
