use quantum_stuff::complex::*;

pub fn main() {
    let a = Complex::new(0.0, 3.0);
    let b = Complex::new(-1.0, -1.0);

    println!("a-b: {}", a - b);
    println!("a/b: {}", a / b);
    println!("a*: {}", a.conjugate());
    println!("|b|: {}", b.modulus());
}