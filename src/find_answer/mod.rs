use crate::formula::Formula;

pub fn square_answer(formula: &Formula) {
	let a: f32 =
		match formula.get_left_side().iter().find(|x| x.get_power() == 2) {
			Some(t) => t.get_coefficient(),
			None =>	0.0
		};
	let b: f32 =
		match formula.get_left_side().iter().find(|x| x.get_power() == 1) {
			Some(t) => t.get_coefficient(),
			None =>	0.0
		};
	let c: f32 =
		match formula.get_left_side().iter().find(|x| x.get_power() == 0) {
			Some(t) => t.get_coefficient(),
			None =>	0.0
		};
	let mut d = b.powi(2) - 4.0 * a * c;

	if d < 0.0 {
		println!("Discriminant is negative: {}. There are no solutions", d);
		return;
	} else if d == 0.0 {
		println!("Discriminant is null, the solution is:");
		println!("{}", -b / (a * 2.0));
	} else {
		println!("Discriminant is strictly positive: {}. The two solutions are:", d);
		d = d.sqrt();
		println!("{}\n{}", (-b - d) / (a * 2.0), (-b + d) / (a * 2.0));
	}
}

pub fn simple_answer(formula: &Formula) {
	let mut result: f32;

	if formula.get_left_side()[0].get_power() == 0 {
		result = formula.get_left_side()[0].get_coefficient() * -1.0;
	} else {
		result = 0.0;
	}

	result /= formula.get_left_side().last().unwrap().get_coefficient();
	println!("The solution is: {}", result);
}