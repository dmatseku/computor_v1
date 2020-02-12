use super::sign::Sign;
use regex::*;

pub struct Monomial {
	coefficient: u32,
	power: u8,
	sign: Sign
}

//r"^((\+ |- )?(-|\+)?((\d{0,9})|(\d{0,9}x(\^(\d{1,9}))?)))$"

impl Monomial {
	pub fn new(data: string) -> Monomial {
		let result: Monomial;



	}

	pub fn get_power(&self) -> u8 {
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