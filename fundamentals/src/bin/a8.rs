fn main() {
	let numbers = vec![10, 20, 30, 40];

	for number in &numbers {
		match number {
			30 => println!("Number: Thirty"),
			_ => println!("Number: {:?}", number)  
		}
	}

	println!("Vector length {:?}", numbers.len());
}
