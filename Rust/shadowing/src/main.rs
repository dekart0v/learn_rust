fn main() {
    println!("shadowing");

    let x = 10;
    {
        let x = 15;
        // do stuff with X
    }
    
    let x = "string"; // let потому что нам надо менять тип даты, мы просто заново создаем переменную
    println!("x is {}", x); // i want here untouched initial value of X

    let x = true; 
    println!("x is {}", x);
}
