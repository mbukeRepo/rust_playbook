fn div (a: i32, b: i32) -> Option<i32> {
	Some(a / b)
}

#[cfg(test)]
mod testing {
	use crate::*;

	#[test]
	fn test_div() {
		let result = div(100, 10);
		assert_eq!(result, Some(10), "should be 10")
	}
}
