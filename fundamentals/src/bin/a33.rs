#[cfg(test)]
mod testing{
	use std::time::Duration;
	use humantime::format_duration;

	#[test]
	fn test_timing() {
		let val1 = Duration::new(9420, 0);
		assert_eq!(format_duration(val1).to_string(), "2h 30m", "Should equal to 2 hours and 37 minutes");
		let val2 = Duration::new(0, 32_000_000);
		assert_eq!(format_duration(val2).to_string(), "32ms")
	}
}
