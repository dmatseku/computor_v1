use crate::formula::Formula;

pub fn check_correct_form(form: &Formula) -> bool {
    let left_last = form.get_left_side().last().unwrap();

    if form.get_left_side().len() == 1 && left_last.get_power() == 0 {
        if left_last.get_coefficient() == 0.0 {
            println!("All the real numbers are solution");
        } else {
            println!("There are no solutions");
        }
        return false;
    }
    if left_last.get_power() > 2 {
        println!("The polynomial degree is strictly greater than 2, I can't solve it");
        return false;
    } else if form.get_left_side().first().unwrap().get_power() < 0 {
        println!("The polynomial degree is less than 0, I can't solve it");
        return false;
    }

    true
}
