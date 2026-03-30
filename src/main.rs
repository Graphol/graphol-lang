use std::env;
use std::fs;
use std::io::{self, Read};

use graphol_rs::parser::parse_program;
use graphol_rs::runtime::{StdIo, Vm};

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let source = if let Some(path) = env::args().nth(1) {
        fs::read_to_string(path)?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    let program = parse_program(&source)?;
    let mut vm = Vm::new(program, Box::new(StdIo));
    vm.run()?;
    Ok(())
}
