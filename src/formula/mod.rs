mod sign;
mod monomial;
mod errors;

use regex::Regex;
use crate::formula::monomial::Monomial;
use std::cmp::Ordering;

pub struct Formula {
	left_side: Vec<monomial::Monomial>,
	right_side: Vec<monomial::Monomial>,
}

impl Formula {
	fn create_vec(formula_sub_string: &str, side: &mut Vec<monomial::Monomial>) {
		let re = Regex::new(r"((\+\s*|-\s*)?[-,+]?((\d{0,9}x(\^\d{1,9})?)|(\d{1,9})))").unwrap();
		let caps = re.find_iter(formula_sub_string);

		for str in caps {
			let new_mon = Monomial::new(str.as_str());

			match side.iter_mut().find(|x| x.get_power() == new_mon.get_power()) {
				Some(t) => {
					t.add(&new_mon).unwrap();
				},
				None => {
					side.push(new_mon);
				},
			}
		}

		side.sort_unstable_by(|a, b| {
			if a.get_power() > b.get_power() {
				return Ordering::Less;
			}
			else if a.get_power() < b.get_power() {
				return Ordering::Greater;
			}
			Ordering::Equal
		} );
	}

	pub fn new(formula_string: &str) -> Result<Formula, errors::SyntaxError> {
		let mut res: Formula = Formula {left_side: Vec::new(), right_side: Vec::new() };
		let re = Regex::new(format!("{}{}{}{}",
				r"^\s*([-,+]?((\d{0,9}x(\^\d{1,9})?)|(\d{1,9})))\s*",
				r"(((\+\s*|-\s*)?[-,+]?((\d{0,9}x(\^\d{1,9})?)|(\d{1,9})))\s*)*=",
				r"\s*([-,+]?((\d{0,9}x(\^\d{1,9})?)|(\d{1,9})))\s*",
				r"(((\+\s*|-\s*)?[-,+]?((\d{0,9}x(\^\d{1,9})?)|(\d{1,9})))\s*)*$").as_str()).unwrap();

		if re.is_match(formula_string) == false {
			return Err(errors::SyntaxError);
		}

		let substrings: Vec<&str> = formula_string.split('=').collect();

		Formula::create_vec(substrings[0], &mut res.left_side);
		Formula::create_vec(substrings[1], &mut res.right_side);
		res.shorten();

		Ok(res)
	}

	fn shorten(&mut self) {
		while !self.right_side.is_empty() {
			let mon = self.right_side.pop().unwrap();

			match self.left_side.iter_mut().find(|x| x.get_power() == mon.get_power()) {
				Some(t) => {
					t.add(&mon).unwrap();
				},
				None => {
					self.left_side.push(mon);
				},
			}
		}
		self.right_side.push(Monomial::new("0"));
	}
}
