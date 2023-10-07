struct Temperature {
 	degrees_f: f64,
}

impl Temperature {
        fn freezing(degrees_f: f64) -> Self {
		Self {degrees_f: degrees_f}
	}

	fn show_temp (&self) {
	  	println!("{:?} degrees F", self.degrees_f);
	}
}
 
fn main() {
  	let hot = Temperature {degrees_f: 99.9};
  	hot.show_temp();

	let temp = Temperature::freezing(23.2);
        temp.show_temp();
}
