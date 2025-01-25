use quantum_stuff::emulator::{lexer::*, emulator::*};

pub fn main() {
    let tokens = scan(&mut 
"INITIALIZE R 2
U TENSOR H H
APPLY U R
MEASURE R RES
".as_bytes()).unwrap();

    print!("{:?}", tokens);

    let res = emulate(tokens).unwrap();
    print!("{res}");
}