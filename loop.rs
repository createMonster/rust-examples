fn main() {
    let mut count = 0u32;

    println!("Let's count unitil infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("Three!");
            continue;
        }
        println!("{}", count);

        if count == 10 {
            println!("OK, that's enough");
            break;
        }
    };
}