use quantum_stuff::complex::*;

pub fn main() {
    let a = Complex::new(-3f64,1.0);
    let b = Complex::new(2.0,-4.0);

    println!("Sum is: {}", a + b);
    println!("Product is: {}", a * b);
}