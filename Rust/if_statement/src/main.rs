fn main() {
    println!("if_statement");

    let x = 50;

    if x > 50 {
        println!("X is greater then 50. X is {}.", x);
    }
    else if x == 50 {
        println!("X equals 50.");
    }
    else {
        println!("X is lower then 50. X is {}.", x);
    }
}
