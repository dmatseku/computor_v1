use super::sign::Sign;
use std::iter::Peekable;

fn atoi(it: &mut Peekable<std::str::Chars>) -> u32 {
	let mut res = 0;
	let mut c: Option<&char> = it.peek();

	while c.is_some() {
		match c {
			Some(t) =>  {
				if *t < '0' || *t > '9' {
					break;
				} else {
					res = res * 10 + (*t as u32 - 48)
				}
			},
			None => break
		}
		it.next();
		c = it.peek();
	}

	res
}

fn skip(it: &mut Peekable<std::str::Chars>) -> Sign {
	let mut c = it.peek();
	let mut res: Sign = Sign::Positive;

	while c.is_some() {
		match c {
			Some(t) => {
				if *t >= '0' || *t <= '9' || *t == 'x' {
					break;
				}
				else if *t == '-' {
					if res == Sign::Positive {
						res = Sign::Negative;
					} else {
						res = Sign::Positive;
					}
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
	coefficient: u32,
	power: u32,
	sign: Sign
}

//r"^((\+ |- )?(-|\+)?((\d{0,9})|(\d{0,9}x(\^(\d{1,9}))?)))$"

impl Monomial {
	pub fn new(mon_string: &str) -> Monomial {
		let mut result: Monomial = Monomial {coefficient: 0, power: 0, sign: Sign::Positive};
		let mut it = mon_string.chars().peekable();

		result.sign = skip(&mut it);
		result.coefficient = match it.peek() {
			Some(t) => {
				if *t == 'x' {
					1
				} else {
					atoi(&mut it)
				}
			},
			None => 0
		};
		result.power = match it.peek() {
			Some(_t) => {
				it.next();
				skip(&mut it);
				if it.peek().is_some() {
					atoi(&mut it)
				}
				else {
					1
				}
			},
			None => 0
		};

		result
	}

	pub fn get_power(&self) -> u32 {
		self.power
	}

	pub fn get_sign(&self) -> Sign {
		self.sign
	}

	pub fn change_sign(&mut self) {
		if self.sign == Sign::Positive {
			self.sign = Sign::Negative;
		}
		else {
			self.sign = Sign::Positive;
		}
	}

	pub fn add(&mut self, monomial: &Monomial) -> Result<(), ()> {
		if self.power == monomial.power {
			if self.sign == monomial.sign {
				self.coefficient += monomial.coefficient;
			}
			else {
				if self.coefficient < monomial.coefficient {
					self.coefficient = monomial.coefficient - self.coefficient;
					self.change_sign();
				}
				else {
					self.coefficient -= monomial.coefficient;
				}
			}
		}
		else {
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