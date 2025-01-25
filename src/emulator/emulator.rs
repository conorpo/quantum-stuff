use std::cell::Cell;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;
use std::rc::Rc;

use super::{
    lexer::{Token, TokenType},
    gates::PrimitiveGate
};

use crate::dynamic::vector::*;
use crate::dynamic::matrix::*;
use std::error::Error;

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

impl Error for RuntimeError {
}


fn parse_identifier(possible_token: Option<Token>, first_token: &Token, label: &'static str) -> Result<String, RuntimeError> {
    match possible_token {
        Some(Token { ty: TokenType::Identifier(name), ..}) => {Ok(name)},
        token if token.is_some() => { Err(RuntimeError::new(token, format!("Expected IDENTIFIER token for {label}, found"))) },
        None => { Err(RuntimeError::new(Some(first_token.clone()), format!("Missing {label} (IDENTIFER) for the"))) },
        _ => panic!("How did we get here?")
    }
}
type Register = (Cell<Option<Vector<f64>>>, usize);

struct SubRegister {
    pub register: Rc<Register>,
    pub interval: (usize, usize)
}

impl SubRegister {
    fn new(register: Rc<Register>, interval: (usize, usize)) -> Self {
        Self {
            register,
            interval
        }
    }

    fn len(&self) -> usize {
        self.interval.1 - self.interval.0
    }

    fn apply(&self, operator: &Matrix<f64>, apply_token: Token) -> Result<(), RuntimeError> {
        if !operator.is_square() || operator.dim().0 != 2usize.pow(self.len() as u32) {
            return Err(RuntimeError {token: Some(apply_token), info: "Provided matrix is not the right size for this (sub)register".to_owned()} )
        }

        let left_eye = Matrix::<f64>::eye(2usize.pow(self.interval.0 as u32));
        let right_eye = Matrix::<f64>::eye(2usize.pow(self.register.1 as u32 - self.interval.1 as u32));

        let full_op = left_eye.tensor_product(operator).tensor_product(&right_eye);

        let new_state = (&full_op * self.register.0.take().as_ref().unwrap()).unwrap();
        self.register.0.set(Some(new_state));
        Ok(())
    }

    fn measure_cheat(&self) -> String {
        let vec = self.register.0.take().unwrap();
        let mut res = String::new();

        for (state, prob) in vec.probabilities().iter().enumerate() {
            res.push_str(&format!("|{:b}>, p = {}", state, prob));
        }
        self.register.0.set(Some(vec));

        res
    }

    fn measure(&self) -> String {
        let rand: u32 = std::random::random();
        let sample = (rand as f64) / (u32::MAX as f64);

        let mut sum = 0.0;
        let vec = self.register.0.take().unwrap();
        let probs = vec.probabilities();
        let mut s = 0;
        while s < probs.len() {
            sum += probs[s];
            if sample < sum { break; }
        }
        let mut collapsed_state = Vector::deterministic(vec.dim(), s).unwrap();
        self.register.0.set(Some(collapsed_state));

        format!("{:b}", s)
    }
}


type SubregisterMap =  HashMap<String, SubRegister>;

fn get_subregister<'a>(possible_token: Option<Token>, first_token: &Token, subregisters: &'a SubregisterMap) -> Result<&'a SubRegister, RuntimeError> {
    let register_name = parse_identifier(possible_token.clone(), first_token, "register name argument")?;
    match subregisters.get(&register_name) {
        Some(entry) => Ok(entry),
        None => {
            return Err(RuntimeError::new(possible_token, "This register does not exist at this point,".to_owned()));
        }
    }
}


type OperatorMap = HashMap<String, Rc<Matrix<f64>>>;

