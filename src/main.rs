
use std::io;
use std::io::{Write, Read};

/* 
#! /usr/bin/env pypy

import sys

w = [l.split(' ', 1) for l in sys.stdin]
pref = [x[0] for x in w]
suff = [x[1].rstrip() for x in w]

for p in pref:
    print "\n".join([(p + s) for s in suff])
*/

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let stdout = io::stdout();
    let mut output = stdout.lock();

    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let tokens: Vec<(&str, &str)> = buffer.split("\r\n")
        .map(|line| {
            let mut words = line.split_whitespace();
            (
                words.next().unwrap(),
                words.next().unwrap()
            )
        })
        .collect();

    let mut result: Vec<u8> = Vec::with_capacity(
        tokens.len() * tokens.len() * 5 * 2 // n^2 strings of 5*2 symbols
      + tokens.len() * tokens.len()         // \n
    );

    for &(pref, _) in &tokens {
        for &(_, suf) in &tokens {
            // if buffer capacity will be exceeded in this iteration,
            // flush the buffer
            if result.capacity() - result.len() < pref.len() + suf.len() + 1 {
                output.write_all(&result).unwrap();
                result.clear();
            }

            result.extend(pref.as_bytes());
            result.extend(suf.as_bytes());
            result.extend(b"\n");
        }
    }

    if !result.is_empty() {
        output.write_all(&result).unwrap();
    }
}
