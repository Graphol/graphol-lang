pub mod ast;
pub mod parser;
pub mod runtime;

use parser::{ParseError, parse_program};
use runtime::{OutputEvent, RuntimeIo, Vm, VmError};

pub fn run_graphol(source: &str, io: Box<dyn RuntimeIo>) -> Result<Vec<OutputEvent>, GrapholError> {
    let program = parse_program(source)?;
    let mut vm = Vm::new(program, io);
    vm.run()?;
    Ok(vm.outputs().to_vec())
}

#[derive(Debug)]
pub enum GrapholError {
    Parse(ParseError),
    Vm(VmError),
}

impl From<ParseError> for GrapholError {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

impl From<VmError> for GrapholError {
    fn from(value: VmError) -> Self {
        Self::Vm(value)
    }
}
