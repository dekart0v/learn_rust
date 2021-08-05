fn main() {
    println!("functions");

    print_numbers_to(20);
    
    /*
    if is_even(42) {
        println!("It is even!");
    }
    */
}

/*
fn print_numbers_to(mut count: u32) {
    let mut number = 0;
    while count > 0 {
        count -= 1;
        println!("{}", number);
        number += 1;
    }
}
*/

fn print_numbers_to(num: u32) {
    for i in 1..num {
        if is_even(i) {
            println!("{} is even", i);
        }
        else {
            println!("{} is odd", i);
        }
    }
}

/*
fn is_even(num: u32) -> bool { // -> значит вернется тип данного именно такой
    if num % 2 == 0 {
        println!("{} is even", num);
        return true;
    }
    else {
        println!("{} is odd", num);
        return false;
    }
}
*/

fn is_even(num: u32) -> bool {
    return num % 2 == 0; // если верно то True, если нет то False
}