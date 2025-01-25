use crate::{
    complex::*,
    static_::vector::*,
    static_::matrix::*,
};

pub fn expected_value<const N: usize, F: Field>(observable: &Matrix<N,N,F>, state: &Vector<N,F>) -> Result<F,&'static str> {
    if !observable.is_hermitian() { return Err("Observables need to be Hermitian matrices.")}

    Ok(state.dot(&(observable * state)).r)
}   

pub fn variance<const N: usize, F: Field>(observable: &Matrix<N,N,F>, state: &Vector<N,F>) -> Result<F, &'static str> {
    let expected = expected_value(observable, state)?;

    let demeaned_observable = -(Matrix::eye() * Complex::new(expected, F::default())) + observable;

    Ok(expected_value(&(&demeaned_observable * &demeaned_observable), state)?)
}

#[cfg(test)]
mod tests {
    use crate::misc::operator::{expected_value, variance};
    use crate::{
        static_::vector::*,
        complex::*,
        static_::matrix::*,
    };

    #[test]
    fn ex_4_2_5() {
        let c: f64 = 2.0.sqrt() / 2.0;
        let state = Vector::new([Complex::new(c,0.0), Complex::new(0.0,c)]);
        let observable = mat64![[1, 0 - 1 i],
                                [0 + 1 i , 2]];
        
        let expectation = expected_value(&observable, &state).unwrap();
        println!("{expectation}");
        assert!((expectation - 2.5).abs() < f64::EPSILON * 100.0);

        let var = variance(&observable, &state).unwrap();
        println!("{var}");
        assert!((var - 0.25).abs() < f64::EPSILON);

    }
}