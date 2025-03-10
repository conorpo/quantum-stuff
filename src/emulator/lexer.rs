use std::{collections::HashMap, io::{BufReader, Bytes, Read}};
use std::io::BufRead;

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

#[derive(Clone, Copy, Debug)]
pub enum PrimitiveGate {
    H,
    CNOT,
    R(f64),
    I(usize)
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
        for word in line.split_whitespace() {
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
            } else if word.len() >= 1 && word.chars().all(|c| c.is_numeric()) {
                TokenType::Number(word.parse().unwrap())
            } else if word.len() >= 1 {
                TokenType::Identifier(word.to_owned())
            } else {
                panic!("Unexpected whitespace.");
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