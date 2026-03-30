use std::collections::VecDeque;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputMode {
    Alert,
    Console,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputEvent {
    pub mode: OutputMode,
    pub value: String,
}

pub trait RuntimeIo {
    fn read_input(&mut self, prompt: &str) -> String;
    fn on_output(&mut self, mode: OutputMode, value: &str);
}

pub struct StdIo;

impl RuntimeIo for StdIo {
    fn read_input(&mut self, prompt: &str) -> String {
        print!("{} ", prompt);
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return String::new();
        }
        input.trim_end_matches(['\n', '\r']).to_string()
    }

    fn on_output(&mut self, _mode: OutputMode, value: &str) {
        println!("{}", value);
    }
}

pub struct TestIo {
    pub inputs: VecDeque<String>,
    pub emitted: Vec<OutputEvent>,
}

impl TestIo {
    pub fn new(inputs: Vec<String>) -> Self {
        Self {
            inputs: inputs.into(),
            emitted: Vec::new(),
        }
    }
}

impl RuntimeIo for TestIo {
    fn read_input(&mut self, _prompt: &str) -> String {
        self.inputs.pop_front().unwrap_or_default()
    }

    fn on_output(&mut self, mode: OutputMode, value: &str) {
        self.emitted.push(OutputEvent {
            mode,
            value: value.to_string(),
        });
    }
}
