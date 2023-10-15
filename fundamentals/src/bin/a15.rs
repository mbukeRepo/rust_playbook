enum Ticket {
	Backstage(f64, String),
	Standard(f64),
	Vip(f64, String)
}

fn main() {
	let tickets = vec! [
		Ticket::Backstage(50.2, "Billy".to_owned()),
		Ticket::Standard(56.4),
		Ticket::Vip(34.3, "Any".to_owned())
	];
 
	for ticket in tickets {
		match ticket {
			Ticket::Backstage(price, holder) => {
				println!("Backstage ticket holder {:?} price {:?}", holder, price);
			},
			Ticket::Standard(price) => {
				println!("Standard ticket holder, price {:?}", price);
			},
			Ticket::Vip(price, holder) => {
				println!("VIP ticket holder {:?} {:?}", holder, price)
			}
		}
	}
}
