contract;
use std::{assert::assert, math::*};
use std::logging::log;

abi Tiny_RSA {
    fn key_gen() -> (u64, u64);
    fn encrypt(m: u64) -> u64;
    fn decrypt(c: u64) -> u64;
}

impl Tiny_RSA for Contract {

    // Has absolutely no other use than testing the tuple type
    fn key_gen() -> (u64, u64) {
        (1,1)
    }

    // p = 7, q = 19
    fn encrypt(m: u64) -> u64 {
        let n = 133;
        let pub_key = 5;
        pow_mod(m, pub_key, n)
    }

    // p = 7, q = 19
    fn decrypt(c: u64) -> u64 {
        let n = 133;
        let priv_key: u64 = 65;
        pow_mod(c, priv_key, n)
    }
}

fn pow_mod(a: u64, b: u64, n: u64) -> u64 {
        let mut i: u64 = 1;
        let mut res: u64 = a;
        while i < b {
            res = (res * a) % n;
            i = i + 1; 
        }
        res
}