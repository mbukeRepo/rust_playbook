enum Discount {
	Percent(i32),
	Flat(i32)
}

struct Ticket {
	event: String,
	price: i32
}

fn main() {
	let n = 3;
	match n {
		3 => println!("Three"),
		other => println!("number: {:?}", other)
	};

	let flat = Discount::Flat(3);

	match flat {
		Discount::Flat(amount) => println!("flat discount of {:?}", amount),
		Discount::Percent(number) => println!("Percentage {:?}", number)
	}


	let events = vec![
		Ticket {event: String::from("house_party"), price: 30},
		Ticket {event: String::from("pool_party"), price: 40},
		Ticket {event: String::from("opening_celemony"), price: 50}
	];

	for event in events {
		match event {
			Ticket {price: 50, event} => println!("Fancy party"),
			_ => println!("It's a party")
		}
	}
}
