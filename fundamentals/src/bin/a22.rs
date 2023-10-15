use std::collections::HashMap;

#[derive(Debug)]
struct Contents {
	content: String
}

fn main() {
	let mut lockers = HashMap::new();
	lockers.insert(
		1, 
		Contents{content: "Hello world".to_owned()}
	);

	lockers.insert(
		2, 
		Contents{content: "Fay kun".to_owned()}
	);


	for (key, value) in lockers.iter() {
		println!("{:?} = {:?}", key, value)
	}
}
