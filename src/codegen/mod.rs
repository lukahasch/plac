use std::collections::HashMap;

use self::asm::{Flag, Instruction, Operand, Register, Value};

pub mod asm;
pub mod interpreter;

pub const INSTRUCTION_SIZE: usize = 4;

pub type Optimizer = Box<dyn FnMut(&mut Vec<u32>, &mut Vec<Ir>)>;

#[derive(Debug, Clone, PartialEq)]
pub enum Ir {
    Instruction(Instruction),
    Label(u64, Instruction),
    Jump(bool, Flag, u64),
}

#[derive(Default)]
pub struct Assembler {
    pub data: Vec<u32>,
    pub code: Vec<Ir>,
    pub optimizers: Vec<Optimizer>,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            data: Vec::new(),
            code: Vec::new(),
            optimizers: Vec::new(),
        }
    }

    pub fn add_optimizer(&mut self, optimizer: Optimizer) {
        self.optimizers.push(optimizer);
    }

    pub fn add_ir(&mut self, instruction: Ir) {
        self.code.push(instruction);
    }

    pub fn add_data(&mut self, data: u32) {
        self.data.push(data);
    }

    pub fn get_data(&self, index: u32) -> u32 {
        self.data[index as usize]
    }

    pub fn fetch_data(&self, index: u32, register: Register) -> Ir {
        Ir::Instruction(Instruction::Load(
            register,
            Value::OperationalIndirect(
                Operand::U32(index),
                Operand::Register(Register::ProgramPointer),
            ),
        ))
    }

    pub fn compile(mut self, passes: usize) -> (Vec<u32>, Vec<Instruction>, u32) {
        {
            let Assembler {
                data,
                code,
                optimizers,
            } = &mut self;
            for _ in 0..passes {
                for optimizer in optimizers.iter_mut() {
                    optimizer(data, code);
                }
            }
        }
        let starting_offset = self.data.len() as u32;
        let labels = self
            .code
            .iter()
            .enumerate()
            .filter_map(|(i, ir)| match ir {
                Ir::Label(label, _) => Some((
                    *label,
                    starting_offset + ((i as u32) * INSTRUCTION_SIZE as u32),
                )),
                _ => None,
            })
            .collect::<HashMap<u64, u32>>();
        (
            self.data,
            self.code
                .into_iter()
                .map(|ir| match ir {
                    Ir::Instruction(instruction) => instruction,
                    Ir::Label(_, instruction) => instruction,
                    Ir::Jump(bool, flag, label) => {
                        let offset = Value::OperationalDirect(
                            Operand::U32(labels[&label]),
                            Operand::Register(Register::ProgramPointer),
                        );
                        match bool {
                            true => Instruction::Jump(flag, offset),
                            false => Instruction::JumpNot(flag, offset),
                        }
                    }
                })
                .collect(),
            starting_offset,
        )
    }
}

impl From<Ir> for String {
    fn from(ir: Ir) -> String {
        match ir {
            Ir::Instruction(instruction) => instruction.into(),
            Ir::Label(label, instruction) => format!(
                "{}: {}",
                label,
                <Instruction as Into<String>>::into(instruction)
            ),
            Ir::Jump(true, flag, label) => {
                format!("jump: {} {}", <Flag as Into<String>>::into(flag), label)
            }
            Ir::Jump(false, flag, label) => {
                format!("jmpn: {} {}", <Flag as Into<String>>::into(flag), label)
            }
        }
    }
}

impl TryFrom<String> for Ir {
    type Error = String;
    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        if value.starts_with("jump: ") {
            value = value.trim_start_matches("jump: ").to_string();
            let flag = <Flag as TryFrom<String>>::try_from(
                value.chars().take_while(|c| c != &' ').collect::<String>(),
            )
            .map_err(|_| "Flag".to_string())?;
            let label = value
                .chars()
                .skip_while(|c| c != &' ')
                .skip(1)
                .collect::<String>()
                .parse::<u64>()
                .map_err(|e| e.to_string())?;
            Ok(Ir::Jump(true, flag, label))
        } else if value.starts_with("jmpn: ") {
            value = value.trim_start_matches("jmpn: ").to_string();
            let flag = <Flag as TryFrom<String>>::try_from(
                value.chars().take_while(|c| c != &' ').collect::<String>(),
            )
            .map_err(|_| "Flag".to_string())?;
            let label = value
                .chars()
                .skip_while(|c| c != &' ')
                .skip(1)
                .collect::<String>()
                .parse::<u64>()
                .map_err(|e| e.to_string())?;
            Ok(Ir::Jump(false, flag, label))
        } else if value.contains(':') {
            let label = value
                .chars()
                .take_while(|c| c != &':')
                .collect::<String>()
                .parse::<u64>()
                .map_err(|e| e.to_string())?;
            dbg!(value
                .chars()
                .skip_while(|c| c != &':')
                .skip(2)
                .collect::<String>());
            let instruction = Instruction::try_from(
                value
                    .chars()
                    .skip_while(|c| c != &':')
                    .skip(2)
                    .collect::<String>(),
            )
            .map_err(|_| "Instruction".to_string())?;
            Ok(Ir::Label(label, instruction))
        } else {
            Instruction::try_from(value)
                .map(Ir::Instruction)
                .map_err(|_| "Instruction".to_string())
        }
    }
}
