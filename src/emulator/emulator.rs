use std::cell::Cell;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;
use std::rc::Rc;

use crate::lexer::{Token, TokenType, PrimitiveGate};
use crate::{complex::*, operator};
use crate::vector::*;
use crate::matrix::*;
use crate::operator::*;
use std::error::Error;

#[derive(Debug)]
struct RuntimeError {
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

struct Register {
    data: Cell<Box<dyn VectorT>>,
    num_qubits: usize
}

struct SubRegister<'a> {
    pub register: &'a Register,
    pub interval: Range<usize>
}

impl Register {
    fn new(register_box: Box<dyn VectorT>, num_qubits: usize) -> Self {
        Self {
            data: Cell::new(register_box),
            num_qubits
        }
    }

    fn apply(&self) {
        // Apply some matrix
    }
}

impl<'a> SubRegister<'a> {
    fn new(register: &'a Register, interval: Range<usize>) -> Self {
        Self {
            register,
            interval
        }
    }
}

type SubregisterMap<'a> =  HashMap<String, SubRegister<'a>>;

fn get_subregister<'a>(possible_token: Option<Token>, first_token: &Token, subregisters: &'a SubregisterMap) -> Result<&'a SubRegister<'a>, RuntimeError> {
    let register_name = parse_identifier(possible_token.clone(), first_token, "register name argument")?;
    match subregisters.get(&register_name) {
        Some(entry) => Ok(entry),
        None => {
            return Err(RuntimeError::new(possible_token, "This register does not exist at this point,".to_owned()));
        }
    }
}

#[derive(Clone)]
enum Operator {
    Tensor(Rc<Operator>, Rc<Operator>),
    Concat(Rc<Operator>, Rc<Operator>),
    Inverse(Rc<Operator>),
    Gate(PrimitiveGate)
}

type OperatorMap = HashMap<String, Rc<Operator>>;

fn parse_operator<'a>(possible_token: Option<Token>, first_token: &Token, operators: &'a OperatorMap) -> Result<Rc<Operator>, RuntimeError> {
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
            Ok(Rc::new(Operator::Gate(primitive.clone())))
        },
        Some(_) => {
            Err(RuntimeError::new(possible_token, "Expected operator identifier OR gate primitive, found".to_owned()))
        },
        None => {
            Err(RuntimeError::new(Some(first_token.clone()), "Missing operator agument (IDENTIFIER | PRIMITIVE_GATE) for the".to_owned()))
        }
    }
}

fn construct_operator(representation: Operator) -> Box<dyn MatrixT> {
    let x = construct_operator(representation);
    x.downcast_ref()
}

