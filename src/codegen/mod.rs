use std::collections::HashMap;

use crate::StackFrame;

use crate::{Type, Variable, Ref, Children};
use asm::{Flag, Instruction};

pub mod asm;
pub mod interpreter;

pub const INSTRUCTION_SIZE: usize = 4;

pub struct Assembler {
    statics: HashMap<Variable, (Type, u64)>,
    blocks: HashMap<Ref<Block>, Block>,
    stackframes: HashMap<Ref<StackFrame>, StackFrame>,
    init: Ref<Block>,
}

pub struct Block {
    instructions: Vec<Ir>,
    stackframe: Ref<StackFrame>,
}

pub enum Ir {
    Instruction(Instruction),
    Call(Flag, Ref<Block>),
    CallNot(Flag, Ref<Block>),
}

pub trait Compilable {
    fn compile(&mut self, children: &mut Children, assembler: &mut Assembler, stackframe: Ref<StackFrame>) -> Ref<Block>;
}
