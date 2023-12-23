// compile time enforcement of logic
// Leverage type system to encode state changes
// => Use move semantics to invalidate a state
// => Return next state from previous state
// => Optionally drop the state
//    - Close file, connection dropped, etc

// Used for: 
// - Invalidating / consuming states
// - Properly transitioning to another state
// - Disallowing access to a missing resource


struct Employee<State> {
	name: String,
	state: State
}

impl <State> Employee <State> {
	fn transition<NextState>(self, state: NextState) -> Employee<NextState> {
		Employee {
			name: self.name,
			state: state
		}
	}
}

struct Agreement;
impl Employee<Agreement> {
	fn new(name: &str) -> Self {
		Self {
			name: name.to_owned(),
			state: Agreement
		}
	}

	fn read_agreement(self) -> Employee<Signature> {
		self.transition(Signature)
	}
}

struct Signature;
impl Employee<Signature> {
	fn sign(self) -> Employee<Training> {
		self.transition(Training)
	}
}

struct Training;
impl Employee<Training> {
	fn train(self, score: u8) -> Result<Employee<OnboardingComplete>, Employee<FailedTraining>> {
		if score >= 7 {
			Ok(self.transition(OnboardingComplete { score }))
		} else {
			Err(self.transition(FailedTraining { score }))
		}
	}
}

struct FailedTraining {
	score: u8,
}
struct OnboardingComplete {
	score: u8,
}


fn main() {
	let employee = Employee::new("Sara");
	let onboarded = employee.read_agreement().sign().train(8);

	match onboarded {
		Ok(complete) => println!("onboarding complete"),
		Err(failed) => println!("training failed, scored less!")
	}
}
