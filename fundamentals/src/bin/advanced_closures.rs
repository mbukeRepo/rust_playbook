fn math (a: i32, b: i32, opt: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
	opt(a, b)
}


fn main() {
	let name = "outside name";
	// add move keyword to take ownership
	let add = Box::new(move |a, b| {
		println!("{}", name);
		a + b
	});

	let sub = |a, b| a - b;

	println!("{}", math(2, 2, add));
	println!("{}", math(3, 3, Box::new(sub)));
}
