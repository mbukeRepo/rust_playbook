use thiserror::Error;
use chrono::{DateTime, Duration, Utc};

struct SubwayPass {
	id: usize,
	funds: isize,
	expires: DateTime<Utc>
}

#[derive(Debug, Error)]
enum PassError {
	#[error("expire pass")]
	PassExpired,
	#[error("insufficient funds: {0}")]
	InsufficientFunds(isize),
	#[error("pass read error: {0}")]
	ReadError(String),
}

fn swipe_card() -> Result<SubwayPass, PassError> {
	Ok(
		SubwayPass {
			id: 0,
			funds: 200,
			expires: Utc::now()
		}
	)
}

fn use_pass(pass: &mut SubwayPass, cost: isize) -> Result<(), PassError> {
	println!("Now = {:?}, Ticket expires = {:?}", Utc::now(), pass.expires);
	if Utc::now() > pass.expires {
		Err(PassError::PassExpired)
	} else {
		if pass.funds - cost < 0 {
			Err(PassError::InsufficientFunds(pass.funds))
		} else {
			pass.funds = pass.funds - cost;
			Ok(())
		}
	}
}

fn main() {
	let pass_status = swipe_card().and_then(|mut pass| use_pass(&mut pass, 600));
	match pass_status {
		Ok(_) => println!("ok to board"),
		Err(e) => println!("error = {:?}", e)
	}
}
