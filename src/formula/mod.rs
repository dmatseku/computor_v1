mod sign;
mod monomial;
mod errors;

#[derive(Copy, Clone)]
pub struct Formula {
	left_side: Vec<monomial::Monomial>,
	right_side: Vec<monomial::Monomial>,
}

impl Formula {
	pub fn new(formula_string: &str) -> Result<Formula, errors::SyntaxError> {

	}

	fn shorten(&self) {

	}
}
