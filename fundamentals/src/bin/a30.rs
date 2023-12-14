struct Person {
	name: String
}

fn main() {
	let name = "Mbuke".to_string();
	let owned = Person{
		name: name
	};

	println!("{:?}", owned.name)
}
