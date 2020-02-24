use std::fmt::{Display, Formatter, Result, Debug};

#[derive(Clone)]
pub struct SyntaxError;

impl Display for SyntaxError {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "Syntax Error!")
	}
}

impl Debug for SyntaxError {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "Syntax Error!")
	}
}