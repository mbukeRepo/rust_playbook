use std::thread;
use crossbeam_channel::unbounded;

enum ThreadMsg {
	PrintData(String),
	Sum(i64, i64),
	Quit
}

fn main() {
	let (s, r) = unbounded();
	let handler = thread::spawn(move || loop {
		match r.recv() {
			Ok(msg) => match msg{
				ThreadMsg::PrintData(d) => println!("Data: {:?}", d),
				ThreadMsg::Sum(a, b) => println!("Sum [a={:?}, b={:?}] {:?}", a, b, a + b),
				ThreadMsg::Quit => {
					println!("thread terminating");
					break;
				}
			},
			Err(e) => {
				println!("Disconnected!");
				break;
			}
		}
	});

	s.send(ThreadMsg::PrintData("hello from main".to_owned()));
	s.send(ThreadMsg::Sum(10, 10));
	s.send(ThreadMsg::Quit);

	handler.join();
}
