mod codegen;
mod lowering;

use std::env;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::parser::{ParseError, parse_program};
use crate::source_loader::{IncludeError, load_entry_source};

pub use codegen::generate_rust_source;
pub use lowering::lower_program;

#[derive(Debug)]
pub enum CompileError {
    Include(IncludeError),
    Parse(ParseError),
    Io(io::Error),
    RustcFailed(String),
}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Include(err) => write!(f, "{err}"),
            Self::Parse(err) => write!(f, "{err}"),
            Self::Io(err) => write!(f, "{err}"),
            Self::RustcFailed(message) => write!(f, "rustc failed: {message}"),
        }
    }
}

impl std::error::Error for CompileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Include(err) => Some(err),
            Self::Parse(err) => Some(err),
            Self::Io(err) => Some(err),
            Self::RustcFailed(_) => None,
        }
    }
}

impl From<IncludeError> for CompileError {
    fn from(value: IncludeError) -> Self {
        Self::Include(value)
    }
}

impl From<ParseError> for CompileError {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

impl From<io::Error> for CompileError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

pub fn compile_entry_to_rust(input: &Path) -> Result<String, CompileError> {
    let source = load_entry_source(input)?;
    compile_resolved_source_to_rust(&source)
}

pub fn compile_resolved_source_to_rust(source: &str) -> Result<String, CompileError> {
    let program = parse_program(source)?;
    let program_ir = lower_program(&program);
    Ok(generate_rust_source(&program_ir))
}

pub fn compile_entry_to_binary(input: &Path, output: &Path) -> Result<(), CompileError> {
    let rust_source = compile_entry_to_rust(input)?;
    compile_rust_source(&rust_source, output)
}

pub fn compile_rust_source(rust_source: &str, output: &Path) -> Result<(), CompileError> {
    let source_path = write_generated_source(rust_source)?;
    let compile_result = compile_with_rustc(&source_path, output);
    let _ = fs::remove_file(&source_path);
    compile_result
}

fn write_generated_source(source: &str) -> io::Result<PathBuf> {
    let now_ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(io::Error::other)?
        .as_nanos();
    let file_name = format!("graphol_native_{}_{}.rs", std::process::id(), now_ns);
    let path = env::temp_dir().join(file_name);
    fs::write(&path, source)?;
    Ok(path)
}

fn compile_with_rustc(source_path: &Path, output: &Path) -> Result<(), CompileError> {
    let output_result = Command::new("rustc")
        .arg("--edition=2024")
        .arg(source_path)
        .arg("-o")
        .arg(output)
        .output()?;

    if output_result.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output_result.stderr);
    let message = if stderr.trim().is_empty() {
        "unknown rustc error".to_string()
    } else {
        stderr.trim().to_string()
    };

    Err(CompileError::RustcFailed(message))
}
