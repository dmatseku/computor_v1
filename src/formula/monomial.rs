use std::iter::Peekable;
use std::fmt::{Display, Formatter};

fn atoi(it: &mut Peekable<std::str::Chars>) -> i32 {
	let mut res = 0;
	let mut c: Option<&char> = it.peek();

	while c.is_some() {
		match c {
			Some(t) =>  {
				if *t < '0' || *t > '9' {
					break;
				} else {
					res = res * 10 + (*t as i32 - 48)
				}
			},
			None => break
		}
		it.next();
		c = it.peek();
	}

	res
}

fn skip(it: &mut Peekable<std::str::Chars>) ->i32 {
	let mut c = it.peek();
	let mut res: i32 = 1;

	while c.is_some() {
		match c {
			Some(t) => {
				if (*t >= '0' && *t <= '9') || *t == 'x' {
					break;
				} else if *t == '-' {
					res *= -1;
				}
			},
			None => ()
		}
		it.next();
		c = it.peek();
	}

	res
}

#[derive(Copy, Clone, Eq)]
pub struct Monomial {
	coefficient: i32,
	power: i32
}

impl Monomial {
	pub fn new(mon_string: &str) -> Monomial {
		let mut result: Monomial = Monomial { coefficient: 0, power: 0 };
		let mut it = mon_string.chars().peekable();
		let mut modifier: i32;

		modifier = skip(&mut it);
		result.coefficient = match it.peek() {
			Some(t) => {
				if *t == 'x' {
					modifier
				} else {
					modifier * atoi(&mut it)
				}
			},
			None => 0
		};
		result.power = match it.peek() {
			Some(t) => {
				if *t != 'x' {
					skip(&mut it);
				}
				it.next();
				modifier = skip(&mut it);
				if it.peek().is_some() {
					modifier * atoi(&mut it)
				} else {
					modifier
				}
			},
			None => 0
		};

		result
	}

	pub fn get_coefficient(&self) -> i32 {
		self.coefficient
	}

	pub fn get_power(&self) -> i32 {
		self.power
	}

	pub fn change_sign(&mut self) {
		self.coefficient *= -1;
	}

	pub fn add(&mut self, monomial: &Monomial) -> Result<(), ()> {
		if self.power == monomial.power {
			self.coefficient += monomial.coefficient;
			if self.coefficient == 0 {
				self.power = 0;
			}
		} else {
			return Err(());
		}

		Ok(())
	}
}

impl PartialEq for Monomial {
	fn eq(&self, other: &Monomial) -> bool {
		other.power == self.power
	}
}

impl Display for Monomial {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		let mut coefficient = self.coefficient;

		if coefficient < 0 {
			coefficient *= -1;
		}
		if coefficient == 0 || self.power == 0 {
			write!(f, "{}", coefficient)
		} else if self.power == 1 {
			if coefficient == 1 {
				write!(f, "x")
			} else {
				write!(f, "{}x", coefficient)
			}
		} else {
			if coefficient == 1 {
				write!(f, "x^{}", self.power)
			} else {
				write!(f, "{}x^{}", coefficient, self.power)
			}
		}
	}
}