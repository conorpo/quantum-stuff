
use std::slice::Iter;

use crate::static_::state::*;
use crate::static_::operator::*;
use crate::complex::*;

pub fn discrete_points<const N: usize>(initial: &State<N, C64>, target: &State<N, C64>) -> f64 {
    //Use canonical basis of vector space
    let norm_i_squared = initial.dot(&initial).r;
    let norm_t_squared = target.dot(&target).r;
    assert_ne!(norm_t_squared, 0.0);
    assert_ne!(norm_i_squared, 0.0);

    let transition_amplitude_unnormalized = target.dot(&initial);

    (transition_amplitude_unnormalized * transition_amplitude_unnormalized.conjugate()).r / (norm_i_squared * norm_t_squared)
}

pub fn probability_at_point<const N: usize>(initial: &State<N, C64>, at: usize) -> f64 {
    let mut target_data = [C64::ZERO; N];
    if let Some(at) = target_data.get_mut(at) {
        at.r = 1.0;
    }

    let target = State {
        data: target_data
    };

    discrete_points(initial, &target)
}

pub fn dynamic_system<'a,const N: usize, C: Complex + 'a>(initial: &State<N,C>, mut dynamics_iter: impl Iterator<Item = &'a Operator<N,C>>) -> State<N,C> {
    let mut cur = initial.clone();
    while let Some(mat) = dynamics_iter.next() {
        cur = mat * &cur;
    }
    cur
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_4_1_1() {
        let initial = state64![-3,-1;0,-2;0,1;2];
        let res = probability_at_point(&initial, 2);
        println!("{res}");
        assert!((res- (1.0 / 19.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn ex_4_1_7() {
        let initial = state64![1; 0,-1];
        let target = state64![0,1;  1];
        let res = discrete_points(&initial, &target);
        println!("{res}");
        assert!((res - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn ex_4_4_2() {
        let initial = state64![1;0;0;0];
        let entry = 1.0f64 / 2.0.sqrt();
        let mut dynamics = op64![[0       ;entry ;entry ;0      ],
                                 [0,entry ;0     ;0     ;entry  ],
                                 [entry   ;0     ;0     ;0,entry],
                                 [0       ;entry ;-entry;0]];

        let dynamics_iter = (0..3).map(|_| {
            &dynamics
        });

        let end = dynamic_system(&initial, dynamics_iter);
        assert!(end.fuzzy_equals(&State::new([c64!(0),C64::new(-1.0 / 2.0.sqrt(),1.0 / 2.0.sqrt()),c64!(0),c64!(0)])));
    }
}