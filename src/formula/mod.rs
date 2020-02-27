mod monomial;
mod errors;

use regex::Regex;
use std::cmp::Ordering;
use self::monomial::Monomial;

pub struct Formula {
	left_side: Vec<monomial::Monomial>,
	right_side: Vec<monomial::Monomial>,
}

impl Formula {

	pub fn get_left_side(&self) -> &Vec<monomial::Monomial> {
		&self.left_side
	}

	pub fn get_right_side(&self) -> &Vec<monomial::Monomial> {
		&self.right_side
	}

	fn create_vec(formula_sub_string: &str, side: &mut Vec<monomial::Monomial>) { //create one side
		let re = Regex::new(
			r"((\+\s*|-\s*)?[-,+]?(((\d{1,9}(.\d{1,6})?)?(\s*\*\s*)?X(\^-?\d{1,9})?)|(\d{1,9}(.\d{1,6})?)))"
		).unwrap();
		let caps = re.find_iter(formula_sub_string);

		for str in caps { //add all operands to the vector
			let new_mon = Monomial::new(str.as_str());

			//if the vector has a monomial with the same power,
			// then a new monomial is added to the existing one.
			// Otherwise it is added to the vector
			match side.iter_mut().find(|x| x.get_power() == new_mon.get_power()) {
				Some(t) => {
					t.add(&new_mon).unwrap();
				},
				None => {
					side.push(new_mon);
				},
			}
		}

		//clear null's elements
		let mut i = 0;
		while i < side.len() {
			if side[i].get_coefficient() == 0.0 && side.len() > 1 {
				side.remove(i);
			} else {
				i += 1;
			}
		}
	}

	pub fn new(formula_string: &str) -> Result<Formula, errors::SyntaxError> {
		let mut res: Formula = Formula { left_side: Vec::new(), right_side: Vec::new() };

		//super-duper regex for check input :)
		let re = Regex::new(format!(
			"{}{}{}{}",
			r"^\s*([-,+]?(((\d{1,9}(.\d{1,6})?)?(\s*\*\s*)?X(\^-?\d{1,9})?)|(\d{1,9}(.\d{1,6})?)))\s*",
			r"(((\+\s*|-\s*)[-,+]?(((\d{1,9}(.\d{1,6})?)?(\s*\*\s*)?X(\^-?\d{1,9})?)|(\d{1,9}(.\d{1,6})?)))\s*)*=",
			r"\s*([-,+]?(((\d{1,9}(.\d{1,6})?)?(\s*\*\s*)?X(\^-?\d{1,9})?)|(\d{1,9}(.\d{1,6})?)))\s*",
			r"(((\+\s*|-\s*)[-,+]?(((\d{1,9}(.\d{1,6})?)?(\s*\*\s*)?X(\^-?\d{1,9})?)|(\d{1,9}(.\d{1,6})?)))\s*)*$"
		).as_str()).unwrap();

		if re.is_match(formula_string) == false {
			return Err(errors::SyntaxError);
		}

		let substrings: Vec<&str> = formula_string.split('=').collect();

		Formula::create_vec(substrings[0], &mut res.left_side);
		Formula::create_vec(substrings[1], &mut res.right_side);

		print!("--- ");
		res.print_formula();
		println!(" ---");
		res.reduce();
		print!("Reduced form: ");
		res.print_formula();
		println!();

		Ok(res)
	}

	fn reduce(&mut self) {
		while !self.right_side.is_empty() {
			let mut mon = self.right_side.pop().unwrap();

			//move right side to the left and clear nulls if they exist
			mon.change_sign();
			match self.left_side.iter_mut().position(|x| x.get_power() == mon.get_power()) {
				Some(t) => {
					self.left_side[t].add(&mon).unwrap();
					if self.left_side[t].get_coefficient() == 0.0 && self.left_side.len() > 1 {
						self.left_side.remove(t);
					}
				},
				None => {
					self.left_side.push(mon);
				},
			}
		}
		self.right_side.push(Monomial::new("0"));

		self.left_side.sort_unstable_by(|a, b| {
			if a.get_power() < b.get_power() {
				return Ordering::Less;
			} else if a.get_power() > b.get_power() {
				return Ordering::Greater;
			}
			Ordering::Equal
		});
	}
}

fn print_sign (mon: &Monomial, first: &mut bool) {
	if *first {
		if mon.get_coefficient() < 0.0 {
			print!("-");
		}
		*first = false;
	} else {
		print!("{}", if mon.get_coefficient() < 0.0 { "- " } else { "+ " });
	}
}

impl Formula {
	pub fn print_formula(&self) {
		let mut it = self.left_side.iter();
		let mut first = true;

		while match it.next() {
			Some(t) => {
				print_sign(t, &mut first);
				print!("{} ", t);
				true
			},
			None => false
		} {}
		print!("=");
		first = true;
		it = self.right_side.iter();
		while match it.next() {
			Some(t) => {
				print_sign(t, &mut first);
				print!(" {}", t);
				true
			},
			None => false
		} {}
	}
}
