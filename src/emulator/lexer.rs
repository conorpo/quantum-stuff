use std::{collections::HashMap, io::{BufReader, Bytes, Read}};
use std::io::BufRead;
use crate::matrix::*;
use crate::complex::*;

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pos: (usize, usize) 
}

impl Token {
    pub fn line(&self) -> usize {
        self.pos.0
    }

    pub fn col(&self) -> usize {
        self.pos.1
    }
}

#[derive(Clone, Debug)]
pub enum TokenType {
    Identifier(String),
    Number(usize),
    ByteArray(Vec<bool>),
    Gate(PrimitiveGate),
    Initialize,
    Select,
    Apply,
    Concat,
    Tensor,
    Inverse,
    Measure,
    NewLine,
}
#[derive(Copy, Clone, Debug)]
pub enum PrimitiveGate {
    H,
    R(f64),
    I(usize),
    CNOT
}

impl PrimitiveGate {
    fn get_operator(self) -> Box<dyn MatrixT> {
        match self {
            PrimitiveGate::H => {
                let h_element: f64 = 1.0 / 2.0f64.sqrt();
                Box::new(
                    Matrix::<2,2,f64>::new([[Complex::new(h_element, 0.0), Complex::new(h_element, 0.0)],
                                            [Complex::new(h_element, 0.0), Complex::new(-h_element, 0.0)]])
                )
            },
            PrimitiveGate::R(theta) => {
                Box::new(
                    Matrix::<2,2,f64>::new([[Complex::one(), Complex::zero()],
                                            [Complex::zero(), Complex::new(f64::exp(theta), 0.0)]])
                )
            },
            PrimitiveGate::I(n) => {
                match n {
                    2 => Box::new(Matrix::<2,2,f64>::eye()),
                    4 => Box::new(Matrix::<4,4,f64>::eye()),
                    8 => Box::new(Matrix::<8,8,f64>::eye()),
                    16 => Box::new(Matrix::<16,16,f64>::eye()),
                    32 => Box::new(Matrix::<32,32,f64>::eye()),
                    64 => Box::new(Matrix::<64,64,f64>::eye()),
                    128 => Box::new(Matrix::<128,128,f64>::eye()),
                    256 => Box::new(Matrix::<256,256,f64>::eye()),
                    _ => { panic!("How did we get here?"); }
                }
            },
            PrimitiveGate::CNOT => {
                Box::new(
                    mat64![[1,0,0,0],
                            [0,1,0,0],
                            [0,0,0,1],
                            [0,0,1,0]]
                )
            }
        }
    } 
}

pub fn scan(stream: &mut impl BufRead) -> Result<Vec<Token>, &'static str>{
    let mut output: Vec<Token> = Vec::new();

    let keywords = HashMap::from([("INITIALIZE", TokenType::Initialize), 
                                 ("SELECT", TokenType::Select),
                                 ("APPLY", TokenType::Apply),
                                 ("MEASURE", TokenType::Measure),
                                 ("TENSOR", TokenType::Tensor),
                                 ("CONCAT", TokenType::Concat),
                                 ("INVERSE", TokenType::Inverse)]);

    

    for (line_number, line) in stream.lines().enumerate() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }

        let mut col = 0;
        for word in line.split(' ') {
            let token_type = if let Some(token_type) = keywords.get(word) {
                token_type.clone()
            } else if let Some(byte_array) =  word.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                let mut bits = Vec::new();
                for &b in byte_array.as_bytes() {
                    if b == b'0' {
                        bits.push(false);
                    } else if b == b'1' {
                        bits.push(true);
                    } else {
                        return Err("Test");
                    }
                }

                TokenType::ByteArray(bits)
            } else if word == "H" {
                TokenType::Gate(
                    PrimitiveGate::H
                )
            } else if word == "CNOT" {
                TokenType::Gate(
                    PrimitiveGate::CNOT
                )
            } else if let Some(theta) = word.strip_prefix("R(").and_then(|s| s.strip_suffix(")")) {
                TokenType::Gate(
                    PrimitiveGate::R(theta.parse().map_err(|_| "Failed to Parse Theta")?)
                )
            } else if let Some(n) = word.strip_prefix("I(").and_then(|s| s.strip_suffix(")")) {
                TokenType::Gate(
                    PrimitiveGate::I(n.parse().map_err(|_| "Failed to passe n")?)
                )
            } else if word.chars().all(|c| c.is_numeric()) {
                TokenType::Number(word.parse().unwrap())
            } else {
                TokenType::Identifier(word.to_owned())
            };

            output.push(Token {
                ty: token_type,
                pos: (line_number, col)
            });

            col += word.len() + 1;
        }

        output.push(Token {
            ty: TokenType::NewLine,
            pos: (line_number, col - 1)
        })
    }

    Ok(output)
}

#[cfg(test)]
pub mod tests {
    use std::io::BufReader;

    use super::scan;

    #[test]
    fn debug_test() {
        let mut program = "
INITIALIZE R 2
U TENSOR H H
APPLY U R
MEASURE R RES".as_bytes();
        
        let tokens = scan(&mut program).unwrap();

        dbg!(&tokens);
    }

    #[test]
    fn debug_test_2() {
        let mut program = "INITIALIZE R 2
U TENSOR H I(2)
APPLY U R
SELECT S1 R 0 1
MEASURE S1 RES
APPLY CNOT R
MEASURE R RES".as_bytes();

        let tokens = scan(&mut program).unwrap();
        dbg!(&tokens);
    }
}