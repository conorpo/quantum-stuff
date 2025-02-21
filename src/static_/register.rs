use super::state::*;
use super::operator::*;
use crate::complex::*;
use std::rc::Rc;
use std::cell::Cell;

pub const fn num_qubits_to_size(num_qubits: usize) -> usize {
    2usize.pow(num_qubits as u32)
}

pub struct Register<const REGISTER_SIZE:usize, const SELECT_SIZE: usize> 
where [(); num_qubits_to_size(REGISTER_SIZE)] : ,
{
    register: Rc<Cell<Option<State<{num_qubits_to_size(REGISTER_SIZE)}, C64>>>>,
    offset: usize,
}

impl<const NUM_QUBITS: usize> Register<NUM_QUBITS, NUM_QUBITS> 
where [(); num_qubits_to_size(NUM_QUBITS)] : ,
{
    pub fn new(state: State<{num_qubits_to_size(NUM_QUBITS)},C64>) -> Self {
        Self {
            register: Rc::new(Cell::new(Some(state))),
            offset: 0
        }
    }
}

impl<const REGISTER_SIZE: usize, const SELECT_SIZE: usize> Register<REGISTER_SIZE, SELECT_SIZE> 
where [(); num_qubits_to_size(REGISTER_SIZE)]: ,
      [(); num_qubits_to_size(SELECT_SIZE)]: ,
{
    pub fn select<const SIZE2: usize, const OFFSET2: usize>(&self) -> Register<REGISTER_SIZE, {SIZE2}> 
        where [(); OFFSET2 + SIZE2 - SELECT_SIZE]: ,
    {
        Register::<REGISTER_SIZE, {SIZE2}>  {
            register: self.register.clone(),
            offset: self.offset + OFFSET2
        }
    }

    //For each i in 2^offset we want to
    //For each element of op we want to
    //Place an n*n identity * the element
    pub fn apply(&self, op: & Operator<{num_qubits_to_size(SELECT_SIZE)}, C64>) 
    {
        let mut final_op_data = Operator::<{num_qubits_to_size(REGISTER_SIZE)},C64>::zero().data;

        let mut left_offset = 0;
        let right_stride = num_qubits_to_size(REGISTER_SIZE - self.offset - SELECT_SIZE);
        for left_i in 0..(2usize.pow(self.offset as u32)) {
            for op_r in 0..num_qubits_to_size(SELECT_SIZE) {
                for op_c in 0..num_qubits_to_size(SELECT_SIZE) {
                    let elem = *op.get(op_r,op_c).unwrap();
                    let offset_r = left_offset + op_r * right_stride;
                    let offset_c = left_offset + op_c * right_stride;
                    for right_i in 0..right_stride {
                        final_op_data[offset_r + right_i][offset_c + right_i] = elem;
                    }
                }
            }

            left_offset += 2usize.pow((REGISTER_SIZE - self.offset) as u32);
        }

        let final_op = Operator::new(final_op_data);
        
        let old = self.register.take().unwrap();
        let new = &final_op * &old;
        self.register.set(Some(new));
    }
}