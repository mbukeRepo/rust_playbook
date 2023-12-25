use std::thread;
use std::time::Duration;

fn main() {
	let value: thread::JoinHandle<usize> = thread::spawn(move || {
		thread::sleep(Duration::from_secs(3));
		42
	});

	println!("Waiting on thread");
	
	match value.join() {
		Ok(n) => println!("value: {}", n),
		Err(e) => println!("error joining thread: {:?}", e)
	}
}
