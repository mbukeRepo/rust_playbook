use std::thread;
use crossbeam_channel::unbounded;

enum WorkerThreadMsg {
	PrintData(String),
	Sum(i64, i64),
	Quit
}

enum MainThreadMsg {
	SumResult(i64),
	WorkerQuit
}

fn main() {
	let (worker_tx, worker_rx) = unbounded();
	let (main_tx, main_rx) = unbounded();

	let worker = thread::spawn(move || loop {
		match worker_rx.recv() {
			Ok(msg) => match msg{
				WorkerThreadMsg::PrintData(d) => println!("Worker: {:?}", d),
				WorkerThreadMsg::Sum(a, b) => {
					main_tx.send(MainThreadMsg::SumResult(a + b));
				},
				WorkerThreadMsg::Quit => {
					println!("Worker thread terminating");
					main_tx.send(MainThreadMsg::WorkerQuit);
					break;
				}
			},
			Err(e) => {
				println!("Disconnected!");
				main_tx.try_send(MainThreadMsg::WorkerQuit);
				break;
			}
		}
	});

	worker_tx.send(WorkerThreadMsg::PrintData("hello from main".to_owned()));
	worker_tx.send(WorkerThreadMsg::Sum(10, 10));
	worker_tx.send(WorkerThreadMsg::Quit);
	
	while let Ok(msg)  = main_rx.recv() {
		match msg {
			MainThreadMsg::SumResult(res) => println!("Main: result = {:?}", res),
			MainThreadMsg::WorkerQuit => println!("Worker thread terminated")
		}
	}

	worker.join();
}
