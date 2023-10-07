enum Light {
  Bright,
  Dull
}

struct Book {
  pages: i32,
  rating: i32,
}

fn display_light(light: &Light) {
  match light {
    Light::Bright => println!("Bright"),
    Light::Dull => println!("Dull")
  }
}

fn display_page_count(book: &Book) {
  println!("pages = {:?}", book.pages);
}

fn display_rating(book: &Book) {
  println!("rating = {:?}", book.rating);
}

fn main() {
  let dull = Light::Dull;
  display_light(&dull);
  display_light(&dull);
  let book = Book {
    pages: 5,
    rating: 9
  };

  display_page_count(&book);
  display_rating(&book);
}
