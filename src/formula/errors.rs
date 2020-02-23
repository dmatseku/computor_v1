use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct SyntaxError;

impl Display for SyntaxError {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "Syntax Error!")
	}
}