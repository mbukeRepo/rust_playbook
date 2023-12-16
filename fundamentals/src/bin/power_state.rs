use std::io;

enum PowerState {
	Off, 
	Sleep,
	Reboot, 
	Shutdown,
	Hibernate,
}

impl PowerState {
	fn new(state: &str) -> Option<PowerState> {
		let state = state.trim().to_lowercase();
		match state.as_ref() {
			"off" => Some(PowerState::Off),
			"sleep" => Some(PowerState::Sleep),
			"reboot" => Some(PowerState::Reboot),
			"shutdown" => Some(PowerState::Shutdown),
			"hibernate" => Some(PowerState::Hibernate),
			_ => None
		}
	}
}

fn print_power_action(state: PowerState) {
	use PowerState::*;
	match state {
		Off => println!("turning off"),
		Sleep => println!("Sleeping"),
		Reboot => println!("Rebooting"),
		Shutdown => println!("Shutting down"),
		Hibernate => println!("Hibernating"),
	}
}


fn main() {
	let mut buffer = String::new();
	let user_input = io::stdin().read_line(&mut buffer);

	if user_input.is_ok() {
		match PowerState::new(&mut buffer) {
			Some(state) => print_power_action(state),
			None => println!("Invalid power state")
		}
	} else {
		println!("error reading input");
	}
}
