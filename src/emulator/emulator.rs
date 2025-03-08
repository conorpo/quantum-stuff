use core::num;
use std::cell::Cell;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;
use std::rc::Rc;
use std::error::Error;

use super::{
    lexer::{Token, TokenType, PrimitiveGate},
};

use crate::dynamic::*;

struct Register {
    pub state: Rc<Cell<Option<State>>>,
    pub interval: (usize, usize)
}

impl Register {
    fn new(state: Rc<Cell<Option<State>>>, interval: (usize, usize)) -> Self {
        Self {
            state,
            interval
        }
    }
    
    fn apply(&self, gate: &Gate) -> Result<(),()> {
        let range = self.interval.0..self.interval.1;
        if gate.get().dim().0 == range.len() {
            return Err(());
        };

        let mut x: State = self.state.take().unwrap();
        x.apply_partial(range, gate);
        self.state.set(Some(x));

        Ok(())
    }

    fn measure(&self) -> Result<usize, ()> {
        let interval = self.interval.0..self.interval.1;
        let mut state = self.state.take().unwrap();
        let (measured, rest) = state.measure_partial(interval);

        // A bit ugly but this emulator is secondary, just going to reconstruct the state
        // todo
        Ok(measured)
    }
}

fn parse_identifier(possible_token: Option<Token>, first_token: &Token, label: &'static str) -> Result<String, RuntimeError> {
    match possible_token {
        Some(Token { ty: TokenType::Identifier(name), ..}) => {Ok(name)},
        token if token.is_some() => { Err(RuntimeError::new(token, format!("Expected IDENTIFIER token for {label}, found"))) },
        None => { Err(RuntimeError::new(Some(first_token.clone()), format!("Missing {label} (IDENTIFER) for the"))) },
        _ => panic!("How did we get here?")
    }
}

type RegisterMap<'a> = HashMap<String, Register>;


fn get_register<'a>(possible_token: Option<Token>, first_token: &Token, registers: &'a RegisterMap) -> Result<&'a Register, RuntimeError> {
    let register_name = parse_identifier(possible_token.clone(), first_token, "register name argument")?;
    match registers.get(&register_name) {
        Some(entry) => Ok(entry),
        None => {
            return Err(RuntimeError::new(possible_token, "This register does not exist at this point,".to_owned()));
        }
    }
}


type OperatorMap = HashMap<String, Rc<Gate>>;

fn get_gate<'a>(possible_token: Option<Token>, first_token: &Token, gate_map: &'a OperatorMap) -> Result<Rc<Gate>, RuntimeError> {
    match possible_token.as_ref() {
        Some(Token {ty: TokenType::Identifier(ident),..}) => {
            match gate_map.get(ident) {
                Some(gate_ref) => { Ok(gate_ref.clone()) },
                None => {
                    Err(RuntimeError::new(possible_token, "Operator does not exist at this point in the program, for".to_owned()))
                }
            }
        }, 
        Some(Token {ty: TokenType::Gate(primitive), ..}) => {
            Ok(match *primitive {
                PrimitiveGate::CNOT => Rc::new(Gate::cnot()),
                PrimitiveGate::H => Rc::new(Gate::hadamard()),
                PrimitiveGate::R(theta) => Rc::new(Gate::phase_shift(theta)),
                PrimitiveGate::I(n) => Rc::new(Gate::identity(n))
            })
        },
        Some(_) => {
            Err(RuntimeError::new(possible_token, "Expected operator identifier OR gate primitive, found".to_owned()))
        },
        None => {
            Err(RuntimeError::new(Some(first_token.clone()), "Missing operator agument (IDENTIFIER | PRIMITIVE_GATE) for the".to_owned()))
        }
    }
}

