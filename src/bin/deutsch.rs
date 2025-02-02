use quantum_stuff::static_::*;
use quantum_stuff::complex::*;

pub fn main() {
    // 01
    // let state: State<4, C64> = State::new([C64::ZERO, C64::ONE, C64::ZERO, C64::ZERO]);
    
    // let H_H = Operator::<2,C64>::H.tensor_product(Operator::<2,C64>::H);

    // let state = &H_H * &state;
    
    // //Change this
    // let my_func = [0,1];
    // let mut u = Operator::<4,C64>::zero();
    // u.data[my_func[0] ^ 0][0] = C64::ONE;
    // u.data[my_func[0] ^ 1][1] = C64::ONE;
    // u.data[my_func[1] ^ 0][2] = C64::ONE;
    // u.data[my_func[1] ^ 1][3] = C64::ONE;

    // let state = &u * &state;

    // let H_I = Operator::<2,C64>::H.tensor_product(Operator::<2,C64>::IDENTITY);

    // let state = &H_I * &state;

    // let res = state.measure();
    // if res < 2 {
    //     println!("Constant!");
    // } else {
    //     println!("Balanced!");
    // }


}