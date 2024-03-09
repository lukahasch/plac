use std::collections::{BTreeMap, BTreeSet};

use super::asm::*;

#[derive(Debug,Clone,PartialEq)]
pub enum MemoryNode {
    None,
    Data(u16),
    Instruction(Instruction),
    FilledByInstruction,
}

trait Executable {
    fn execute(&self, interpreter: &mut Interpreter) -> bool;
}

#[derive(Debug,Clone,PartialEq, Default)]
pub struct Interpreter {
    pub memory: BTreeMap<u16,MemoryNode>,
    pub registers: BTreeMap<Register, u16>,
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

    pub fn load(&mut self, program: Vec<Instruction>) {
        let mut adress = 0;
        for instruction in program {
            self.memory.insert(adress, MemoryNode::Instruction(instruction));
            self.memory.insert(adress + 1, MemoryNode::FilledByInstruction);
            self.memory.insert(adress + 2, MemoryNode::FilledByInstruction);
            self.memory.insert(adress + 3, MemoryNode::FilledByInstruction);
            adress += 4;
        }
    }

    pub fn get(&mut self, adress: u16) -> &mut MemoryNode {
        if self.memory.contains_key(&adress) {
            self.memory.get_mut(&adress).unwrap()
        } else {
            self.memory.insert(adress, MemoryNode::None);
            self.memory.get_mut(&adress).unwrap()
        }
    }

    pub fn get_register(&mut self, register: &Register) -> &mut u16 {
        if self.registers.contains_key(register) { 
            self.registers.get_mut(register).unwrap()
        } else {
            self.registers.insert(*register, 0);
            self.registers.get_mut(register).unwrap()
        }
    }

    pub fn set_register(&mut self, register: Register, value: u16) {
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
        let instruction = match self.get(pc) {
            MemoryNode::Instruction(instruction) => instruction,
            _ => return Err("Invalid instruction".to_string()),
        }.clone();
        self.registers.insert(Register::ProgramCounter, pc + 4);
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
                output.push_str(&format!("{}: {:?}\n", adress, String::from(instruction.clone())));
            } else if let MemoryNode::Data(value) = node {
                output.push_str(&format!("{}: {:?}\n", adress, value));
            } 
        }
        output
    }
}

impl Operand {
    pub fn aquire(&self, interpreter: &mut Interpreter) -> u16 {
        match self {
            Operand::U16(value) => *value,
            Operand::Register(register) => *interpreter.get_register(register),
        }
    }
}

impl Value {
    pub fn aquire(&self, interpreter: &mut Interpreter) -> u16 {
        match self {
            Value::Direct(operand) => operand.aquire(interpreter),
            Value::Indirect(operand) => {
                let adress = operand.aquire(interpreter);
                match interpreter.get(adress) {
                    MemoryNode::Data(value) => *value,
                    _ => 0,
                }
            },
            Value::OperationalDirect(operand1, operand2) => {
                let value1 = operand1.aquire(interpreter);
                let value2 = operand2.aquire(interpreter);
                value1 + value2
            },
            Value::OperationalIndirect(operand1, operand2) => {
                let adress = operand1.aquire(interpreter);
                let value2 = operand2.aquire(interpreter);
                match interpreter.get(adress) {
                    MemoryNode::Data(value) => *value + value2,
                    _ => 0,
                }
            },
        }
    }
}

impl Executable for Print {
    fn execute(&self, interpreter: &mut Interpreter) -> bool {
        match self {
            Print::Print(value) => {
                let value = value.aquire(interpreter);
                println!("{}", value);
            },
            Print::PrintChar(value) => {
                let value = value.aquire(interpreter);
                print!("{}", value as u8 as char);
            },
            Print::PrintString { start, length } => {
                let start = start.aquire(interpreter);
                let length = length.aquire(interpreter);
                for i in 0..length {
                    let adress = start + i;
                    match interpreter.get(adress) {
                        MemoryNode::Data(value) => print!("{}", *value as u8 as char),
                        _ => (),
                    }
                }
            },
        }
        true
    }
}

impl Executable for Instruction {
    fn execute(&self, interpreter: &mut Interpreter) -> bool {
        match self {
            Instruction::Swap(reg1, reg2) => {
                let value1 = interpreter.get_register(reg1).clone();
                let value2 = interpreter.get_register(reg2).clone();
                interpreter.set_register(*reg1, value2);
                interpreter.set_register(*reg2, value1);
            },
            Instruction::Print(print) => {
                print.execute(interpreter);
            },
            Instruction::Add(register, value1, value2, math_type) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(*register, match math_type {
                    MathType::Unsigned => value.wrapping_add(value2),
                    MathType::Signed => {
                        let value = value as i16;
                        let value2 = value2 as i16;
                        value.wrapping_add(value2) as u16
                    },
                });
            },
            Instruction::Sub(register, value1, value2, math_type) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(*register, match math_type {
                    MathType::Unsigned => value - value2,
                    MathType::Signed => {
                        let value = value as i16;
                        let value2 = value2 as i16;
                        value.wrapping_sub(value2) as u16
                    },
                });
            },
            Instruction::Mul(register, value1, value2, math_type) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(*register, match math_type {
                    MathType::Unsigned => value * value2,
                    MathType::Signed => {
                        let value = value as i16;
                        let value2 = value2 as i16;
                        value.wrapping_mul(value2) as u16
                    },
                });
            },
            Instruction::Div(register, value1, value2, math_type) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(*register, match math_type {
                    MathType::Unsigned => value / value2,
                    MathType::Signed => {
                        let value = value as i16;
                        let value2 = value2 as i16;
                        value.wrapping_div(value2) as u16
                    },
                });
            },
            Instruction::Mod(register, value1, value2, math_type) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(*register, match math_type {
                    MathType::Unsigned => value % value2,
                    MathType::Signed => {
                        let value = value as i16;
                        let value2 = value2 as i16;
                        value.wrapping_rem(value2) as u16
                    },
                });
            },
            Instruction::And(register, value1, value2) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(*register, value & value2);
            },
            Instruction::Or(register, value1, value2) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(*register, value | value2);
            },
            Instruction::Xor(register, value1, value2) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(*register, value ^ value2);
            },
            Instruction::Not(register, value) => {
                let value = value.aquire(interpreter);
                interpreter.set_register(*register, !value);
            },
            Instruction::ShiftLeft(register, value1, value2) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(*register, value << value2);
            },
            Instruction::ShiftRight(register, value1, value2) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.set_register(*register, value >> value2);
            },
            Instruction::Compare(value1, value2) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                let flag = if value > value2 {
                    Flag::Greater
                } else if value < value2 {
                    Flag::Lesser
                } else {
                    Flag::Equal
                };
                interpreter.set_flag(flag);
            },
            Instruction::Jump(flag, value) => {
                if interpreter.flags.contains(flag) {
                    let value = value.aquire(interpreter);
                    interpreter.set_register(Register::ProgramCounter, value);
                }
            },
            Instruction::Load(register, value) => {
                let value = value.aquire(interpreter);
                interpreter.set_register(*register, value);
            },
            Instruction::Store(value1, value2) => {
                let value = value1.aquire(interpreter);
                let value2 = value2.aquire(interpreter);
                interpreter.memory.insert(value, MemoryNode::Data(value2));
            },
            Instruction::Halt => {
                return false;
            },
        }
        true
    }
}