fn get_operator<'a>(possible_token: Option<Token>, first_token: &Token, operators: &'a OperatorMap) -> Result<Rc<Matrix::<f64>>, RuntimeError> {
    match possible_token.as_ref() {
        Some(Token {ty: TokenType::Identifier(ident),..}) => {
            match operators.get(ident) {
                Some(operator) => {
                    Ok(operator.clone())
                },
                None => {
                    Err(RuntimeError::new(possible_token, "Operator does not exist at this point in the program, for".to_owned()))
                }
            }
        }, 
        Some(Token {ty: TokenType::Gate(primitive), ..}) => {
            Ok(Rc::new(primitive.get_operator()))
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

    let mut registers: Vec<Rc<Register>> = Vec::new();
    let mut subregisters: SubregisterMap = HashMap::new();
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

                let register = Vector::zero(2usize.pow(num_qubits as u32));
                registers.push(Rc::new((Cell::new(Some(register)), num_qubits)));
                
                let register_ref = registers.last().unwrap().clone();
                subregisters.insert(name, SubRegister::new(register_ref, (0,num_qubits)));
            },
            TokenType::Select => {
                let name_token = token_iter.next();
                let name = match name_token {
                    Some(Token { ty: TokenType::Identifier(name), ..}) => {name}
                    Some(token) => { return Err(RuntimeError::new(Some(token), "Expected IDENTIFER for slice name, found".to_owned()))},
                    None => { return Err(RuntimeError::new(Some(first_token), "Missing slice name argument (IDENTIFER) for the ".to_owned())); }
                };

                let sub_register = get_subregister(token_iter.next(), &first_token, &subregisters)?;

                let offset_token = token_iter.next();
                let sub_offset = match offset_token.as_ref() {
                    Some(Token {ty: TokenType::Number(offset), ..}) => {
                        if (0..sub_register.len()).contains(offset) {
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
                        if (1..=(sub_register.len() - sub_offset)).contains(num_qubits) {
                            *num_qubits
                        } else {
                            return Err(RuntimeError::new(num_qubits_token, format!("NUMQUBITS must be between 1 and {}, for ", sub_register.len() - sub_offset)));
                        }
                    },
                    Some(token) => { return Err(RuntimeError::new(num_qubits_token, "Expected NUMQUBITS argument (NUMBER), found".to_owned())); },
                    None => { return Err(RuntimeError::new(Some(first_token), "Missing NUMQUBITS argument (NUMBER) for the".to_owned())); }
                };

                let interval = ((sub_register.interval.0 + sub_offset),(sub_register.interval.0 + sub_offset + num_qubits));
                subregisters.insert(name, SubRegister::new(sub_register.register.clone(), interval));
            },
            TokenType::Apply => {                
                let operator = get_operator(token_iter.next(), &first_token, &operators)?;
                let subregister = get_subregister(token_iter.next(), &first_token, &subregisters)?;

                subregister.apply(operator.as_ref(), first_token)?;
            },
            TokenType::Identifier(operator_ident) => {
                //TENSOR MACRO
                if let Some(token) = token_iter.next() {
                    let operator = match token.ty {
                        TokenType::Tensor => {
                            let a = get_operator(token_iter.next(), &first_token, &operators)?;
                            let b = get_operator(token_iter.next(), &first_token, &operators)?;
                            a.tensor_product(b.as_ref())
                        },
                        TokenType::Concat => {
                            let a = get_operator(token_iter.next(), &first_token, &operators)?;
                            let b = get_operator(token_iter.next(), &first_token, &operators)?;

                            (a.as_ref() * b.as_ref()).map_err(|_| {
                                RuntimeError{token: Some(token), info: "Matrix multiplication requires first argument's column count matches second's row count. For".to_owned()}
                            })?
                        },
                        TokenType::Inverse => {
                            let a = get_operator(token_iter.next(), &first_token, &operators)?;
                            
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

                let register = get_subregister(token_iter.next(), &first_token, &subregisters)?;
                //let results = register.measure(cheat);
                results.push_str(&(match cheat {
                    true => register.measure_cheat(),
                    false => register.measure()   
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