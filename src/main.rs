mod check_correct_form;
mod find_answer;
mod formula;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Too few arguments");
    } else {
        let mut it = args.iter();

        it.next();
        for input_str in it {
            println!("\n=== {} ===\n", input_str);

            match formula::Formula::new(input_str) {
                Ok(t) => {
                    let degree = t.get_left_side().last().unwrap().get_power();
                    println!("Polynomial degree: {}", degree);
                    if !check_correct_form::check_correct_form(&t) {
                        return;
                    }
                    if degree == 2 {
                        find_answer::square_answer(&t);
                    } else {
                        find_answer::simple_answer(&t);
                    }
                }
                Err(_e) => println!("Syntax error"),
            }
        }
    }
}
