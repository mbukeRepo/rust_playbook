// match guards

enum Status {
	Error(i32),
	Info,
	Warn
}

enum Species {
	Finch,
	Hawk, 
	Parrot
}

struct Bird {
	species: Species,
	age: u32
}

#[derive(PartialEq)]
enum Difficulty {
	Normal,
	Hard,
	Easy
}

struct Vehicle {
	km: usize, 
	year: usize
}

fn main() {
	let status = Status::Error(4);
	
	match status {
		Status::Error(s @ 3) => println!("error three"),
		Status::Error(s @ 5..=6) => println!("error btwn 5 and 6"),
		Status::Error(s @ 18 | s @ 19) => println!("error 18 or 19"),
		Status::Error(s) => println!("error code {}", s),
		Status::Info => println!("info"),
		Status::Warn => println!("warn")
	}

	let hawk = Bird {
		age: 13,
		species: Species::Hawk
	};

	match hawk {
		Bird { age: 4, .. } => println!("4 year old bild"),
		Bird { age: 5..=10 | 15..=20, .. } => println!("5-10 or 15-20 year old bird"),
		Bird { .. } => println!("other bird")
	}

	let stage = 5;
	let diff = Difficulty::Normal;
	match stage {
		s if (s == 5 && diff == Difficulty::Easy) => println!("easy mode stage"),
		s if diff == Difficulty::Normal => println!("normal difficulty stage"),
		s @ 10 | s @ 15 => println!("stage 10 or 15"),
		s => println!("stage {}", stage)
	}

	let car = Vehicle {
		km: 80_000,
		year: 2020
	};

	match car {
		Vehicle {km, year} if year == 2020 && km == 0 => println!("new 2020 wagon"),
		Vehicle {km, ..} if km <= 50_000 => println!("under 50k km"),
		Vehicle {year, ..} if year == 2020 => println!("made in 2020"),
		Vehicle { .. } => println!("other milage")	
	}
}
