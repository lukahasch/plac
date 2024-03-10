use plac::codegen::asm::*;
use plac::codegen::interpreter::*;
use plac::codegen::*;

fn main() {
    let instructions: Vec<Ir> = vec![
        "load r0 15",
        "load r1 1",
        "load r2 2",
        "0: sub r0 r0 1 U",
        "swap r1 r2",
        "add r2 r1 r2 U",
        "print r2",
        "cmp r0 0",
        "jmpn: E 0",
        "halt",
    ]
    .into_iter()
    .map(|x| Ir::try_from(x.to_string()).unwrap_or_else(|e| panic!("{}::{}", e, x)))
    .collect();
    println!("{:#?}", instructions);
    println!("====================");
    let mut assembler = Assembler::new();
    for i in instructions {
        assembler.add_ir(i);
    }
    let (data, instructions, start) = assembler.compile(1);
    println!("{:?}", instructions);
    println!("====================");
    let mut interpreter = Interpreter::new();
    interpreter.load(data, instructions, start);
    interpreter.set_register(Register::StackPointer, 1000);
    let err = interpreter.execute();
    println!("====================");
    println!("{:?}", err);
    println!("{}", interpreter.debug());
}
