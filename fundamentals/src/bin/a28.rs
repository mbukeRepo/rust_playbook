fn main() {
	let numbers = vec![1, 2, 3, 4, 5];

	let plus_one: Vec<_> = numbers
			.iter()
			.map(|num| num + 1)
			.collect();

	let new_numbers: Vec<_> = numbers
				.iter()
				.filter(|num| *num <= &3)
				.collect();
	let number: Option<_> = numbers
			.iter()
			.find(|num| **num == 4);
	let count = numbers
		.iter()
		.count();

	let last = numbers
		.iter()
		.last();
	let max = numbers
		.iter()
		.max();

	let min = numbers
		.iter()
		.min();

	let take: Vec<_> = numbers
		.iter()
		.take(3)
		.collect();

	println!("{:?}", number);
	println!("{:?}", count);
	println!("{:?}", last);
	println!("{:?} {:?}", min, max);
	println!("{:?}", take)
}
