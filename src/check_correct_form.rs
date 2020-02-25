use crate::formula::Formula;

pub fn check_correct_form(form: &Formula) -> bool {
	let left = form.get_left_side();
	let right = form.get_right_side();

	if left.len() == 1 && left[0].get_power() == right[0].get_power() {
		if left[0].get_coefficient() == right[0].get_coefficient() {
			println!("All the real numbers are solution");
		} else {
			println!("There are no solutions");
		}

		return false;
	}
	if left[0].get_power() > 2 {
		println!("The polynomial degree is strictly greater than 2, I can't solve it");
		return false;
	} else if left.last().unwrap().get_power() < 0 {
		println!("The polynomial degree is strictly less than 0, I can't solve it");
	}

	true
}