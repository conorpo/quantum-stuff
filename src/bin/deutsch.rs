use quantum_stuff::dynamic::*;
use quantum_stuff::complex::*;

pub fn main() {
    let mut input = State::qubit(false);
    let mut output = State::qubit(true);
    // 0
    
    let H = Operator::hadamard();
    input.apply(&H);
    output.apply(&H);

    //1

    let mut state = input.tensor_product(output);    
    
    // //Change this
    let my_func = [0,1];
    let mut u = Matrix::zeroes(4);
    *u.get_mut(my_func[0] ^ 0, 0) = C64::ONE;
    *u.get_mut(my_func[0] ^ 1,1) = C64::ONE;
    *u.get_mut(my_func[1] ^ 0,2) = C64::ONE;
    *u.get_mut(my_func[1] ^ 1,3) = C64::ONE;
    let U = Operator::try_from(u).unwrap();

    state.apply(&U);

    //2

    state.apply_partial(0..1, &H);

    //3
    
    let res = state.measure_partial(0..1);
    if res == 0 {
        println!("Constant");
    } else {
        println!("Balaanced")
    }
}