//This is going to be cancer with const generic Vector / Matrix types.
pub fn emulate(tokens: Vec<Token>) -> Result<(), RuntimeError> {
    let mut registers: Vec<Register> = Vec::new();
    let mut subregisters: SubregisterMap = HashMap::new();
    let mut operators: OperatorMap = HashMap::new();

    let mut token_iter = tokens.into_iter().peekable();
    while token_iter.peek().is_some() {
        let mut line_iter = token_iter.take_while(|token| if let TokenType::NewLine = token.ty {false} else {true});
        
        let first_token = match token_iter.next() {
            Some(token) => token,
            None => { continue; }
        };

        match first_token.ty {
            TokenType::Initialize => {
                let name = parse_identifier(line_iter.next(), &first_token, "register name argument")?;
                
                let num_qubits_token = line_iter.next();
                let num_qubits = match num_qubits_token.as_ref() {
                    Some(Token { ty: TokenType::Number(num_qubits), ..})  if (1..=8).contains(num_qubits) => {*num_qubits},
                    Some(_) => { return Err(RuntimeError::new(num_qubits_token, "Expected NUMBER within 1-8 (inclusive) for register qubit num_qubits, found".to_owned())); },
                    None => { return Err(RuntimeError::new(Some(first_token), "Missing register num_qubits argument (NUMBER) for the".to_owned())); }
                };

                let register: Box<dyn VectorT> = match num_qubits {
                    1 => Box::new(Vector::<2,f64>::zero()),
                    2 => Box::new(Vector::<4,f64>::zero()),
                    3 => Box::new(Vector::<8,f64>::zero()),
                    4 => Box::new(Vector::<16,f64>::zero()),
                    5 => Box::new(Vector::<32,f64>::zero()),
                    6 => Box::new(Vector::<64,f64>::zero()),
                    7 => Box::new(Vector::<128,f64>::zero()),
                    8 => Box::new(Vector::<256,f64>::zero()),
                    _ => { panic!("How did we get here?") }
                };
                
                registers.push(Register::new(register, num_qubits));
                let register_ref = registers.last().unwrap()
                subregisters.insert(name, SubRegister::new(register_ref, 0..num_qubits));
            },
            TokenType::Select => {
                let name_token = line_iter.next();
                let name = match name_token {
                    Some(Token { ty: TokenType::Identifier(name), ..}) => {name}
                    Some(token) => { return Err(RuntimeError::new(Some(token), "Expected IDENTIFER for slice name, found".to_owned()))},
                    None => { return Err(RuntimeError::new(Some(first_token), "Missing slice name argument (IDENTIFER) for the ".to_owned())); }
                };

                let &SubRegister {register, interval} = get_subregister(line_iter.next(), &first_token, &subregisters)?;

                let offset_token = line_iter.next();
                let sub_offset = match offset_token.as_ref() {
                    Some(Token {ty: TokenType::Number(offset), ..}) => {
                        if (0..interval.len()).contains(offset) {
                            *offset
                        } else {
                            return Err(RuntimeError::new(offset_token, format!("Offset outside of (sub)register bounds (0..{})", register_size)));
                        }
                    },
                    Some(token) => { return Err(RuntimeError::new(offset_token, "Expected offset argument (NUMBER), found".to_owned())); },
                    None => { return Err(RuntimeError::new(Some(first_token), "Missing offset argument (NUMBER) for the".to_owned())); }
                };

                let num_qubits_token = line_iter.next();
                let num_qubits = match num_qubits_token.as_ref() {
                    Some(Token {ty: TokenType::Number(num_qubits), ..}) => {
                        if (1..=(interval.len() - sub_offset)).contains(num_qubits) {
                            *num_qubits
                        } else {
                            return Err(RuntimeError::new(num_qubits_token, format!("NUMQUBITS must be between 1 and {}, for ", register_size - offset)));
                        }
                    },
                    Some(token) => { return Err(RuntimeError::new(num_qubits_token, "Expected NUMQUBITS argument (NUMBER), found".to_owned())); },
                    None => { return Err(RuntimeError::new(Some(first_token), "Missing NUMQUBITS argument (NUMBER) for the".to_owned())); }
                };

                let interval = (interval.start + sub_offset)..(interval.start + sub_offset + num_qubits);
                subregisters.insert(name, SubRegister::new(register, interval));
            },
            TokenType::Apply => {                
                let operator_representation = parse_operator(line_iter.next(), &first_token, &operators)?;
                let subregister = get_subregister(line_iter.next(), &first_token, &subregisters)?;

                //let operator = construct_operator(operator_representation);
                //subregister.apply(&operator);
            },
            TokenType::Identifier(operator_ident) => {
                //TENSOR MACRO
                if let Some(token) = line_iter.next() {
                    let operator = match token.ty {
                        TokenType::Tensor => {
                            let a = parse_operator(line_iter.next(), &first_token, &operators)?;
                            let b = parse_operator(line_iter.next(), &first_token, &operators)?;
                            Operator::Tensor(a,b)
                        },
                        TokenType::Concat => {
                            let a = parse_operator(line_iter.next(), &first_token, &operators)?;
                            let b = parse_operator(line_iter.next(), &first_token, &operators)?;
                            Operator::Concat(a, b)
                        },
                        TokenType::Inverse => {
                            let a = parse_operator(line_iter.next(), &first_token, &operators)?;
                            Operator::Inverse(a)
                        },
                        _ => {
                            return Err(RuntimeError::new(Some(token), "Expected an operator macro (TENSOR, CONCAT, INVERSE), instead found".to_owned()));
                        },
                    };

                    operators.insert(operator_ident, Rc::new(operator));
                } else {
                    return Err(RuntimeError::new(Some(first_token), "Assumed operator macro decleration, found no defenition. For".to_owned()));
                }
            }
            _ => {}
        }
    }

    todo!();
}