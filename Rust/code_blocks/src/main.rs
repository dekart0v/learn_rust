fn main() {
    println!("code_blocks");
    // в код блоке доступно то что есть тут
    {
        //code block
    }

    let x = 10;
    {
        let y = 5;

        println!("{} {}", x, y);
    }
    //println!("{} {}", x, y); // y не будет найдено потому что y находтся в код блоке
}
