use std::fmt::{Display, Formatter};
use std::iter::Peekable;

fn str_to_i(it: &mut Peekable<std::str::Chars>) -> i32 {
    let mut res = 0;
    let mut opt: Option<&char> = it.peek();

    //if char is not number, then I don't move the iterator
    while opt.is_some() {
        let c = *(opt.unwrap());

        if c < '0' || c > '9' {
            break;
        } else {
            res = res * 10 + (c as i32 - 48)
        }
        it.next();
        opt = it.peek();
    }

    res
}

fn str_to_f(it: &mut Peekable<std::str::Chars>) -> f32 {
    let mut res = 0.0;
    let mut opt: Option<&char> = it.peek();

    //if char is not number, then I don't move the iterator
    while opt.is_some() {
        let c = *(opt.unwrap());

        if c < '0' || c > '9' {
            if c == '.' {
                it.next();
                opt = it.peek();
            }
            break;
        } else {
            res = res * 10.0 + (c as i32 - 48) as f32;
        }
        it.next();
        opt = it.peek();
    }
    let mut i = 1;
    while opt.is_some() {
        let c = *(opt.unwrap());

        if c < '0' || c > '9' {
            break;
        } else {
            res += (c as i32 - 48) as f32 / (10 * i) as f32;
        }
        i += 1;
        it.next();
        opt = it.peek();
    }

    res
}

//function returns the final sign of the number
fn skip(it: &mut Peekable<std::str::Chars>) -> i32 {
    let mut opt = it.peek();
    let mut res: i32 = 1;

    //if char is a number, then I don't move the iterator
    while opt.is_some() {
        let c = *(opt.unwrap());

        if (c >= '0' && c <= '9') || c == 'X' {
            break;
        } else if c == '-' {
            res *= -1;
        }
        it.next();
        opt = it.peek();
    }

    res
}

#[derive(Copy, Clone)]
pub struct Monomial {
    coefficient: f32,
    power: i32,
}

impl Monomial {
    pub fn new(mon_string: &str) -> Monomial {
        let mut result: Monomial = Monomial {
            coefficient: 0.0,
            power: 0,
        };
        let mut it = mon_string.chars().peekable();
        let mut modifier: i32;

        //skip to coefficient
        modifier = skip(&mut it);
        result.coefficient = match it.peek() {
            Some(t) => {
                if *t == 'X' {
                    //coefficient wasn't entered
                    modifier as f32
                } else {
                    modifier as f32 * str_to_f(&mut it)
                }
            }
            None => 0.0,
        };
        result.power = match it.peek() {
            Some(_t) => {
                //if 'X' entered
                skip(&mut it); // skip a possible "\s*\*?\s*"
                it.next(); //skip 'X'
                modifier = skip(&mut it);
                if it.peek().is_some() {
                    modifier * str_to_i(&mut it)
                } else {
                    modifier
                }
            }
            None => 0,
        };

        result
    }

    pub fn get_coefficient(&self) -> f32 {
        self.coefficient
    }

    pub fn get_power(&self) -> i32 {
        self.power
    }

    pub fn change_sign(&mut self) {
        self.coefficient = -self.coefficient;
    }

    pub fn add(&mut self, monomial: &Monomial) -> Result<(), ()> {
        if self.power == monomial.power {
            self.coefficient += monomial.coefficient;
            if self.coefficient == 0.0 {
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
    //display in some variants
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut coefficient = self.coefficient;

        if coefficient < 0.0 {
            coefficient *= -1.0;
        }
        if coefficient == 0.0 || self.power == 0 {
            write!(f, "{}", coefficient)
        } else if self.power == 1 {
            if coefficient == 1.0 {
                write!(f, "X")
            } else {
                write!(f, "{}X", coefficient)
            }
        } else {
            if coefficient == 1.0 {
                write!(f, "X^{}", self.power)
            } else {
                write!(f, "{}X^{}", coefficient, self.power)
            }
        }
    }
}
