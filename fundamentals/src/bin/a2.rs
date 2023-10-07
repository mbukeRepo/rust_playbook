fn sub(a: i32, b:i32) -> i32 {
    a - b
}

enum Direction {
  Left,
  Right       
}

enum Color {
    Red,
    Yellow,
    Blue
}

fn print_color(my_color: Color) {
    match my_color {
        Color::Red => println!("red"),
        Color::Yellow => println!("yellow"),
        Color::Blue => println!("blue")
    }
}

fn main() {
    let five = sub(8, 3);

    println!("{:?}", five);

    // if conditions
    let some_bool = true;
    if some_bool {
        println!("it's true")
    } else {
        println!("it's false")
    }

    match some_bool {
        true => println!("its true"),
        false => println!("its false"),
    }

    let my_name = "Paul";

    match my_name {
        "John" => println!("My name is John"),
        "Peter" => println!("My name is Peter"),
        _ => println!("You are nameless")
    }

    // loop with loop
    let mut i = 0;
    loop {
        if i == 5 {
            break;
        }
        println!("Hello!");
        i = i + 1;
    }

    // loop with while loop
    let mut i = 1;
    while i <= 3 {
        println!("{:?}", i);
        i = i + 1;
    }

    // enumerations

    let go = Direction::Left;
    let go_right = Direction::Right;
    match go {
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right")
    }
    match go_right {
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right")
    }
    print_color(Color::Blue);
}