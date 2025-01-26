use super::state::*;
use super::matrix::*;
use core::num;
use std::rc::Rc;
use std::cell::Cell;

type StaticPrecision = f64;

const fn num_qubits_to_size(num_qubits: usize) -> usize {
    2usize.pow(num_qubits as u32)
}

struct Register<const SELECT_QUBITS: usize, const REGISTER_QUBITS: usize> 
where [(); {num_qubits_to_size(REGISTER_QUBITS)}] : ,
{
    register: Rc<Cell<Option<State<{num_qubits_to_size(REGISTER_QUBITS)}>>>>,
    qubit_offset: usize
}

impl<const SELECT_QUBITS: usize, const REGISTER_QUBITS: usize> Register<SELECT_QUBITS,REGISTER_QUBITS> 
where [(); {num_qubits_to_size(REGISTER_QUBITS)}] : ,
      [(); {num_qubits_to_size(SELECT_QUBITS)}] : ,
{
    type Operator = Matrix<{num_qubits_to_size(SELECT_QUBITS)},{num_qubits_to_size(SELECT_QUBITS)}>;

    pub fn new() -> Self {
        Self {
            register: Rc::new(Cell::new(Some(State::zero()))),
            qubit_offset: 0
        }
    }

    fn select<const OFFSET: usize, const SELECT_SIZE: usize>(&self) -> Register<SELECT_SIZE, REGISTER_QUBITS> 
    where 
        [(); num_qubits_to_size(REGISTER_QUBITS)] :,
        [(); num_qubits_to_size(SELECT_SIZE)] :,
        [();OFFSET + SELECT_SIZE - SELECT_QUBITS] :,
    {
        Register::<SELECT_SIZE, REGISTER_QUBITS> {
            register: self.register.clone(),
            qubit_offset: OFFSET
        }
    }

    fn apply(&self, operator: &Self::Operator) {
        let current_state = self.register.take().unwrap();
        let qubit_size = current_state.dim().log2();
    }
}