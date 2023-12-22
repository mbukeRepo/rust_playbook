// Dynamic dispatch => memory calculations at runtime
// Static dispatch => memory calculattion at build / compile time

// Dynamic dispatch => very slow compared to static dispatch
// First iteration
trait Clicky {
	fn click(&self);
}

struct Keyboard;

impl Clicky for Keyboard {
	fn click(&self) {
		println!("click clack");
	}
}

fn borrow_clicky(obj: &dyn Clicky) {
	obj.click();
}

// second iteration

trait Sale {
	fn amount(&self) -> f64;
}

struct FullSale(f64);
impl Sale for FullSale {
	fn amount(&self) -> f64 {
		self.0
	}
}

struct OneDollarOffCoupon(f64);
impl Sale for OneDollarOffCoupon {
	fn amount(&self) -> f64 {
		self.0 - 1.0
	}
}

struct TenPercentOffPromo(f64);
impl Sale for TenPercentOffPromo {
	fn amount(&self) -> f64 {
		self.0 * 0.9
	}
}


fn calculate_revenue(sales: &Vec<Box<dyn Sale>>) -> f64 {
	sales.iter().map(|item| item.amount()).sum()
}
fn main () {
	let price = 20.0;
	let regular = Box::new(FullSale(price));
	let coupon = Box::new(OneDollarOffCoupon(price));
	let disc = Box::new(TenPercentOffPromo(price));

	let arr: Vec<Box<dyn Sale>> = vec![regular, coupon, disc];
	let revenue = calculate_revenue(&arr);

	println!("Revenue = {:?}", revenue);
}
