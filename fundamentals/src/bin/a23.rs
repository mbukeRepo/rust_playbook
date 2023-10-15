use std::collections::HashMap;

fn main() {
	let mut stock = HashMap::new();
	stock.insert("Chair", 3);
	stock.insert("Hello", 0);
	stock.insert("Table", 3);

	// counting stock value
	let mut total_stock = 0;

	for (key, value) in stock.iter() {
		total_stock = total_stock + value;

		let stock_count = if value.to_owned() == 0 {
			"out of stock".to_owned()
		} else {
			format!("{:?}", value)
		};

		println!("item={:?}, stock={:?}", key, stock_count)
	};

	
	println!("Total stock={:?}", total_stock);
}
