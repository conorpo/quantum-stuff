#![feature(random)]

use core::f64;
use std::fmt::{Display, UpperHex};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::LazyLock;
use std::thread;
use std::random::random;
use std::env;
use quantum_stuff::{dvec64, dynamic::*, complex::*, c64};
use display_bytes::*;

static BASIS: LazyLock<[State; 2]> = LazyLock::new(|| {
    let entry = 1.0 / f64::consts::SQRT_2;
    [State::try_from(dvec64![1;0]).unwrap(), State::try_from(dvec64![entry;entry]).unwrap()]
});

static PLUS_BASIS: LazyLock<[State; 2]> = LazyLock::new(|| {
    [State::try_from(dvec64![1;0]).unwrap(), State::try_from(dvec64![0;1]).unwrap()]
});

static CROSS_BASIS:  LazyLock<[State; 2]> = LazyLock::new(|| {
    let entry = 1.0 / f64::consts::SQRT_2;

    [State::try_from(dvec64![entry;entry]).unwrap(), State::try_from(dvec64![-entry;entry]).unwrap()]
});

fn alice(n: usize, senders: (Sender<State>, Sender<bool>), recievers: (Receiver<bool>, Receiver<(usize, u8)>)) -> impl FnOnce() -> (bool, Option<Vec<u8>>) {
    move || {
        let mut correct_bits = Vec::new();
        for _ in 0..n {
            let bit: u8 = random::<u8>() % 2;
            let qubit = BASIS[bit as usize].clone();
            senders.0.send(qubit).unwrap();
            let sure = recievers.0.recv().unwrap();
            if sure {
                correct_bits.push(bit);
            }
        }


        let mut correct = 0;
        let expected_check_bits = correct_bits.len() / 2;
        let mut revealed = vec![false; correct_bits.len()];
        for _ in 0..expected_check_bits {
            let bob_bit = recievers.1.recv().unwrap();
            correct += (correct_bits[bob_bit.0] == bob_bit.1) as i32;
            revealed[bob_bit.0] = true;
        }

        let agreed = (correct as f64) / (expected_check_bits as f64) > 0.95;
        senders.1.send(agreed).unwrap();

        let remaining_bits: Vec<u8> = correct_bits.into_iter().enumerate().filter(|(i,_)| !revealed[*i]).map(|(_,b)| b).collect();

        let mut bytes = vec![0; (remaining_bits.len() / 8) + 1];
        for (i, b) in remaining_bits.into_iter().enumerate() {
            bytes[i / 8] |= b << (i % 8);
        }

        (agreed, if agreed {Some(bytes)} else {None})
    }
}

fn bob(n: usize, senders: (Sender<bool>,Sender<(usize, u8)>), recievers: (Receiver<State>, Receiver<bool>)) -> impl FnOnce() -> (bool, Option<Vec<u8>>) {
    move || {
        let mut correct_bits = Vec::new();
        for _ in 0..n {
            let qubit = recievers.0.recv().unwrap();

            let basis: bool = random();
            let zero = if basis {&CROSS_BASIS[0]} else {&PLUS_BASIS[0]};
            let prob_of_0 = qubit.get().dot(zero.get()).unwrap().modulus_squared();
            
            let random_u64 = random::<u64>().min(u64::MAX - 1);
            let random_sample = (random_u64 as f64) / (u64::MAX as f64);
            
            let sure: bool = random_sample > prob_of_0;
            senders.0.send(sure).unwrap();

            if sure {
                correct_bits.push(!basis as u8);
            }
        }

        let mut indexed_bits: Vec<_> = correct_bits.into_iter().enumerate().map(|(i, b)| (random::<u32>(), i, b)).collect();
        indexed_bits.sort();

        let check_bits_n = indexed_bits.len() / 2;
        for bit_info in indexed_bits.iter().take(check_bits_n).map(|tup| (tup.1, tup.2)) {
            senders.1.send(bit_info).unwrap();
        }

        let agreed = recievers.1.recv().unwrap();

        let mut remaining_bits: Vec<_> = indexed_bits.into_iter().skip(check_bits_n).map(|(_,i,b)| (i,b)).collect();
        remaining_bits.sort();
    
        let mut bytes = vec![0; (remaining_bits.len() / 8) + 1];
        for (i, (_, b)) in remaining_bits.into_iter().enumerate() {
            bytes[i / 8] |= b << (i % 8);
        }

        (agreed, if agreed {Some(bytes)} else {None})
    }
}

struct AlgoResult {
    pub alice_key: Vec<u8>,
    pub bob_key: Vec<u8>
}

impl Display for AlgoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hex_formatter = display_bytes::FormatHex {
            prefix: "",
            separator: "",
            uppercase: true
        };
        
        f.write_str("Alice Result: ")?;
        hex_formatter.fmt_bytes(&self.alice_key[..], f)?;

        f.write_str("\nBob Result: ")?;
        hex_formatter.fmt_bytes(&self.bob_key[..], f)
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let n: usize = args.get(1).and_then(|n_string| n_string.parse().ok()).unwrap_or(1024);
    assert!(n >= 4);

    //Yes weird use of sender/reciever just wanted to do it this way..
    let (sender_1, reciever_1) = channel();
    let (sender_2, reciever_2) = channel();
    let (sender_3, reciever_3) = channel();
    let (sender_4, reciever_4) = channel();

    let alice = thread::spawn(alice(n, (sender_1, sender_3), (reciever_2, reciever_4)));
    let bob = thread::spawn(bob(n, (sender_2, sender_4), (reciever_1, reciever_3)));

    let (agreed_a, key_a) = alice.join().unwrap();
    let (agreed_b, key_b) = bob.join().unwrap();



    if agreed_a && agreed_b {
        println!("Keys matched!");
        let results = AlgoResult {
            alice_key: key_a.unwrap(),
            bob_key: key_b.unwrap()
        };
        println!("{}", results);
    } else {
        println!("Keys werent agreed on, maybe someone is eavesdropping.");
    }
}