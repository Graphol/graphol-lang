use super::io::OutputMode;
use super::object::BlockSnapshot;

pub trait ExecutionHost {
    fn read_input(&mut self, prompt: &str) -> String;
    fn emit_output(&mut self, mode: OutputMode, value: &str);
    fn call_block(&mut self, block: BlockSnapshot);
}
