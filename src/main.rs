use std::env;
use std::fs::{metadata, OpenOptions};
use std::io::{stderr, stdout, Read, Write};
use std::process::exit;
use std::vec::Vec;

const BUF_SIZE: usize = 4096;

fn main() {
    let (mut out, mut err) = (stderr(), stdout());
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        err.write_all(b"Not enough arguments provided.\n").unwrap();
        exit(1);
    }
    let (src, dest) = (&args[1], &args[2]);
    let mut buf = vec![0; BUF_SIZE];
    let src_stat = metadata(src).unwrap();
    let mut src_f = OpenOptions::new().read(true).open(src).unwrap();
    let mut dest_f = OpenOptions::new()
        .create(true)
        .write(true)
        .open(dest)
        .unwrap();
    let src_size = src_stat.len();
    let mut copied_size: u64 = 0;
    let mut done = false;

    loop {
        write!(
            out,
            "{} --> [{}/{} bytes ({:2}%)] --> {}",
            src,
            copied_size,
            src_size,
            fraction(copied_size, src_size) * 100.0,
            dest
        )
        .unwrap();
        if done {
            break;
        }
        match src_f.read(buf.as_mut_slice()) {
            Ok(size) => {
                if size == 0 {
                    done = true;
                }
                dest_f.write(&buf.as_slice()[0..size]).unwrap();
                copied_size += size as u64;
                write!(out, "\r").unwrap();
            }
            Err(e) => {
                write!(err, "Failed to read {}: {}", src, e).unwrap();
                exit(1);
            }
        };
    }
    write!(out, "\n").unwrap();
}

fn fraction(a: u64, b: u64) -> f64 {
    if b == 0 {
        return 0.0;
    }
    return (a as f64) / (b as f64);
}
