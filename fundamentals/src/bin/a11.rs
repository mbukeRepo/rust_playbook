#[derive(Debug, Clone, Copy)]
enum Position {
  	Manager,
	Developer
}

#[derive(Debug, Clone, Copy)]
struct Employee {
  	position: Position,
	salary: i32
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp)
}

fn main() {
	let emp = Employee{position: Position::Manager, salary: 34};
	print_employee(emp);
	print_employee(emp);
}
