struct LineItem {
	name: String,
	count: i32,
}

fn print_name(name: &str) {
	println!("Name: {:?}", name);
}

fn main() {
	let receipt = vec![
		LineItem {
			name: "Cereal".to_owned(),
			count: 1
		},
		LineItem {
			name: "Chips".to_owned(),
			count: 1
		}
	];

	for item in receipt {
		print_name(&item.name);
		println!("Count: {:?}", item.count);
	}
}
