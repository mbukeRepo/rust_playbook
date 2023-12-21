#[derive(Debug)]
enum ServicePriority{
	High,
	Standard
}

trait Priority {
	fn get_priority(&self) -> ServicePriority;
}

#[derive(Debug)]
struct ImportantGuest;

impl Priority for ImportantGuest {
	fn get_priority(&self) -> ServicePriority {
		ServicePriority::High
	}
}

#[derive(Debug)]
struct Guest;

impl Priority for Guest {
	fn get_priority(&self) -> ServicePriority {
		ServicePriority::Standard
	}
}

fn print_guest_priority<T>(guest: T)
where
	T: Priority + std::fmt::Debug
{
	println!("{:?} is {:?} priority", guest, guest.get_priority());
}

fn main() {
	let guest = Guest;
	let vip = ImportantGuest;
	print_guest_priority(guest);
	print_guest_priority(vip);
}
