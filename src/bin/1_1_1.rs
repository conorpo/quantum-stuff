use quantum_stuff::complex::*;

pub fn main() {
    let a = Complex::new(-3,1);
    let b = Complex::new(2,-4);

    println!("Sum is: {}", a + b);
    println!("Product is: {}", a * b);
}