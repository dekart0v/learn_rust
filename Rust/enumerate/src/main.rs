enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    println!("enum");

    let player_direction:Direction = Direction::Down; // : это direction.up (что-то такое)

    match player_direction { // switch case
        Direction::Up => println!("We are heading up!"), // работает это потому что переменная Up
        Direction::Down => println!("We are heading down!"),
        Direction::Left => println!("We are heading left!"),
        Direction::Right => println!("We are heading right!"), // ВНИМАНИЕ НА ЗАПЯТЫЕ
    }
}
