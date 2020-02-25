mod formula;
mod check_correct_form;

fn main() {
	match formula::Formula::new("6=5") {
		Ok(t) => {
			println!("Polynomial degree: {}", t.get_left_side()[0].get_power());
			if !check_correct_form::check_correct_form(&t) {
				return;
			}
		},
		Err(_e) => println!("Syntax error")
	}
}