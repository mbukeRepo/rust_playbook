// first example
struct Uppercase(String);

impl From<String> for Uppercase {
	fn from(data: String) -> Self {
		Uppercase(data.to_uppercase())
	}
}

impl From<&str> for Uppercase {
	fn from(data: &str) -> Self {
		Uppercase(data.to_uppercase())
	}
}

// second example 

enum KeyPress {
	Down,
	Up,
}

struct KeyEvent {
	keycode: u16,
	state: KeyPress
}

struct InputEvent {
	Key(u16, KeyPress),
	Mouse
}

impl From<KeyEvent> for InputEvent {
	fn from (ev: KeyEvent) -> Self {
		InputEvent::Key(ev.keycode, ev.state)
	}
}

fn main() {
	let upper = Uppercase::from("lowercase");
	let upper: Uppercase = "lowercase".into();


	let key_ev = KeyEvent {
		keycode: 5, 
		state: KeyPress::Down
	};

	let input_ev = InputEvent::from(key_ev);
	let input_ev: InputEvent = key_ev.into();
}
