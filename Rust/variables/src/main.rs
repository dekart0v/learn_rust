/*

let var_name: VAR_TYPE = <value>

TYPES:
1) i32 - signed integer 32b
2) i64 - signed integer 64b
3) u32 - unsigned integer 32b
4) u64 - unsigned integer 64b
5) f32 - floating point 32b
6) f64 - floating point 64b
7) bool - true/false
8) char - characters/emojis
...

*/

fn main() {
    println!("variables");

    let mut x: i32 = 10; // mut means X is changable


    println!("1) X is {}", x);
    
    x = 100;
    println!("2) X is {}", x);
}
