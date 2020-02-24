mod formula;

fn main() {
	let _f;

	match formula::Formula::new("-5x   +   -1x   -   +1x =5") {
		Ok(t) => _f = t,
		Err(_e) => println!("Syntax error")
	}
}