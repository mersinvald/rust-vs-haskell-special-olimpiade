
use std::io;
use std::io::{BufRead, BufWriter, Read, Write};

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
    let input = stdin.lock();
    let stdout = io::stdout();
    let mut output = stdout.lock();

    let lines: Vec<String> = input.lines()
        .map(Result::unwrap)
        .collect();

    let mut result: Vec<u8> = Vec::with_capacity(
        lines.len() * lines.len() * 5 * 2 // n^2 strings of 5*2 symbols
      + lines.len() * lines.len()         // \n
    );

    println!("{}", result.capacity());
    println!("{}", result.len());

    for pref in lines.iter().map(|line| line.split_at(5).0) {
        for suf in lines.iter().map(|line| line.split_at(6).1) {
            result.extend(pref.as_bytes());
            result.extend(suf.as_bytes());
            result.extend(b"\n");
        }
    }

    println!("{}", result.capacity());
    println!("{}", result.len());

 //   output.write_all(&result).unwrap();
}
