#[derive(Debug)]
struct Adult {
	name: String,
	age: u8
}

impl Adult {
	fn new(age: u8, name: &str) -> Result<Self, &str> {
		if age >= 21 {
			Ok(Self {
				age,
				name: name.to_string()
			})
		} else {
			Err("Age must be atleast 21")
		}
	}
}

fn main() {
	let child = Adult::new(15, "Ange");
	let adult = Adult::new(24, "Prince");

	match child {
		Ok(child) => println!("{} is {} years old", child.name, child.age),
		Err(e) => println!("{e}")
	};

	match adult {
		Ok(a) => println!("{} is {} years old", a.name, a.age),
		Err(e) => println!("{e}")
	}
}
