fn to_owned(slice: &str) -> String {
	slice.into()
}

enum Status {
	Broken(u8),
	Working,
}

impl From<u8> for Status {
	fn from(code: u8) -> Self {
		match code {
			0 => Status::Working,
			c => Status::Broken(code)
		}
	}
}

fn legacy_interface() -> u8 {
	5
}

struct Job;

enum JobError {
	Expired,
	Missing,
	Other(u8)
}

impl From<u8> for JobError {
	fn from(code: u8) -> Self {
		match code {
			1 => Self::Expired,
			2 => Self::Missing,
			c => Self::Other(c)
		}
	}
}

fn execute_job(job: Job) -> Result<(), JobError> {
	Err(2)?
}

fn main () {
	let owned = String::from("slice");
	let owned: String = "slice".into();
	
	let status: Status = legacy_interface.into();
	let status1 = Status::from(legacy_interface());
}
