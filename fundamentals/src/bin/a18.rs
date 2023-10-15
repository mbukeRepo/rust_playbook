struct Student {
	name: String,
	locker: Option<i32>
}

fn main() {
	let mary = Student {
		name: "Mary".to_owned(),
		locker: Some(32)
	};

	println!("student {:?}", mary.name);
	match mary.locker {
		Some(num) => println!("locker number: {:?}", num),
		None => println!("no locker assigned")
	}
}
