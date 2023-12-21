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


fn main() {
 	fall(Vase {});
	fall(Cat {});
}
