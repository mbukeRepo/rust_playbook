use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use backoff::ExponentialBackoff;
use std::time::Duration;
use std::thread::{JoinHandle};

#[derive(Copy, Clone)]
struct Account {
	balance: f64
}

type ArcAccount = Arc<Mutex<Account>>;

// this version is likely to have deadlocks
fn transfer_v1(from: ArcAccount, to: ArcAccount, amount: f64) {
	let mut from = from.lock();
	let mut to = to.lock();

	from.balance -= amount;
	to.balance -= amount;
}

// sub-optimal way of handling deadlocks 
fn transfer_v2(from: ArcAccount, to: ArcAccount, amount: f64) {
	loop {
		if let Some(mut from)  = from.try_lock() {
			if let Some(mut to) = to.try_lock() {
				from.balance -= amount;
				to.balance -= amount;
				return;
			}
		}

		thread::sleep(Duration::from_millis(3));
	}
}

// handling deadlocks optimally
fn transfer(from: ArcAccount, to: ArcAccount, amount: f64) -> JoinHandle<()> {
	thread::spawn(move || {
		let op = || {
			if let Some(mut from) = from.try_lock() {
				if let Some(mut to) = to.try_lock() {
					from.balance -= amount;
					to.balance -= amount;
					println!("New balance ----");
					println!("Giving ACCOUNT: {:?}", from.balance);
					println!("Receiving ACCOUNT: {:?}", to.balance);
					return Ok(());
				}
			}

			Err(0)?
		};

		let backoff = ExponentialBackoff::default();
		backoff::retry(backoff, op);
	})
}

fn main () {
	let a = Arc::new(Mutex::new(Account {
		balance: 500.0
	}));

	let b = Arc::new(Mutex::new(Account {
		balance: 4.5
	}));


	let t1 = transfer(Arc::clone(&a), Arc::clone(&b), 500.0);

	let t2 = transfer(Arc::clone(&a), Arc::clone(&b), 800.9);

	t2.join();
	t1.join();
}
