use super::asm::*;
use super::INSTRUCTION_SIZE;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryNode {
    None,
    Data(u32),
    Instruction(Instruction),
    FilledByInstruction,
}

trait Executable {
    fn execute(&self, interpreter: &mut Interpreter) -> bool;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Interpreter {
    pub memory: BTreeMap<u32, MemoryNode>,
    pub registers: BTreeMap<Register, u32>,
    pub flags: BTreeSet<Flag>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            memory: BTreeMap::new(),
            registers: BTreeMap::new(),
            flags: BTreeSet::new(),
        }
    }

    pub fn load(&mut self, data: Vec<u32>, instructions: Vec<Instruction>, start: u32) {
        let mut adress = 0;
        for value in data {
            self.memory.insert(adress, MemoryNode::Data(value));
            adress += 1;
        }
        for instruction in instructions {
            self.memory
                .insert(adress, MemoryNode::Instruction(instruction));
            for i in 1..INSTRUCTION_SIZE as u32 {
                self.memory
                    .insert(adress + i, MemoryNode::FilledByInstruction);
            }
            adress += INSTRUCTION_SIZE as u32;
        }
        self.registers.insert(Register::ProgramCounter, start);
        self.registers.insert(Register::StackPointer, 1000);
        self.registers.insert(Register::ProgramPointer, 0);
    }

    pub fn get(&mut self, adress: u32) -> &mut MemoryNode {
        if let std::collections::btree_map::Entry::Vacant(e) = self.memory.entry(adress) {
            e.insert(MemoryNode::None);
            self.memory.get_mut(&adress).unwrap()
        } else {
            self.memory.get_mut(&adress).unwrap()
        }
    }

    pub fn get_register(&mut self, register: &Register) -> &mut u32 {
        if self.registers.contains_key(register) {
            self.registers.get_mut(register).unwrap()
        } else {
            self.registers.insert(*register, 0);
            self.registers.get_mut(register).unwrap()
        }
    }

    pub fn set_register(&mut self, register: Register, value: u32) {
        self.registers.insert(register, value);
    }

    pub fn set_flag(&mut self, flag: Flag) {
        self.flags.insert(flag);
    }

    pub fn clear_flag(&mut self, flag: Flag) {
        self.flags.remove(&flag);
    }

    pub fn tick(&mut self) -> Result<(), String> {
        self.set_flag(Flag::Always);
        let pc = *self.get_register(&Register::ProgramCounter);
        let instruction = *match self.get(pc) {
            MemoryNode::Instruction(instruction) => instruction,
            _ => return Err("Invalid instruction".to_string()),
        };
        self.registers
            .insert(Register::ProgramCounter, pc + INSTRUCTION_SIZE as u32);
        match instruction.execute(self) {
            true => Ok(()),
            false => Err("Halt".to_string()),
        }
    }

    pub fn execute(&mut self) -> Result<(), String> {
        loop {
            match self.tick() {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
    }

    pub fn debug(&self) -> String {
        let mut output = String::new();
        for (register, value) in &self.registers {
            output.push_str(&format!("{:?}: {}\n", register, value));
        }
        for (adress, node) in &self.memory {
            if let MemoryNode::Instruction(instruction) = node {
                output.push_str(&format!("{}: {:?}\n", adress, String::from(*instruction)));
            } else if let MemoryNode::Data(value) = node {
                output.push_str(&format!("{}: {:?}\n", adress, value));
            }
        }
        for flag in &self.flags {
            output.push_str(&format!("{:?}\n", flag));
        }
        output
    }
}

impl Operand {
    pub fn aquire(&self, interpreter: &mut Interpreter) -> u32 {
        match self {
            Operand::U32(value) => *value,
            Operand::Register(register) => *interpreter.get_register(register),
        }
    }
}

impl Value {
    pub fn aquire(&self, interpreter: &mut Interpreter) -> u32 {
        match self {
            Value::Direct(operand) => operand.aquire(interpreter),
            Value::Indirect(operand) => {
                let adress = operand.aquire(interpreter);
                match interpreter.get(adress) {
                    MemoryNode::Data(value) => *value,
                    _ => 0,
                }
            }
            Value::OperationalDirect(operand1, operand2) => {
                let value1 = operand1.aquire(interpreter);
                let value2 = operand2.aquire(interpreter);
                value1 + value2
            }
            Value::OperationalIndirect(operand1, operand2) => {
                let adress = operand1.aquire(interpreter);
                let value2 = operand2.aquire(interpreter);
                match interpreter.get(adress) {
                    MemoryNode::Data(value) => *value + value2,
                    _ => 0,
                }
            }
        }
    }
}

impl Executable for SysCall {
    fn execute(&self, interpreter: &mut Interpreter) -> bool {
        match self {
            SysCall::Print(value) => {
                let value = value.aquire(interpreter);
                println!("{}", value);
            }
            SysCall::PrintChar(value) => {
                let value = value.aquire(interpreter);
                print!("{}", value as u8 as char);
            }
            SysCall::PrintString { start, length } => {
                let start = start.aquire(interpreter);
                let length = length.aquire(interpreter);
                for i in 0..length {
                    let adress = start + i;
                    if let MemoryNode::Data(value) = interpreter.get(adress) {
                        print!("{}", *value as u8 as char)
                    }
                }
            }
        }
        true
    }
}

fn transmute<S, D>(source: S) -> D {
    unsafe {
        let ptr = &source as *const S as *const D;
        std::mem::forget(source);
        ptr.read()
    }
}

fn with<S, T, R>(source: (S, S), f: impl FnOnce(T, T) -> T) -> R {
    let (s1, s2) = source;
    transmute(f(transmute(s1), transmute(s2)))
}

impl Executable for Instruction {
    fn execute(&self, interpreter: &mut Interpreter) -> bool {
        match self {
            Instruction::Swap(reg1, reg2) => {
                let value1 = *interpreter.get_register(reg1);
                let value2 = *interpreter.get_register(reg2);
                interpreter.set_register(*reg1, value2);
                interpreter.set_register(*reg2, value1);
            }
            Instruction::SysCall(print) => {
                print.execute(interpreter);
            }
            Instruction::Add(register, value1, value2, math_type) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(
                    *register,
                    match math_type {
                        MathType::Unsigned => value.wrapping_add(value2),
                        MathType::Signed => {
                            with::<u32, i32, u32>((value, value2), |v1, v2| v1.wrapping_add(v2))
                        }
                        MathType::Float => with::<u32, f32, u32>((value, value2), |v1, v2| v1 + v2),
                    },
                );
            }
            Instruction::Sub(register, value1, value2, math_type) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(
                    *register,
                    match math_type {
                        MathType::Unsigned => value - value2,
                        MathType::Signed => {
                            with::<u32, i32, u32>((value, value2), |v1, v2| v1.wrapping_sub(v2))
                        }
                        MathType::Float => with::<u32, f32, u32>((value, value2), |v1, v2| v1 - v2),
                    },
                );
            }
            Instruction::Mul(register, value1, value2, math_type) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(
                    *register,
                    match math_type {
                        MathType::Unsigned => value * value2,
                        MathType::Signed => {
                            with::<u32, i32, u32>((value, value2), |v1, v2| v1.wrapping_mul(v2))
                        }
                        MathType::Float => with::<u32, f32, u32>((value, value2), |v1, v2| v1 * v2),
                    },
                );
            }
            Instruction::Div(register, value1, value2, math_type) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(
                    *register,
                    match math_type {
                        MathType::Unsigned => value / value2,
                        MathType::Signed => {
                            with::<u32, i32, u32>((value, value2), |v1, v2| v1.wrapping_div(v2))
                        }
                        MathType::Float => with::<u32, f32, u32>((value, value2), |v1, v2| v1 / v2),
                    },
                );
            }
            Instruction::Mod(register, value1, value2, math_type) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(
                    *register,
                    match math_type {
                        MathType::Unsigned => value % value2,
                        MathType::Signed => {
                            with::<u32, i32, u32>((value, value2), |v1, v2| v1.wrapping_rem(v2))
                        }
                        MathType::Float => with::<u32, f32, u32>((value, value2), |v1, v2| v1 % v2),
                    },
                );
            }
            Instruction::And(register, value1, value2) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(*register, value & value2);
            }
            Instruction::Or(register, value1, value2) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(*register, value | value2);
            }
            Instruction::Xor(register, value1, value2) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(*register, value ^ value2);
            }
            Instruction::Not(register, value) => {
                let value = value.aquire(interpreter);
                interpreter.set_register(*register, !value);
            }
            Instruction::ShiftLeft(register, value1, value2) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(*register, value << value2);
            }
            Instruction::ShiftRight(register, value1, value2) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(*register, value >> value2);
            }
            Instruction::Compare(value1, value2) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                #[allow(clippy::comparison_chain)]
                let flag = if value > value2 {
                    Flag::Greater
                } else if value < value2 {
                    Flag::Lesser
                } else {
                    Flag::Equal
                };
                interpreter.set_flag(flag);
            }
            Instruction::Jump(flag, value) => {
                if interpreter.flags.contains(flag) {
                    let value = value.aquire(interpreter);
                    interpreter.set_register(Register::ProgramCounter, value);
                }
            }
            Instruction::Load(register, value) => {
                let value = value.aquire(interpreter);
                interpreter.set_register(*register, value);
            }
            Instruction::Move(value1, value2) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.memory.insert(value, MemoryNode::Data(value2));
            }
            Instruction::JumpNot(flag, value) => {
                if !interpreter.flags.contains(flag) {
                    let value = value.aquire(interpreter);
                    interpreter.set_register(Register::ProgramCounter, value);
                }
            }
            Instruction::Halt => {
                return false;
            }
        }
        true
    }
}
