use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

// Implement Display for MinMax
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64, 
    y: f64
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Display Acivity
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let minmax = MinMax(0, 14);
    
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug mode: {:?}", minmax);

    let big_ranage =  MinMax(-300, 300);
    let small_range = MinMax(-10, 10);
    println!("The big is {big} and small is {small}",
            big=big_ranage,
            small=small_range);
    
    
    let point = Point2D { x:3.3, y:19.1 };
    println!("Compare points!");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Display Acivity
    let complex_number = Complex { real:-3.3, imag:7.2 };
    println!("Compare Complex numbers!");
    println!("Display: {}", complex_number);
    println!("Debug: {:?}", complex_number);
}