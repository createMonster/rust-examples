fn main() {
    println!("There are {} days in Feb", 28);
    
    println!("Hi {0}, this is {1}, hi {1} this is {0}", "Alex", "Bob");

    println!(
        "{subject} {verb} {object}",
        subject="Alice",
        verb="is going to eat",
        object="buffet"
    );

    println!("{number:>5}", number=1);
    println!("{number:0<10}", number=9);

    let number: f64 = 99.99;
    let width: usize = 10;
    println!("{number:0>width$}");
}