//This is going to be cancer with const generic Vector / Matrix types.
pub fn emulate(tokens: Vec<Token>) -> Result<String, RuntimeError> {
    let mut results = String::new();

    let mut states: Vec<Rc<Cell<Option<State>>>> = Vec::new();
    let mut register_map: RegisterMap = HashMap::new();
    let mut operators: OperatorMap = HashMap::new();

    let mut token_iter = tokens.into_iter().peekable();
    while token_iter.peek().is_some() {
        let first_token = match token_iter.next() {
            Some(token) => token,
            None => { continue; }
        };

        match &first_token.ty {
            TokenType::Initialize => {
                let name = parse_identifier(token_iter.next(), &first_token, "register name argument")?;
                
                let num_qubits_token = token_iter.next();
                let num_qubits = match num_qubits_token.as_ref() {
                    Some(Token { ty: TokenType::Number(num_qubits), ..})  if (1..=8).contains(num_qubits) => {*num_qubits},
                    Some(_) => { return Err(RuntimeError::new(num_qubits_token, "Expected NUMBER within 1-8 (inclusive) for register qubit num_qubits, found".to_owned())); },
                    None => { return Err(RuntimeError::new(Some(first_token), "Missing register num_qubits argument (NUMBER) for the".to_owned())); }
                };

                let state = State::from_qubits((0..num_qubits).map(|_| false));
                let state_ref = Rc::new(Cell::new(Some(state)));
                states.push(state_ref.clone());
                register_map.insert(name, Register::new(state_ref, (0, num_qubits)));
            },

            TokenType::Select => {
                let name_token = token_iter.next();
                let name = match name_token {
                    Some(Token { ty: TokenType::Identifier(name), ..}) => {name}
                    Some(token) => { return Err(RuntimeError::new(Some(token), "Expected IDENTIFER for slice name, found".to_owned()))},
                    None => { return Err(RuntimeError::new(Some(first_token), "Missing slice name argument (IDENTIFER) for the ".to_owned())); }
                };

                let sub_register = get_register(token_iter.next(), &first_token, &register_map)?;
                let len = sub_register.interval.1 - sub_register.interval.0;

                let offset_token = token_iter.next();
                let sub_offset = match offset_token.as_ref() {
                    Some(Token {ty: TokenType::Number(offset), ..}) => {
                        if (0..len).contains(offset) {
                            *offset
                        } else {
                            return Err(RuntimeError::new(offset_token, format!("Offset outside of (sub)register bounds (0..{})", sub_register.len())));
                        }
                    },
                    Some(token) => { return Err(RuntimeError::new(offset_token, "Expected offset argument (NUMBER), found".to_owned())); },
                    None => { return Err(RuntimeError::new(Some(first_token), "Missing offset argument (NUMBER) for the".to_owned())); }
                };

                let num_qubits_token = token_iter.next();
                let num_qubits = match num_qubits_token.as_ref() {
                    Some(Token {ty: TokenType::Number(num_qubits), ..}) => {
                        if (1..=(len - sub_offset)).contains(num_qubits) {
                            *num_qubits
                        } else {
                            return Err(RuntimeError::new(num_qubits_token, format!("NUMQUBITS must be between 1 and {}, for ", sub_register.len() - sub_offset)));
                        }
                    },
                    Some(token) => { return Err(RuntimeError::new(num_qubits_token, "Expected NUMQUBITS argument (NUMBER), found".to_owned())); },
                    None => { return Err(RuntimeError::new(Some(first_token), "Missing NUMQUBITS argument (NUMBER) for the".to_owned())); }
                };

                let interval = ((sub_register.interval.0 + sub_offset),(sub_register.interval.0 + sub_offset + num_qubits));
                register_map.insert(name, Register::new(sub_register.state.clone(), interval));
            },
            TokenType::Apply => {                
                let gate = get_gate(token_iter.next(), &first_token, &operators)?;
                let register = get_register(token_iter.next(), &first_token, &register_map)?;

                if register.apply(gate.as_ref()).is_err() {
                    return Err(RuntimeError::new(Some(first_token), "Provided gate and register dimensions do not match.".to_owned()));
                }
            },
            TokenType::Identifier(operator_ident) => {
                //TENSOR MACRO
                if let Some(token) = token_iter.next() {
                    let operator = match token.ty {
                        TokenType::Tensor => {
                            let a = get_gate(token_iter.next(), &first_token, &operators)?;
                            let b = get_gate(token_iter.next(), &first_token, &operators)?;
                            a.tensor_product(b.as_ref())
                        },
                        TokenType::Concat => {
                            let a = get_gate(token_iter.next(), &first_token, &operators)?;
                            let b = get_gate(token_iter.next(), &first_token, &operators)?;

                            (a.as_ref() * b.as_ref()).map_err(|_| {
                                RuntimeError{token: Some(token), info: "Matrix multiplication requires first argument's column count matches second's row count. For".to_owned()}
                            })?
                        },
                        TokenType::Inverse => {
                            let a = get_gate(token_iter.next(), &first_token, &operators)?;
                            
                            a.as_ref().clone().adjoint()
                        },
                        _ => {
                            return Err(RuntimeError::new(Some(token), "Expected an operator macro (TENSOR, CONCAT, INVERSE), instead found".to_owned()));
                        },
                    };

                    operators.insert(operator_ident.clone(), Rc::new(operator));
                } else {
                    return Err(RuntimeError::new(Some(first_token), "Assumed operator macro decleration, found no defenition. For".to_owned()));
                }
            },
            TokenType::Measure => {
                let cheat = match token_iter.peek() {
                    Some(Token {ty: TokenType::Measure, ..}) => {
                        token_iter.next();
                        true
                    },
                    _ => false
                };

                let register = get_register(token_iter.next(), &first_token, &register_map)?;
                //let results = register.measure(cheat);
                results.push_str(&(match cheat {
                    true => register.measure().unwrap().to_string(), // todo add cheat back
                    false => register.measure().unwrap().to_string()  
                }));
            },
            _ => {}
        }

        let new_line = token_iter.next().unwrap();
        if let TokenType::NewLine = new_line.ty {} else {
            return Err(RuntimeError::new(Some(new_line), "Expected new line token".to_owned()));
        }
    }

    todo!();
}

#[derive(Debug)]
pub struct RuntimeError {
    token: Option<Token>,
    info: String
}

impl RuntimeError {
    pub fn new(token: Option<Token>, info: String) -> Self {
        Self {
            token,
            info
        }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.token.as_ref() {
            Some(token) => {
                f.write_fmt(format_args!("{} {:?} token, at {}:{}", self.info, token.ty, token.line(), token.col()))
            },
            None => {
                f.write_str(&self.info)
            }
        }
    }
}

impl Error for RuntimeError {}