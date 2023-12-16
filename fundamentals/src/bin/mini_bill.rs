use std::io;

fn get_input() -> Option<String> {
	let mut buffer = String::new();
	while io::stdin().read_line(&mut buffer).is_err() {
		println!("Please enter your data again");		
	}

	let input = buffer.trim().to_owned();
	if &input == "" {
		None
	} else {
		Some(input)
	}
}

enum MainMenu {
	AddBill,
	ViewBill
}

impl MainMenu {
	fn from_str(input: &str) -> Option<MainMenu>{
		match input {
			"1" => Some(MainMenu::AddBill),
			"2" => Some(Self::ViewBill),
			_ => None
		}
	}

	fn show() {
		println!("");
		println!("== Bill Manager ==");
		println!("1. Add Bill");
		println!("2. View Bill");
		println!("");
		println!("Enter selection: ");
	}
}

fn main() {
	loop {
		MainMenu::show();
		let input = get_input().expect("no data entered");

		match MainMenu::from_str(input.as_str()) {
			Some(MainMenu::AddBill) => (),
			Some(MainMenu::ViewBill) => (),
			None => return
		}
		
	}
}
