fn main() {
    println!("for loop");

    for i in 1..11 { // 11 не вкючено
        println!("the number is {}", i);
    }

    let numbers = 30..51;
    for i in numbers {
        println!("NUMBER is {}", i);
    }

    let animals = vec!["dog", "cat", "female", "woman", "slut"]; // vec! это вектор(массив)
    //for i in animals { МОЖНО ИСПОЛЬЗОВАТЬ НО ПОТОМ ВЕКТОР СТАНОВИТСЯ НЕДОСТУПНЫМ ДЛЯ ИСПОЛЬЗОВАНИЯ
    //    println!("animal is {}", i); 
    //}

    //animals = vec!["sgggg", "sdg", "dsa", "fac", "dog"];
    for (index, i) in animals.iter().enumerate() { // ПОЭТОМУ МЫ ВЫЗЫВАЕМ ЭТОТ МЕТОД
        println!("animal #{} is {}", index+1, i); // внимание на порядок!
    }
}
