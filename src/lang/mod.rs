mod instruction;
mod tokenizer;
pub use instruction::Instruction;

pub fn read_line(s: &str) -> Instruction {
    let (instruction, args) = tokenizer::tokenize_line(s);
    Instruction::parse(instruction.as_str(), args)
}
