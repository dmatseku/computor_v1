use super::sign::Sign;

pub struct Term {
	coefficient: u32,
	power: u8,
	sign: Sign
}

impl Term {
    pub fn new(data: string) -> Term {
        let result: Term;


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

    pub fn add(&mut self, term: &Term) -> Result<(), ()> {
        if self.power == term.power {
            if self.sign == term.sign {
                self.coefficient += term.coefficient;
            }
            else {
                if self.coefficient < term.coefficient {
                    self.coefficient = term.coefficient - self.coefficient;
                    self.change_sign();
                }
                else {
                    self.coefficient -= term.coefficient;
                }
            }
        }
        else {
            return Err(());
        }

        Ok(())
    }
}