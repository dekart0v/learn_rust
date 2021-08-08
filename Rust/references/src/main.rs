// pointers

fn main() {
    println!("references");

    let x = 10; //
    let x_reference = &x; // x_reference это поинтер; &x = значение x; immutable reference
    println!("x is {}", x_reference);

    let mut y = 5; 
    {  
        let y_reference_mut = &mut y; // mutable reference (pointer)
        *y_reference_mut += 1; // * берет значение из ячейки памяти
        println!("y is {}", y_reference_mut);
    }
    println!("y is {}", y); // просто y is immutable, чтобы можно было это вызвать, надо references использовать в code blocks
}
