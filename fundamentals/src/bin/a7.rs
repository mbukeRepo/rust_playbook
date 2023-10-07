struct Test {
 	 score : i32
}

fn main() {
  	let my_scores = vec![
		Test {score: 3},
		Test {score: 9}
	];

 	for score in my_scores {
		println!("Score {:?}", score.score);
	}
}
