struct Person {
	name: String,
	fav_color: String,
	age: i32
}

fn print(data: &str) {
	println!("Data: {:?}", data);
}

fn main() {
	let people = vec![
		Person{
			name: String::from("George"),
			fav_color: String::from("green"),
			age: 7
		},
		Person{
			name: String::from("Anna"),
			fav_color: String::from("purple"),
			age: 9
		}
	];
	for person in people {
		print(&person.name);
		print(&person.fav_color);
	}
}
