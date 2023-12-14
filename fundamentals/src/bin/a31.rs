fn main() {
    let range = 1..10;

    for i in range {
        println!("i = {:?}", i);
    }
    let array = vec![1, 2, 3, 4, 5];

    let mut array_iter = array.iter();

    while let Some(i) = array_iter.next() {
        println!("i = {:?}", i);
    }
}