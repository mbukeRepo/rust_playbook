use std::convert::TryFrom;

enum NonZeroError {
	IsZero
}

struct NonZero(i32);

impl TryFrom<i32> for NonZero {
	type Error = NonZeroError;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		if value == 0 {
			Err(NonZeroError::IsZero)
		} else {
			Ok(NonZero(value))
		}
	}
}


fn main() {
	match NonZero::try_from(9) {
		Ok(nonzero) => println!("not zero"),
		Err(e) => println!("is zero")
	}
}
