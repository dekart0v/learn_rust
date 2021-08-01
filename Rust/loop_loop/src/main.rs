fn main() {
    println!("loop_loop");

    let mut i = 0;

    loop {
        i += 1;

        if i == 5 {
            continue; // back to the begining of the loop
        }

        if i >= 10 { 
            break; // stop the loop
        }
        
        println!("the value of N is {}", i);
    }
}
