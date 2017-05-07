extern crate argparse;
extern crate rand;

use argparse::{ArgumentParser, Store};
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;
use std::path::Path;
use rand::{thread_rng, Rng};

fn main() {
	let mut filepath: String = String::new();
    let mut n: u32 = 1;
	{
		let mut parser = ArgumentParser::new();
		parser.refer(&mut n).add_option(&["-n"], Store, "number of choice");
        parser.refer(&mut filepath).add_argument("file", Store, "source filepath");
		parser.parse_args_or_exit();
	}

    if filepath.len() > 0 {
        let items: Vec<String> = match read_from_file(filepath.as_str()) {
            Ok(items) => items, 
            Err(e) => {
                let _ = writeln!(&mut std::io::stderr(), "{}", e);
                return
            }
        };
        pickup(items, n);
    } else {
        let items: Vec<String> = match read_from_stdin() {
            Ok(items) => items, 
            Err(e) => {
                let _ = writeln!(&mut std::io::stderr(), "{}", e);
                return
            }
        };
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

fn pickup(items: Vec<String>, n: u32) {
    if items.len() == 0 {
        return;
    }
    let mut rng = thread_rng();
    for _ in 0..n {
        println!("{}", rng.choose(&items).unwrap());
    }
}