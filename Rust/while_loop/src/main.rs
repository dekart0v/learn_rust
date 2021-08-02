fn main() {
    println!("while loop");

    let mut n = 1;
    while n <= 50 {
        if n % 5 == 0 { // если число кончается на 5 или 0
            println!("n is {}", n);
        }
        n += 1;
    }
}
