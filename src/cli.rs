use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};

use graphol::compile_entry_to_binary;

#[derive(Debug, Default)]
pub struct CliOptions {
    pub input: PathBuf,
    pub output: PathBuf,
}

pub fn parse_cli_args(args: impl IntoIterator<Item = OsString>) -> io::Result<CliOptions> {
    let mut input: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        if arg == "-o" || arg == "--output" {
            let value = args.next().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "missing value after -o/--output",
                )
            })?;
            output = Some(PathBuf::from(value));
            continue;
        }

        if let Some(value) = arg.to_string_lossy().strip_prefix("-o=") {
            output = Some(PathBuf::from(value));
            continue;
        }

        if let Some(value) = arg.to_string_lossy().strip_prefix("--output=") {
            output = Some(PathBuf::from(value));
            continue;
        }

        if arg.to_string_lossy().starts_with('-') {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unknown option: {}", arg.to_string_lossy()),
            ));
        }

        if input.is_some() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "multiple input files provided",
            ));
        }
        input = Some(PathBuf::from(arg));
    }

    let input = input.ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "missing input .graphol file")
    })?;
    let output = match output {
        Some(path) => path,
        None => derive_default_output_name(&input)?,
    };

    Ok(CliOptions { input, output })
}

fn derive_default_output_name(input: &Path) -> io::Result<PathBuf> {
    if input.is_dir() {
        return Ok(PathBuf::from("main"));
    }

    input
        .file_stem()
        .filter(|stem| !stem.is_empty())
        .map(PathBuf::from)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "could not derive output name from input path '{}'",
                    input.display()
                ),
            )
        })
}

pub fn compile_file(input: &Path, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    compile_entry_to_binary(input, output)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_cli_args;
    use std::ffi::OsString;
    use std::path::PathBuf;

    #[test]
    fn parses_compile_arguments() {
        let options = parse_cli_args([
            OsString::from("examples/program5.graphol"),
            OsString::from("-o"),
            OsString::from("program5"),
        ])
        .expect("args should be valid");

        assert_eq!(options.input, PathBuf::from("examples/program5.graphol"));
        assert_eq!(options.output, PathBuf::from("program5"));
    }

    #[test]
    fn output_option_requires_input() {
        let error = parse_cli_args([OsString::from("-o"), OsString::from("program5")])
            .expect_err("missing input should fail");
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    }

    #[test]
    fn derives_output_name_from_graphol_file_when_missing_output_option() {
        let options = parse_cli_args([OsString::from("examples/program5.graphol")])
            .expect("input-only args should be valid");

        assert_eq!(options.input, PathBuf::from("examples/program5.graphol"));
        assert_eq!(options.output, PathBuf::from("program5"));
    }

    #[test]
    fn derives_main_output_name_for_directory_input_when_missing_output_option() {
        let options = parse_cli_args([OsString::from("examples/")])
            .expect("directory input should derive main output");

        assert_eq!(options.input, PathBuf::from("examples/"));
        assert_eq!(options.output, PathBuf::from("main"));
    }
}
