// holistic example

trait Noise {
	fn make_noise(&self);
}

fn hello(noisy: impl Noise) {
	noisy.make_noise();
}

struct Person;

impl Noise for Person {
	fn make_noise(&self) {
		println!("hello")
	}
}

struct Dog;

impl Noise for Dog {
	fn make_noise(&self) {
		println!("woof")
	}
}

// second example

trait Fall {
	fn hit_ground(&self);
}

struct Vase;

impl Fall for Vase {
	fn hit_ground(&self) {
		println!("the vase broke!");
	}
}

struct Cat;

impl Fall for Cat {
	fn hit_ground(&self) {
		println!("the cat casually walked away")
	}
}

fn fall(thing: impl Fall) {
	thing.hit_ground()
}

// third example 

trait Move {
	fn move_to(&self, x: i32, y: i32);
}

// First example
// fn make_move(thing: impl Move, x: i32, y: i32) {
// 	thing.move_to(x, y);
// }

// Second example => Generic function
// fn make_move<T: Move>(thing: T, x: i32, y: i32) {
//	thing.move_to(x, y);
// }

fn make_move<T>(thing: T, x: i32, y: i32) 
where 
	T: Move
{
	thing.move_to(x, y);
}

struct Snake;

impl Move for Snake {
	fn move_to(&self, x: i32, y: i32) {
		println!("Moving [x={:?}, y={:?}]", x, y);
	}
}


fn main() {
 	fall(Vase {});
	fall(Cat {});

	let python = Snake {};
	make_move(python, 1, 1);
}
