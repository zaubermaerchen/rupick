extern crate argparse;
extern crate rand;

use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::cmp;
use rand::{thread_rng, Rng};
use argparse::{ArgumentParser, Store, StoreTrue};

fn main() {
    let mut filepath: String = String::new();
    let mut n: usize = 1;
    let mut deny_duplicates: bool = false;
    {
        let mut parser = ArgumentParser::new();
        parser.refer(&mut n).add_option(&["-n"], Store, "number of choice");
        parser.refer(&mut deny_duplicates).add_option(&["-d"], StoreTrue, "deny duplicates");
        parser.refer(&mut filepath).add_argument("file", Store, "source filepath");
        parser.parse_args_or_exit();
    }

    let items = match if filepath.len() > 0 { read_from_file(filepath.as_str()) } 
                        else { read_from_stdin() } {
        Ok(items) => items, 
        Err(e) => panic!(e.to_string())
    };

    if deny_duplicates {
        pickup_deny_duplicates(items, n);
    } else {
        pickup(items, n);
    }
}

fn read_from_stdin() -> io::Result<Vec<String>> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut items : Vec<String> = Vec::new();
    let mut buffer = String::new();
    while reader.read_line(&mut buffer)? > 0 {
        items.push(buffer.trim().to_string());
        buffer.clear();
    }
    return Ok(items);
}

fn read_from_file(filepath: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filepath);
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);

    let mut items : Vec<String> = Vec::new();
    let mut buffer = String::new();
    while reader.read_line(&mut buffer)? > 0 {
        items.push(buffer.trim().to_string());
        buffer.clear();
    }
    return Ok(items);
}

fn pickup(items: Vec<String>, n: usize) {
    if items.len() == 0 || n == 0 {
        return;
    }
    let mut rng = thread_rng();
    for _ in 0..n {
        println!("{}", rng.choose(&items).unwrap());
    }
}

fn pickup_deny_duplicates(items: Vec<String>, n: usize) {
    if items.len() == 0 || n == 0 {
        return;
    }
    let n = cmp::min(n, items.len());
    if n == 1 {
        pickup(items, n);
        return;
    }
    let mut rng = thread_rng();
    let mut indexes: Vec<usize> = (0..items.len()).collect();
    rng.shuffle(&mut indexes);
    for _ in 0..n {
        println!("{}", items[indexes.pop().unwrap()]);
    }
}