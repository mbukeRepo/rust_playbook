struct Survey {
	q1: Option<String>,
	q2: Option<String>,
	q3: Option<String>
}

fn main() {
	let response = Survey {
		q1: Some("What is your name?".to_owned()),
		q2: Some("How old are you?".to_owned()),
		q3: None
	};

	match response.q1 {
		Some(ans) => println!("{:?}", ans),
		None => println!("Question 1 is none")
	};

	match response.q3 {
		Some(ans) => println!("{:?}", ans),
		None => println!("Question 3 is none")
	}
}
