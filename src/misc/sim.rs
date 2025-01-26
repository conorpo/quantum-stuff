use std::array;

use crate::complex::*;
use crate::static_::{
    matrix::*,
    state::*
};

fn dynamic_system<const N: usize, F: Real>(initial: &State<N, F>, dynamics: &Matrix<N,N,F>, time_steps: usize) -> State<N,F> {
    //Test valid dynamics matrix
    let mut res: Matrix<N,N, F> = Matrix::eye();
    let mut mult = dynamics.clone();
    let mut k = time_steps;
    while k > 0 {
        if k % 2 == 1 {
            res = &mult * &res;
        }
        k /= 2;
        mult = &mult * &mult;
    }

    &res * initial
}

fn deterministic<const N: usize>(initial: &State<N, f32>, dynamics: &Matrix<N,N,f32>, time_steps: usize) -> Result<State<N,f32>,  &'static str>{
    for c in 0..N {
        let mut outgoing = 0.0;
        for r in 0..N {
            let edge = dynamics.data[r][c];
            if edge != c32!(1) && edge != c32!(0) {
                return Err("Dynamics Matrix entries must be boolean values, 1 or 0");
            }
            outgoing += edge.r;
        }
        if outgoing != 1.0 {
            return Err("Only one outgoing edge per vertex");
        }
    }

    Ok(dynamic_system(initial, dynamics, time_steps))
}

fn probabilistic<const N: usize>(initial: &State<N, f32>, dynamics: &Matrix<N,N,f32>, time_steps: usize) -> Result<State<N,f32>,  &'static str> {
    for c in 0..N {
        let mut outgoing = 0.0;
        for r in 0..N {
            let edge = dynamics.data[r][c];
            if edge.i != 0.0 || edge.r < 0.0 || edge.r > 1.0 {
                return Err("Weights must represent probabilities, real values between 0 and 1.");
            }
            outgoing += edge.r;
        }
        if (outgoing - 1.0).abs() > f32::EPSILON{
            return Err("Outgoing edges must sum to 1.0 per vertex");
        }
    }
    
    Ok(dynamic_system(initial, dynamics, time_steps))
}

fn multislit<const SLITS: usize, const TARGETS: usize>(probabilties: &[(usize, usize, f32)]) -> State<{SLITS + TARGETS + 1}, f32>
where [(); SLITS + TARGETS + 1]: {
    let initial = State::<{SLITS + TARGETS + 1}, f32>::new(array::from_fn(|i| Complex::new((i == 0) as u8 as f32, 0.0)));

    let mut dynamics_data = [[Complex::<f32>::default(); {SLITS + TARGETS + 1}]; {SLITS + TARGETS + 1}];

    let slit_prob = 1.0 / (SLITS as f32);
    for i in 1..(1 + SLITS) {
        dynamics_data[i][0].r = slit_prob;
    }

    for &(slit, target, prob) in probabilties {
        let slit_i = slit + 1;
        let target_i = target + SLITS + 1;
        dynamics_data[target_i][slit_i].r = prob;
    }

    let dynamics = Matrix {
        data: dynamics_data
    };

    
    dynamic_system(&initial, &dynamics, 2)
}

// Does not require unitary matrices as of now

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::static_::{
        matrix::*,
        state::*
    };
    use crate::complex::*;
    use super::*;

    #[test]
    fn exc_3_1_1() {
        let initial = vec32![5,5,0,2,0,15];
        let dynamics = mat32![[0,0,0,0,0,0],
                              [0,0,0,0,0,0],
                              [0,1,0,0,0,1],
                              [0,0,0,1,0,0],
                              [0,0,1,0,0,0],
                              [1,0,0,0,1,0]];

        assert!(deterministic(&initial, &dynamics, 1).unwrap().fuzzy_equals(&vec32![0,0,20,2,0,5]));

        let initial = vec32![0,0,100,0,0,0];
        let res = deterministic(&initial, &dynamics, 6).unwrap();
        assert!(res.fuzzy_equals(&initial));
    }

    #[test]
    fn ex_3_2_2() {
        let initial = vec32![1,0,0,0];
        let dynamics = mat32![[0,0.5,0.5,0],
                              [0.5,0,0,0.5],
                              [0.5,0,0,0.5],
                              [0,0.5,0.5,0]];
        let res = probabilistic(&initial, &dynamics, 2).unwrap();
        assert!(res.fuzzy_equals(&vec32![0.5,0,0,0.5]));
        let _3 = probabilistic(&initial, &dynamics, 3).unwrap();
        let _1003 = probabilistic(&initial, &dynamics, 1003).unwrap();
        assert!(_3.fuzzy_equals(&_1003));
    }

    #[test]
    fn slit() {
        let third = 1.0 / 3.0;
        let res = multislit::<2,5>(&[(0,0,third), (0,1,third), (0,2,third),
                                     (1,2,third), (1,3,third), (1,4,third)]);

        let expected = State::new([Complex::new(0.0, 0.0), Complex::new(0.0,0.0), Complex::new(0.0, 0.0), Complex::new(1.0 / 6.0, 0.0), Complex::new(1.0 / 6.0, 0.0), Complex::new(1.0 / 3.0, 0.0), Complex::new(1.0 / 6.0, 0.0), Complex::new(1.0 / 6.0, 0.0)]);
        assert!(res.fuzzy_equals(&expected));
    }

    #[test]
    fn quantum_double_slit() {
        let initial = vec64![1, 0, 0, 0, 0, 0, 0, 0];
        let mut data = [[Complex::<f64>::default(); 8];8];
        data[1][0].r = 1.0 / 2.0.sqrt();
        data[2][0].r = 1.0 / 2.0.sqrt();
        data[3][1] = c64!(-1 + 1 i) / 6.0.sqrt();
        data[4][1] = c64!(-1 - 1 i) / 6.0.sqrt();
        data[5][1] = c64!(1 +- 1 i) / 6.0.sqrt();
        data[5][2] = c64!(-1 + 1 i) / 6.0.sqrt();
        data[6][2] = c64!(-1 - 1 i) / 6.0.sqrt();
        data[7][2] = c64!(1 - 1 i) / 6.0.sqrt();
        data[3][3].r = 1.0;
        data[4][4].r = 1.0;
        data[5][5].r = 1.0;
        data[6][6].r = 1.0;
        data[7][7].r = 1.0;
        
        let dynamics = Matrix {
            data
        };

        let res = dynamic_system(&initial, &dynamics, 2);
        println!("{:?}", &res.probabilities());
    }
}