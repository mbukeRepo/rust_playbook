use activity::math;
use activity::msg::{trim};

fn main() {
	let res = {
		let two_plus_two = math::add(2, 2);
		let three = math::add(two_plus_two, -1);
		three
	};

	println!("{:?}", res);

	let name = " Mbuke  ";

	println!("{:?}", trim(name))
}
