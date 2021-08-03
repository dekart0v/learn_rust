fn main() {
    println!("tuples");


    let tup1 = (20, 30, 40, 50, "sex", 34.1, false, (4, 1));
    println!("{}", (tup1.7).0); // (<tuple_name>.7).0 == tuple_name[7][0]

    let tup2 = (20, 30, 40, 50, "sex", 34.1, false);
    let (a, b, c, d, e, f, g) = tup2;
    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);
    println!("d is {}", d);
    println!("e is {}", e);
    println!("f is {}", f);
    println!("g is {}", g);

}
