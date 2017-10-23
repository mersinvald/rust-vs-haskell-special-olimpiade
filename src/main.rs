#![feature(core_intrinsics)]
use std::io;
use std::io::{Write, Read};

use std::time::Instant;

fn clock<R, F: FnMut() -> R>(msg: &str, mut f: F) -> R {
    // Start timer
    let start = Instant::now();
    
    // Run the measured code
    let r = f();

    // Pretty print timing
    let elapsed = start.elapsed();
    let secs = elapsed.as_secs();
    let millis = elapsed.subsec_nanos() / 1000_000; 
    let msg = format!("{}: {}.{}\n", msg, secs, millis);
    io::stderr().write(msg.as_bytes()).unwrap();

    // Return the runned code return value
    r
}

fn main() {
    // Lock streams to not to spend time on that further
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let stdout = io::stdout();
    let mut output = stdout.lock();
    
    // Read input from stdn
    let mut buffer = String::new();
    clock("read", || {
        input.read_to_string(&mut buffer).unwrap();
    });

    // Tokenize preffixes and suffixes
    let tokens: Vec<(&str, &str)> = clock("tokenize", || {
        buffer.split("\r\n")
        .map(|line| {
            let mut words = line.split_whitespace();
            (
                words.next().unwrap(),
                words.next().unwrap()
            )
        })
        .collect()
    });

    // Generate and output token combinations
    clock("main", || {
        // The on-stack buffer
        let mut buffer = Buffer::new();

        for &(pref, _) in &tokens {
            for &(_, suf) in &tokens {
                buffer.extend_or_flush(pref.as_bytes(), &mut output);
                buffer.extend_or_flush(suf.as_bytes(), &mut output);
                buffer.extend_or_flush(b"\n", &mut output);
            }
        }

        // Flush in case there is some data in buffer
        buffer.flush(&mut output);
    });
}

// 200kb on stack should be OK
const BUFFER_CAPACITY: usize = 1024 * 200;

struct Buffer {
    buf: [u8; BUFFER_CAPACITY],
    len: usize,
}

// Cheating :D (-30ms)
use std::intrinsics::unlikely;

impl Buffer {
    fn new() -> Self {
        Buffer { 
            buf: [0; BUFFER_CAPACITY], 
            len: 0
        }
    }

    fn extend_or_flush<W: Write>(&mut self, slice: &[u8], out: &mut W) {
        if unsafe { unlikely(self.len + slice.len() > BUFFER_CAPACITY) } {
            self.flush(out);
        }
        let new_len = self.len + slice.len();
        self.buf[self.len..new_len].copy_from_slice(slice);
        self.len = new_len;
    }

    fn flush<W: Write>(&mut self, out: &mut W) {
        out.write_all(&self.buf[..self.len]).unwrap();
        self.len = 0;
    }
}