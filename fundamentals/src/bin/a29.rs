#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Invalid choice".to_string())
    }
}

fn print_choice(choice: &MenuChoice) {
    match choice {
        MenuChoice::MainMenu => println!("Main menu"),
        MenuChoice::Start => println!("Start"),
        MenuChoice::Quit => println!("Quit")
    }

}

fn main () {
    let choice = get_choice("leave");
    match choice {
        Ok(d) => print_choice(&d),
        Err(e) => println!("error = {:?}", e)
    }
}