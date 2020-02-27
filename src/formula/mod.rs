mod errors;
mod monomial;

use self::monomial::Monomial;
use regex::Regex;
use std::cmp::Ordering;

pub struct Formula {
    left_side: Vec<monomial::Monomial>,
}

impl Formula {
    pub fn get_left_side(&self) -> &Vec<monomial::Monomial> {
        &self.left_side
    }

    fn complete_vec(&mut self, formula_sub_string: &str, side: bool) {
        //create one side
        let re = Regex::new(
			r"((\+\s*|-\s*)?[-,+]?(((\d{1,9}(.\d{1,6})?)?(\s*\*\s*)?X(\^-?\d{1,9})?)|(\d{1,9}(.\d{1,6})?)))"
		).unwrap();
        let caps = re.find_iter(formula_sub_string);

        for str in caps {
            //add all operands to the vector
            let mut new_mon = Monomial::new(str.as_str());

            if side {
                new_mon.change_sign();
            }

            //if the vector has a monomial with the same power,
            // then a new monomial is added to the existing one.
            // Otherwise it is added to the vector
            match self
                .left_side
                .iter_mut()
                .find(|x| x.get_power() == new_mon.get_power())
            {
                Some(t) => {
                    t.add(&new_mon).unwrap();
                }
                None => {
                    self.left_side.push(new_mon);
                }
            }
        }

        //delete null's elements
        let mut i = 0;
        while i < self.left_side.len() {
            if self.left_side[i].get_coefficient() == 0.0 && self.left_side.len() > 1 {
                self.left_side.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn new(formula_string: &str) -> Result<Formula, errors::SyntaxError> {
        let mut res: Formula = Formula {
            left_side: Vec::new(),
        };

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

        res.complete_vec(substrings[0], false);
        res.complete_vec(substrings[1], true);

        res.left_side.sort_unstable_by(|a, b| {
            if a.get_power() < b.get_power() {
                return Ordering::Less;
            } else if a.get_power() > b.get_power() {
                return Ordering::Greater;
            }
            Ordering::Equal
        });

        print!("Reduced form: ");
        res.print_formula();
        println!();

        Ok(res)
    }

    pub fn print_formula(&self) {
        let mut it = self.left_side.iter();
        let mut first = true;

        while match it.next() {
            Some(t) => {
                //print sign
                if first {
                    if t.get_coefficient() < 0.0 {
                        print!("-");
                    }
                    first = false;
                } else {
                    print!(
                        "{}",
                        if t.get_coefficient() < 0.0 {
                            "- "
                        } else {
                            "+ "
                        }
                    );
                }

                print!("{} ", t);
                true
            }
            None => false,
        } {}
        print!("= 0");
    }
}
