#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Flag {
    Overflow,
    Lesser,
    Greater,
    Equal,
    Always,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    ProgramCounter,
    StackPointer,
    ProgramPointer,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Value {
    Direct(Operand),
    Indirect(Operand),
    OperationalDirect(Operand, Operand),
    OperationalIndirect(Operand, Operand),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Operand {
    U32(u32),
    Register(Register),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum MathType {
    Unsigned,
    Signed,
    Float,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Instruction {
    Add(Register, Value, Value, MathType),
    Sub(Register, Value, Value, MathType),
    Mul(Register, Value, Value, MathType),
    Div(Register, Value, Value, MathType),
    Mod(Register, Value, Value, MathType),
    And(Register, Value, Value),
    Or(Register, Value, Value),
    Xor(Register, Value, Value),
    Not(Register, Value),
    ShiftLeft(Register, Value, Value),
    ShiftRight(Register, Value, Value),
    Compare(Value, Value),
    Jump(Flag, Value),
    JumpNot(Flag, Value),
    Load(Register, Value),
    Move(Value, Value),
    Swap(Register, Register),
    Halt,
    SysCall(SysCall),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum SysCall {
    Print(Value),
    PrintChar(Value),
    PrintString { start: Value, length: Value },
}

impl From<Flag> for String {
    fn from(flag: Flag) -> String {
        match flag {
            Flag::Overflow => "O".to_string(),
            Flag::Lesser => "L".to_string(),
            Flag::Greater => "G".to_string(),
            Flag::Equal => "E".to_string(),
            Flag::Always => "A".to_string(),
        }
    }
}

impl TryFrom<String> for Flag {
    type Error = ();
    fn try_from(flag: String) -> Result<Flag, ()> {
        Ok(match flag.as_str() {
            "O" => Flag::Overflow,
            "L" => Flag::Lesser,
            "G" => Flag::Greater,
            "E" => Flag::Equal,
            "A" => Flag::Always,
            _ => return Err(()),
        })
    }
}

impl From<Register> for String {
    fn from(register: Register) -> String {
        match register {
            Register::R0 => "r0".to_string(),
            Register::R1 => "r1".to_string(),
            Register::R2 => "r2".to_string(),
            Register::R3 => "r3".to_string(),
            Register::R4 => "r4".to_string(),
            Register::R5 => "r5".to_string(),
            Register::R6 => "r6".to_string(),
            Register::R7 => "r7".to_string(),
            Register::R8 => "r8".to_string(),
            Register::R9 => "r9".to_string(),
            Register::R10 => "r10".to_string(),
            Register::R11 => "r11".to_string(),
            Register::R12 => "r12".to_string(),
            Register::R13 => "r13".to_string(),
            Register::R14 => "r14".to_string(),
            Register::R15 => "r15".to_string(),
            Register::ProgramCounter => "PC".to_string(),
            Register::StackPointer => "SP".to_string(),
            Register::ProgramPointer => "PP".to_string(),
        }
    }
}

impl TryFrom<String> for Register {
    type Error = ();
    fn try_from(register: String) -> Result<Register, ()> {
        Ok(match register.as_str() {
            "r0" => Register::R0,
            "r1" => Register::R1,
            "r2" => Register::R2,
            "r3" => Register::R3,
            "r4" => Register::R4,
            "r6" => Register::R6,
            "r7" => Register::R7,
            "r8" => Register::R8,
            "r9" => Register::R9,
            "r10" => Register::R10,
            "r11" => Register::R11,
            "r12" => Register::R12,
            "r13" => Register::R13,
            "r14" => Register::R14,
            "r15" => Register::R15,
            "PC" => Register::ProgramCounter,
            "SP" => Register::StackPointer,
            "PP" => Register::ProgramPointer,
            _ => return Err(()),
        })
    }
}

impl From<Operand> for String {
    fn from(operand: Operand) -> String {
        match operand {
            Operand::U32(value) => value.to_string(),
            Operand::Register(register) => register.into(),
        }
    }
}

impl TryFrom<String> for Operand {
    type Error = ();
    fn try_from(operand: String) -> Result<Operand, ()> {
        match operand.parse::<u32>() {
            Ok(value) => Ok(Operand::U32(value)),
            Err(_) => Ok(Operand::Register(Register::try_from(operand)?)),
        }
    }
}

impl From<Value> for String {
    fn from(value: Value) -> String {
        match value {
            Value::Direct(operand) => operand.into(),
            Value::Indirect(operand) => format!("[{}]", String::from(operand)),
            Value::OperationalDirect(operand1, operand2) => {
                format!("{}+{}", String::from(operand1), String::from(operand2))
            }
            Value::OperationalIndirect(operand1, operand2) => {
                format!("[{}+{}]", String::from(operand1), String::from(operand2))
            }
        }
    }
}

impl TryFrom<String> for Value {
    type Error = ();
    fn try_from(mut value: String) -> Result<Value, ()> {
        if value.starts_with('[') {
            if value.contains('+') {
                value.pop();
                value.remove(0);
                let (first, second) = value.split_once('+').unwrap();
                Ok(Value::OperationalIndirect(
                    Operand::try_from(first.to_string())?,
                    Operand::try_from(second.to_string())?,
                ))
            } else {
                Ok(Value::Indirect(Operand::try_from(
                    value[1..value.len() - 1].to_string(),
                )?))
            }
        } else if value.contains('+') {
            let (first, second) = value.split_once('+').unwrap();
            Ok(Value::OperationalDirect(
                Operand::try_from(first.to_string())?,
                Operand::try_from(second.to_string())?,
            ))
        } else {
            Ok(Value::Direct(Operand::try_from(value)?))
        }
    }
}

impl From<MathType> for String {
    fn from(math_type: MathType) -> String {
        match math_type {
            MathType::Unsigned => "U".to_string(),
            MathType::Signed => "S".to_string(),
            MathType::Float => "F".to_string(),
        }
    }
}

impl TryFrom<String> for MathType {
    type Error = ();
    fn try_from(math_type: String) -> Result<MathType, ()> {
        Ok(match math_type.as_str() {
            "U" => MathType::Unsigned,
            "S" => MathType::Signed,
            "F" => MathType::Float,
            _ => return Err(()),
        })
    }
}

impl From<SysCall> for String {
    fn from(print: SysCall) -> String {
        match print {
            SysCall::Print(value) => format!("print {}", String::from(value)),
            SysCall::PrintChar(value) => format!("printc {}", String::from(value)),
            SysCall::PrintString { start, length } => {
                format!("printstr {} {}", String::from(start), String::from(length))
            }
        }
    }
}

impl TryFrom<String> for SysCall {
    type Error = ();
    fn try_from(mut print: String) -> Result<Self, ()> {
        if print.starts_with("printstr") {
            print = print.replace("printstr", "").trim().to_string();
            let (start, length) = print.split_once(' ').unwrap();
            Ok(Self::PrintString {
                start: Value::try_from(start.to_string())?,
                length: Value::try_from(length.to_string())?,
            })
        } else if print.starts_with("printc") {
            print = print.replace("printc", "").trim().to_string();
            Ok(Self::PrintChar(Value::try_from(print)?))
        } else {
            print = print.replace("print", "").trim().to_string();
            Ok(Self::Print(Value::try_from(print)?))
        }
    }
}

macro_rules! fmt_string {
    ($first:expr, $($rest:expr),*) => {
        format!($first, $(String::from($rest)),*)
    };
}

impl From<Instruction> for String {
    fn from(instruction: Instruction) -> String {
        match instruction {
            Instruction::Add(register, value1, value2, math_type) => {
                fmt_string!("add {} {} {} {}", register, value1, value2, math_type)
            }
            Instruction::Sub(register, value1, value2, math_type) => {
                fmt_string!("sub {} {} {} {}", register, value1, value2, math_type)
            }
            Instruction::Mul(register, value1, value2, math_type) => {
                fmt_string!("mul {} {} {} {}", register, value1, value2, math_type)
            }
            Instruction::Div(register, value1, value2, math_type) => {
                fmt_string!("div {} {} {} {}", register, value1, value2, math_type)
            }
            Instruction::Mod(register, value1, value2, math_type) => {
                fmt_string!("mod {} {} {} {}", register, value1, value2, math_type)
            }
            Instruction::And(register, value1, value2) => {
                fmt_string!("and {} {} {}", register, value1, value2)
            }
            Instruction::Or(register, value1, value2) => {
                fmt_string!("or {} {} {}", register, value1, value2)
            }
            Instruction::Xor(register, value1, value2) => {
                fmt_string!("xor {} {} {}", register, value1, value2)
            }
            Instruction::Not(register, value) => fmt_string!("not {} {}", register, value),
            Instruction::ShiftLeft(register, value1, value2) => {
                fmt_string!("shl {} {} {}", register, value1, value2)
            }
            Instruction::ShiftRight(register, value1, value2) => {
                fmt_string!("shr {} {} {}", register, value1, value2)
            }
            Instruction::Compare(value1, value2) => fmt_string!("cmp {} {}", value1, value2),
            Instruction::Jump(flag, value) => fmt_string!("jmp {} {}", flag, value),
            Instruction::Load(register, value) => fmt_string!("load {} {}", register, value),
            Instruction::Move(value1, value2) => fmt_string!("mov {} {}", value1, value2),
            Instruction::Halt => "halt".to_string(),
            Instruction::SysCall(print) => fmt_string!("{}", print),
            Instruction::Swap(reg1, reg2) => fmt_string!("swap {} {}", reg1, reg2),
            Instruction::JumpNot(flag, value) => fmt_string!("jmpn {} {}", flag, value),
        }
    }
}

macro_rules! from_str {
    ($first:expr, $($rest:ty),*) => {
        (|str:&str| {
            if !str.starts_with($first) {
                return Err(());
            }
            let str = str.trim_start_matches($first);
            let str = str.trim();
            let mut iter = str.split_whitespace();
            Ok((
                $(
                    <$rest>::try_from(iter.next().ok_or(())?.to_string())?,
                )*
            ))
        })
    };
}

impl TryFrom<String> for Instruction {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Ok((register, value1, value2, value3)) =
            from_str!("add", Register, Value, Value, MathType)(value.as_str())
        {
            Ok(Instruction::Add(register, value1, value2, value3))
        } else if let Ok((register, value1, value2, value3)) =
            from_str!("sub", Register, Value, Value, MathType)(value.as_str())
        {
            Ok(Instruction::Sub(register, value1, value2, value3))
        } else if let Ok((register, value1, value2, value3)) =
            from_str!("mul", Register, Value, Value, MathType)(value.as_str())
        {
            Ok(Instruction::Mul(register, value1, value2, value3))
        } else if let Ok((register, value1, value2, value3)) =
            from_str!("div", Register, Value, Value, MathType)(value.as_str())
        {
            Ok(Instruction::Div(register, value1, value2, value3))
        } else if let Ok((register, value1, value2, value3)) =
            from_str!("mod", Register, Value, Value, MathType)(value.as_str())
        {
            Ok(Instruction::Mod(register, value1, value2, value3))
        } else if let Ok((register, value1, value2)) =
            from_str!("and", Register, Value, Value)(value.as_str())
        {
            Ok(Instruction::And(register, value1, value2))
        } else if let Ok((register, value1, value2)) =
            from_str!("or", Register, Value, Value)(value.as_str())
        {
            Ok(Instruction::Or(register, value1, value2))
        } else if let Ok((register, value1, value2)) =
            from_str!("xor", Register, Value, Value)(value.as_str())
        {
            Ok(Instruction::Xor(register, value1, value2))
        } else if let Ok((register, value)) = from_str!("not", Register, Value)(value.as_str()) {
            Ok(Instruction::Not(register, value))
        } else if let Ok((register, value1, value2)) =
            from_str!("shl", Register, Value, Value)(value.as_str())
        {
            Ok(Instruction::ShiftLeft(register, value1, value2))
        } else if let Ok((register, value1, value2)) =
            from_str!("shr", Register, Value, Value)(value.as_str())
        {
            Ok(Instruction::ShiftRight(register, value1, value2))
        } else if let Ok((value1, value2)) = from_str!("cmp", Value, Value)(value.as_str()) {
            Ok(Instruction::Compare(value1, value2))
        } else if let Ok((flag, value)) = from_str!("jmp", Flag, Value)(value.as_str()) {
            Ok(Instruction::Jump(flag, value))
        } else if let Ok((register, value)) = from_str!("load", Register, Value)(value.as_str()) {
            Ok(Instruction::Load(register, value))
        } else if let Ok((value1, value2)) = from_str!("mov", Value, Value)(value.as_str()) {
            Ok(Instruction::Move(value1, value2))
        } else if value == "halt" {
            Ok(Instruction::Halt)
        } else if let Ok((reg1, reg2)) = from_str!("swap", Register, Register)(value.as_str()) {
            Ok(Instruction::Swap(reg1, reg2))
        } else if let Ok(print) = SysCall::try_from(value.clone()) {
            Ok(Instruction::SysCall(print))
        } else if let Ok((flag, value)) = from_str!("jmpn", Flag, Value)(value.as_str()) {
            Ok(Instruction::JumpNot(flag, value))
        } else {
            Err(())
        }
    }
}
