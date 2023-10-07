enum Flavor {
    Sparkling,
    Sweet,
    Fruity
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64
}

fn print_drink (drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("Sparkling"),
        Flavor::Sweet => println!("flavor: sweet"),
        Flavor::Fruity => println!("flavor: fruity")
    }
    println!("oz: {:?}", drink.fluid_oz)
}

fn main() {
  let sweet = Drink {
     flavor: Flavor::Sweet,
     fluid_oz: 8.8   
  };   
  print_drink(sweet);
  let coord = (2, 3);
  println!("x: {:?}, y: {:?}", coord.0, coord.1);
  let (x, y) = coord;
  println!("x: {:?}, y: {:?}", x,y);
}