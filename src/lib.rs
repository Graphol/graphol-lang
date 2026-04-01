pub mod ast;
pub mod compiler;
pub mod ir;
pub mod parser;
pub mod source_loader;

pub use compiler::{
    CompileError, compile_entry_to_binary, compile_entry_to_rust, compile_resolved_source_to_rust,
    compile_rust_source, generate_rust_source, lower_program,
};
