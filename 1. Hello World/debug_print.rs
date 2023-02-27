#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    println!("Now we are printing {:?}", Structure(5));

    println!("Now we are printing {:?}", Deep(Structure(3)));

    let name = "Peter";
    let age = 18;
    let person = Person{ name, age };
    println!("This person is {:?}", person);
    println!("{:#?}", person); // Pretty printing
}